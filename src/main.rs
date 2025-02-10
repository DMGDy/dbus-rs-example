use dbus::blocking::Connection;
use std::time::Duration;

const WIFI_DEV: &str = "wlp170s0";

const NM_DBUS: &str = "org.freedesktop.NetworkManager";
const NM_PATH: &str = "/org/freedesktop/NetworkManager";


fn main() -> Result<(),Box<dyn std::error::Error>> {
    let conn = Connection::new_system()?;

    // wrapper struct around connection in order to send method calls to specific
    // destination (NetworkManager)
    let proxy = conn.with_proxy(
        NM_DBUS,
        NM_PATH,
        Duration::from_millis(5000));

    // make method call
    // GetAllDevices(OUT ao devices)
    let (devices,): (Vec<dbus::Path<'static>>,) = proxy.method_call(NM_DBUS,"GetAllDevices",())?;

    for device in devices {println!("{:?}",device)};

    Ok(())
}
