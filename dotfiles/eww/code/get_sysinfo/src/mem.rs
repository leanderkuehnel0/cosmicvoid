use super::*;

#[derive(Debug, Serialize, Clone, Copy)]
pub struct Mem {
    pub used: f32,
    pub other: f32,
    pub total: f32,
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            used: 0.0,
            other: 0.0,
            total: 0.0,
        }
    }
    pub fn update(&mut self) {
        let contents = fs::read_to_string("/proc/meminfo").unwrap();
        for line in contents.lines() {
            if line.starts_with("MemTotal") {
                self.total = parse_line(line);
            } else if line.starts_with("MemFree") {
                self.other = self.total - parse_line(line);
            } else if line.starts_with("MemAvailable") {
                self.used = self.total - parse_line(line);
            }
        }
        self.other -= self.used;
    }
    pub fn update_swap(&mut self) {
        let contents = fs::read_to_string("/proc/meminfo").unwrap();
        for line in contents.lines() {
            if line.starts_with("SwapTotal") {
                self.total = parse_line(line)
            } else if line.starts_with("SwapFree") {
                self.used = self.total - parse_line(line)
            }
        }
    }
}
