use dbus::blocking::Connection;
use std::{
    time::Duration,
    str,
};

const WIFI_DEV: &str = "wlp170s0";

const NM_DBUS: &str = "org.freedesktop.NetworkManager";
const NM_DEV_WL_DBUS: &str = "org.freedesktop.NetworkManager.Device.Wireless";
const NM_AP_DBUS: &str = "org.freedesktop.NetworkManager.AccessPoint";
const NM_PATH: &str = "/org/freedesktop/NetworkManager";
const DBUS_TIMEOUT: Duration = Duration::from_millis(5000);


fn main() -> Result<(),Box<dyn std::error::Error>> {
    let conn = Connection::new_system()?;

    // wrapper struct around connection in order to send method calls to specific
    // destination (NetworkManager)
    let proxy = conn.with_proxy(
        NM_DBUS,
        NM_PATH,
        DBUS_TIMEOUT);

    // make method call
    // GetAllDevices(OUT ao devices)
    //let (devices,): (Vec<dbus::Path<'static>>,) = proxy.method_call(NM_DBUS,"GetAllDevices",())?;

    //for device in devices {println!("{:?}",device)};

    //GetDeviceByIpIface(IN s iface, OUT o device)
    let (device_path,): (dbus::Path<'static>,) = proxy.method_call(NM_DBUS,"GetDeviceByIpIface",(WIFI_DEV,))?;

    let dev_proxy = conn.with_proxy(
        NM_DBUS,
        device_path,
        DBUS_TIMEOUT);

    // get AP path property 
    let ap_path: dbus::Path<'static> = dbus::blocking::stdintf::org_freedesktop_dbus::Properties::get(
        &dev_proxy,
        NM_DEV_WL_DBUS,
        "ActiveAccessPoint",)?;

    let ap_proxy = conn.with_proxy(
        NM_DBUS,
        ap_path,
        DBUS_TIMEOUT);

    // get AccessPoint 'Ssid' property
    let ssid_bytes: Vec<u8> = dbus::blocking::stdintf::org_freedesktop_dbus::Properties::get(
        &ap_proxy,
        NM_AP_DBUS,
        "Ssid",
    )?;
    
    let ssid = str::from_utf8(&ssid_bytes).unwrap();
    println!("{ssid}");

    Ok(())
}
