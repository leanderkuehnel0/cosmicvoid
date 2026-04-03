use super::*;

#[derive(Debug, Serialize)]
pub struct Gpu {
    pub util: f32,
    pub vram: Mem,
    pub gtt: Mem,
    #[serde(skip_serializing)]
    paths: [String; 5],

    hwmon: Temp,
    #[serde(skip_serializing)]
    temp_paths: Temps,
}

impl Gpu {
    pub fn new() -> Gpu {
        let path = GPU_PATH.to_owned();
        let paths = [
            path.clone() + "mem_info_vram_used",
            path.clone() + "mem_info_vram_total",
            path.clone() + "mem_info_gtt_used",
            path.clone() + "mem_info_gtt_total",
            path.clone() + "gpu_busy_percent",
        ];

        let temp_paths = Temps::None(path.clone());
        Gpu {
            util: 0.0,
            gtt: Mem::new(),
            vram: Mem::new(),
            temp_paths,
            hwmon: Temp {
                power_draw: 0.0,
                edge: 0.0,
                junction: 0.0,
            },
            paths,
        }
    }
    pub fn update(&mut self) {
        self.util = parse_file::<f32>(&self.paths[4]).unwrap() / 100.0;
        self.vram.used = parse_file::<f32>(&self.paths[0]).unwrap() / (1024.0 * 1024.0 * 1024.0);
        self.vram.total = parse_file::<f32>(&self.paths[1]).unwrap() / (1024.0 * 1024.0 * 1024.0);
        self.gtt.used = parse_file::<f32>(&self.paths[2]).unwrap() / (1024.0 * 1024.0 * 1024.0);
        self.gtt.total = parse_file::<f32>(&self.paths[3]).unwrap() / (1024.0 * 1024.0 * 1024.0);
        (self.hwmon.power_draw, self.hwmon.edge, self.hwmon.junction) = self.temp_paths.update();
    }
}

#[derive(Debug, Serialize)]
pub struct Temp {
    pub power_draw: f32,
    pub edge: f32,
    pub junction: f32,
}

use std::string::String;
#[derive(Debug, Serialize)]
pub enum Temps {
    Some {
        power_path: String,
        edge_path: String,
        junction_path: Option<String>,
    },
    None(String),
}

impl Temps {
    fn update(&mut self) -> (f32, f32, f32) {
        use Temps::*;
        if let None(path) = self {
            let temp = get_temp_path(path, "amdgpu");
            if let Ok(temp) = temp {
                *self = Temps::Some {
                    power_path: temp.get("PPT").cloned().unwrap(),
                    edge_path: temp.get("edge").cloned().unwrap(),
                    junction_path: temp.get("junction").cloned(),
                }
            } else {
                return (0.0, 0.0, 0.0);
            }
        }
        if let Some {
            edge_path,
            junction_path,
            power_path,
        } = self
        {
            let power_draw =
                parse_file::<f32>(power_path).expect("coudn't read power draw") / 1000000.0;
            let edge = parse_file::<f32>(edge_path).expect("coudn't read edge temp") / 1000.0;
            let junction = if let Some(path) = junction_path {
                parse_file::<f32>(path).expect("coudn't read junction temp") / 1000.0
            } else {
                0.0
            };
            (power_draw, edge, junction)
        } else {
            (0.0, 0.0, 0.0)
        }
    }
}
