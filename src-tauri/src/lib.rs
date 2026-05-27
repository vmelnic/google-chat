use std::path::{Path, PathBuf};

use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    utils::config::Color,
    webview::{DownloadEvent, NewWindowResponse, PageLoadEvent},
    Manager, RunEvent, Theme, WebviewUrl, WebviewWindowBuilder, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_opener::OpenerExt;

fn downloads_dir() -> PathBuf {
    dirs::download_dir().unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
}

fn filename_from_url(url: &str) -> String {
    url.rsplit('/')
        .next()
        .and_then(|s| {
            let name = s.split('?').next().unwrap_or(s);
            if name.is_empty() || !name.contains('.') {
                None
            } else {
                Some(urlencoding::decode(name).unwrap_or_else(|_| name.into()).into_owned())
            }
        })
        .unwrap_or_else(|| "download".to_string())
}

fn unique_path(dir: &Path, filename: &str) -> PathBuf {
    let path = dir.join(filename);
    if !path.exists() {
        return path;
    }
    let stem = std::path::Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("download");
    let ext = std::path::Path::new(filename)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    for i in 1..1000 {
        let candidate = if ext.is_empty() {
            dir.join(format!("{stem} ({i})"))
        } else {
            dir.join(format!("{stem} ({i}).{ext}"))
        };
        if !candidate.exists() {
            return candidate;
        }
    }
    path
}

#[tauri::command]
fn save_downloaded_file(filename: String, data: Vec<u8>) -> Result<String, String> {
    let dir = downloads_dir();
    let path = unique_path(&dir, &filename);
    std::fs::write(&path, &data).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().into_owned())
}

const DOWNLOAD_INTERCEPT_JS: &str = r#"
(function() {
  if (window.__tauriDownloadInterceptInstalled) return;
  window.__tauriDownloadInterceptInstalled = true;

  const origCreateObjectURL = URL.createObjectURL;
  const blobMap = new Map();

  URL.createObjectURL = function(obj) {
    const url = origCreateObjectURL.call(this, obj);
    if (obj instanceof Blob) {
      blobMap.set(url, obj);
    }
    return url;
  };

  const origRevokeObjectURL = URL.revokeObjectURL;
  URL.revokeObjectURL = function(url) {
    blobMap.delete(url);
    return origRevokeObjectURL.call(this, url);
  };

  async function handleBlobDownload(blobUrl, filename) {
    try {
      let blob = blobMap.get(blobUrl);
      if (!blob) {
        const resp = await fetch(blobUrl);
        blob = await resp.blob();
      }
      const buf = await blob.arrayBuffer();
      const bytes = Array.from(new Uint8Array(buf));
      const result = await window.__TAURI__.core.invoke('save_downloaded_file', {
        filename: filename || 'download',
        data: bytes
      });
      console.log('[Tauri] File saved to:', result);
    } catch (e) {
      console.error('[Tauri] Download failed:', e);
    }
  }

  document.addEventListener('click', function(e) {
    const anchor = e.target.closest('a[href]');
    if (!anchor) return;
    const href = anchor.href;
    if (href && href.startsWith('blob:')) {
      e.preventDefault();
      e.stopPropagation();
      const filename = anchor.download || anchor.getAttribute('download') || 'download';
      handleBlobDownload(href, filename);
    }
  }, true);

  const origAnchorClick = HTMLAnchorElement.prototype.click;
  HTMLAnchorElement.prototype.click = function() {
    if (this.href && this.href.startsWith('blob:')) {
      const filename = this.download || this.getAttribute('download') || 'download';
      handleBlobDownload(this.href, filename);
      return;
    }
    return origAnchorClick.call(this);
  };
})();
"#;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![save_downloaded_file])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("Google Chat")
                .inner_size(1200.0, 800.0)
                .min_inner_size(600.0, 400.0)
                .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
                .theme(Some(Theme::Dark))
                .background_color(Color(26, 26, 46, 255))
                .disable_drag_drop_handler()
                .on_navigation(|_url| true)
                .on_new_window(move |url, _features| {
                    let _ = app_handle.opener().open_url(url.as_str(), None::<&str>);
                    NewWindowResponse::Deny
                })
                .on_download(|_webview, event| {
                    match event {
                        DownloadEvent::Requested { url, destination } => {
                            let filename = filename_from_url(url.as_str());
                            *destination = unique_path(&downloads_dir(), &filename);
                        }
                        DownloadEvent::Finished { .. } => {}
                        _ => {}
                    }
                    true
                })
                .on_page_load(|webview, payload| {
                    if matches!(payload.event(), PageLoadEvent::Finished) {
                        let _ = webview.eval(DOWNLOAD_INTERCEPT_JS);
                    }
                })
                .build()?;

            window.navigate("https://chat.google.com".parse().unwrap())?;

            let hide = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit Google Chat", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&hide, &quit])?;

            let icon = Image::from_bytes(include_bytes!("../icons/32x32.png"))?;

            let _tray = TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .tooltip("Google Chat")
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "hide" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.hide();
                        }
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click { .. } = event {
                        if let Some(w) = tray.app_handle().get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                })
                .build(app)?;

            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window_clone.hide();
                }
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| match &event {
            #[cfg(target_os = "macos")]
            RunEvent::Reopen { .. } => {
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.show();
                    let _ = w.set_focus();
                }
            }
            RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
