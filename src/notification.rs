#[cfg(target_os = "macos")]
use mac_notification_sys;

pub fn send(title: &str, body: &str) {
    #[cfg(target_os = "macos")]
    mac_notification_sys::send_notification(title, &None, body, &Some("Ping")).unwrap();
}
