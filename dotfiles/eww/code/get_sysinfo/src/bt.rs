use super::*;

#[derive(Debug, Serialize)]
pub struct Bt {
    pub dev: String,
    pub con: bool,
    pub rfkill: bool,
}

impl Bt {
    pub fn new() -> Bt {
        Bt {
            dev: String::new(),
            con: false,
            rfkill: false,
        }
    }
    pub fn update(&mut self, rfkill: bool) {
        self.rfkill = rfkill;
        if rfkill {
            let output = match Command::new("bluetoothctl")
                .args(["devices", "Connected"])
                .output()
            {
                Ok(output) => output.stdout,
                Err(_) => return,
            };
            let output = String::from_utf8_lossy(&output);
            let lines = output.lines();
            let mut out = String::new();
            for line in lines {
                if line.starts_with("Device ") {
                    let name = &line[25..];
                    if out.is_empty() {
                        out = name.to_string()
                    } else {
                        out = format!("{}, {}", out, name)
                    }
                }
            }
            self.con = !out.is_empty();
            self.dev = out;
        }
    }
}
