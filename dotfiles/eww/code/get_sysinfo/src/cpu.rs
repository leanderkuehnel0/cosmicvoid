use super::*;
use std::{fs, time::SystemTime};

#[derive(Debug, Serialize, Copy, Clone)]
struct Tempor {
    total: usize,
    idle: usize,
}

#[derive(Debug, Serialize)]
pub struct Cpu {
    pub threads: [f32; CPU_THREADS],
    #[serde(skip_serializing)]
    threads_info: [Tempor; CPU_THREADS],
    pub total: f32,
    #[serde(skip_serializing)]
    total_info: Tempor,

    pub temp: f32,
    #[serde(skip_serializing)]
    temp_path: String,

    power_draw: f32,
    #[serde(skip_serializing)]
    power_path: String,
    #[serde(skip_serializing)]
    power_timestamp: SystemTime,
    #[serde(skip_serializing)]
    power_temp_uj: u64,
}

impl Cpu {
    pub fn new() -> Cpu {
        let temps = get_temp_path(&"/sys/class/".to_string(), CPU_HWMON_NAME).unwrap();
        let temps = temps.get(CPU_HWMON_LABEL).cloned();
        Cpu {
            threads: [0.0; CPU_THREADS],
            total: 0.0,
            total_info: Tempor::new(),
            threads_info: [Tempor::new(); CPU_THREADS],

            temp: 0.0,
            temp_path: temps.expect("expected to find the CPU_HWMON_NAME path"),

            power_draw: 0.0,
            power_path: CPU_ENERGY_UJ_PATH.to_string(),
            power_timestamp: SystemTime::now(),
            power_temp_uj: 0,
        }
    }
    pub fn update(&mut self) {
        fn update_line(temp: &mut Tempor, line: &str) -> f32 {
            let idle = line
                .split_whitespace()
                .nth(4)
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            let mut fields = line.split_whitespace();
            let mut total = 0;
            fields.next(); // skip the name
            for field in fields {
                total += field.parse::<usize>().unwrap();
            }
            let util = total
                .saturating_sub(temp.total)
                .saturating_sub(idle.saturating_sub(temp.idle)) as f32
                / total.saturating_sub(temp.total) as f32;
            (temp.total, temp.idle) = (total, idle);
            util
        }
        {
            let content = fs::read_to_string("/proc/stat").expect("couldn't read /proc/stat");
            let mut lines = content.lines();
            self.total = update_line(&mut self.total_info, lines.next().unwrap());
            for num in 0..CPU_THREADS {
                self.threads[num] = update_line(&mut self.threads_info[num], lines.next().unwrap());
            }
        }
        {
            self.temp = parse_file::<f32>(&self.temp_path).unwrap() / 1000.0;
        }
        {
            let uj_new = parse_file::<u64>(&self.power_path).unwrap();
            let j = (uj_new - self.power_temp_uj) as f32 / 1000000.0;
            let new_time = SystemTime::now();
            let time = new_time
                .duration_since(self.power_timestamp)
                .unwrap()
                .as_millis() as f32
                / 1000.0;

            if time > 0.2 {
                self.power_draw = j / time;
            }

            self.power_temp_uj = uj_new;
            self.power_timestamp = new_time;
        }
    }
}

impl Tempor {
    fn new() -> Tempor {
        Tempor { total: 0, idle: 0 }
    }
}
