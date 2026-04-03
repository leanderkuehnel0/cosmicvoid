use super::*;

use networkmanager::{
    devices::{Any, Device, Wired, Wireless},
    types::ConnectivityState,
};

#[derive(Debug, Serialize)]
pub struct Wlan {
    name: Option<String>,
    ip: Option<String>,
    rssi: isize,
    con: bool,
    rfkill: bool,
    iface: String,
    internet: bool,
}

#[derive(Debug, Serialize)]
pub struct Ethernet {
    ip: Option<String>,
    up: bool,
    internet: bool,
    iface: String,
}

impl Wlan {
    pub fn new() -> Wlan {
        Wlan {
            name: None,
            ip: None,
            rssi: 0,
            con: false,
            rfkill: false,
            internet: false,
            iface: String::new(),
        }
    }
    pub fn update(&mut self, device: networkmanager::devices::WiFiDevice, rfkill: bool) {
        self.rfkill = rfkill;
        if rfkill {
            self.iface = device.interface().unwrap_or_default();
            self.ip = if let Ok(x) = device.ip4_config() {
                get_ip(x)
            } else {
                None
            };
            (self.name, self.rssi) = if let Ok(x) = device.active_access_point() {
                (x.ssid().ok(), x.strength().ok().unwrap_or(0) as isize)
            } else {
                (None, 0)
            };
            self.con = self.rssi != 0;
            self.internet = matches!(Any::ip4_connectivity(&device), Ok(ConnectivityState::Full));
        } else {
            self.empty();
        }
    }
    fn empty(&mut self) {
        self.name = None;
        self.ip = None;
        self.rssi = 0;
        self.con = false;
        self.internet = false;
    }
}

impl Ethernet {
    pub fn new() -> Ethernet {
        Ethernet {
            ip: None,
            up: false,
            internet: false,
            iface: String::new(),
        }
    }
    pub fn update(&mut self, device: networkmanager::devices::EthernetDevice) {
        self.iface = device.interface().unwrap_or_default();
        self.ip = if let Ok(x) = device.ip4_config() {
            get_ip(x)
        } else {
            None
        };
        self.internet = matches!(Any::ip4_connectivity(&device), Ok(ConnectivityState::Full));
        self.up = device.carrier().unwrap_or_default();
    }
    fn empty(&mut self) {
        self.ip = None;
        self.up = false;
        self.internet = false;
    }
}

fn get_ip(x: networkmanager::configs::Ip4Config) -> Option<String> {
    if let Ok(x) = x.addresses() {
        if let Some(x) = x.first() {
            if let Some(x) = x.first() {
                Some(u32_to_ip(x))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn u32_to_ip(x: &u32) -> String {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    format!("{}.{}.{}.{}", b4, b3, b2, b1)
}
