pub const GPU_PATH: &str =
    "/sys/devices/pci0000:00/0000:00:03.1/0000:2b:00.0/0000:2c:00.0/0000:2d:00.0/";

pub const CPU_THREADS: usize = 24;
pub const CPU_HWMON_NAME: &str = "k10temp";
pub const CPU_HWMON_LABEL: &str = "Tctl";
pub const CPU_ENERGY_UJ_PATH: &str = "/sys/class/powercap/intel-rapl:0/energy_uj";

pub const SWAP: bool = false;
