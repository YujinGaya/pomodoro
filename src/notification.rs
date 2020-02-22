use mac_notification_sys;

pub fn send(title: &str, body: &str) {
    mac_notification_sys::send_notification(title, &None, body, &Some("Ping")).unwrap();
}
