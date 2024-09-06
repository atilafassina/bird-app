use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    const QUIET_TWITTER: &str = r#"
    let styles = `
[data-testid="sidebarColumn"],
[data-testid="SideNav_AccountSwitcher_Button"], 
[data-testid="AppTabBar_Explore_Link"] {
  display: none;
}


[data-testid="primaryColumn"] {
  border: none;
}

  h1 > a[href="/home"] {
    display: none;
}
`
  let style = document.createElement('style');
  style.innerText = styles;
  document.body.appendChild(style);

"#;

    tauri::Builder::default()
        .setup(|app| {
            let t = app
                .get_webview_window("main")
                .expect("Failed to grab WebView");

            t.eval(QUIET_TWITTER).expect("Failed to inject Bookmarklet");
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
