extern crate serde;
use dbus::blocking::Connection;
use derivative::Derivative;
use glob::glob;
use networkmanager::devices::{Any, Bluetooth, Device, Wired, Wireless};
use networkmanager::NetworkManager;
use serde::Serialize;

use rfkillr::rfkill::RfKill;
use rfkillr::CRfKillEvent;
use rfkillr::RfkillOperations;
use rfkillr::RfkillType;

use std::{
    collections::HashMap, error::Error, fmt, fs, path::PathBuf, process::Command, sync::mpsc,
    thread, time::Duration,
};

mod cpu;
use cpu::*;

mod gpu;
use gpu::*;

mod mem;
use mem::*;

// mod wlan;
// use wlan::*;

mod bt;
use bt::*;

mod net;
use net::*;

//mod proc;
//use proc::*;

mod consts;
use consts::*;

#[derive(Serialize, Derivative)]
#[derivative(Debug)]
pub struct Info<'a> {
    pub cpu: Cpu,
    pub gpu: Gpu,
    pub mem: Mem,
    pub swap: Mem,
    pub wlan: Wlan,
    pub ether: Ethernet,
    pub bt: Bt,
    // #[serde(skip_serializing)]
    // #[derivative(Debug="ignore")]
    // dbus: Connection,
    #[serde(skip_serializing)]
    #[derivative(Debug = "ignore")]
    nm: NetworkManager<'a>,
    #[serde(skip_serializing)]
    #[derivative(Debug = "ignore")]
    rfkill: (RfKill, CRfKillEvent),
    #[serde(skip_serializing)]
    #[derivative(Debug = "ignore")]
    states: (bool, bool),
}

impl<'a> Info<'_> {
    pub fn new(dbus: &'a Connection) -> Info<'a> {
        let nm = NetworkManager::new(dbus);
        let rfkill = (
            RfKill::new().unwrap(),
            CRfKillEvent::default().set_event_type(RfkillType::All),
        );
        Info {
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            mem: Mem::new(),
            swap: Mem::new(),
            wlan: Wlan::new(),
            ether: Ethernet::new(),
            states: (false, false),
            bt: Bt::new(),
            nm,
            rfkill,
        }
    }
    pub fn update(&mut self) {
        while let Ok(x) = self.rfkill.0.read_event(self.rfkill.1) {
            let block = !(x.is_soft || x.is_hard);
            match x.device {
                RfkillType::Wlan => self.states.0 = block,
                RfkillType::Bluetooth => self.states.1 = block,
                RfkillType::All => self.states = (block, block),
                _ => (),
            }
        }
        for dev in self.nm.get_devices().unwrap() {
            match dev {
                Device::WiFi(x) => {
                    self.wlan.update(x, self.states.0);
                }
                Device::Ethernet(x) => self.ether.update(x),
                _ => (),
            }
        }
        self.cpu.update();
        self.gpu.update();

        self.bt.update(self.states.1);
        self.mem.update();
        if SWAP {
            self.swap.update_swap();
        }
        println!("{}", serde_json::to_string(self).unwrap());
    }
}

pub fn parse_file<T: std::str::FromStr>(path: &String) -> Result<T, Box<dyn Error>>
where
    <T as std::str::FromStr>::Err: std::error::Error,
    <T as std::str::FromStr>::Err: 'static,
{
    Ok(fs::read_to_string(path)?.trim().parse::<T>()?)
}

fn get_temp_path(path: &String, name: &str) -> Result<HashMap<String, String>, ()> {
    let mut out = HashMap::new();
    let mut get_list = |tail: &str| -> Result<(), ()> {
        match glob(&(format!("{}hwmon/hwmon*/*_{}", path, tail))) {
            Ok(glob) => {
                for entry in glob {
                    if let Ok(path) = entry {
                        if fs::read_to_string(path.parent().unwrap().join("name"))
                            .unwrap()
                            .trim()
                            != name
                        {
                            continue;
                        }
                        let mut path2 = path.display().to_string();
                        for _ in 0..tail.len() {
                            path2.pop();
                        }
                        let temp_name = parse_file(&format!("{}label", &path2)).unwrap_or({
                            let mut str = path2.split('/').last().unwrap().to_string();
                            str.pop();
                            str
                        });
                        let path_name = path.display().to_string();
                        out.insert(temp_name, path_name);
                    };
                }
                Ok(())
            }
            Err(_) => Err(()),
        }
    };
    get_list("input")?;
    get_list("average")?;
    Ok(out)
}

#[derive(Debug, Serialize)]
pub struct Temp {
    #[serde(skip_serializing)]
    path: String,
    temp: f32,
}

fn parse_line(line: &str) -> f32 {
    line.split_whitespace()
        .nth(1)
        .unwrap()
        .trim()
        .parse::<f32>()
        .unwrap()
        / (1024.0 * 1024.0)
}
