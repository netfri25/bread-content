use clap::Parser;

#[derive(Parser)]
pub struct Config {
    /// which battery to use, e.g. BAT1 (in /sys/class/power_supply)
    #[arg(short = 'b', long = "bat")]
    pub battery: Option<String>,

    /// which GPU to use, e.g. card0 (in /sys/class/drm, depends on drm/{gpu}/device/gpu_busy_percent)
    #[arg(short = 'g', long = "gpu")]
    pub gpu: Option<String>,

    /// which thermal component to use, e.g. acpitz (in /sys/class/hwmon/, and each hwmonX has /name)
    #[arg(short = 't', long = "thermal")]
    pub thermal: Option<String>,

    /// which wifi card to use, e.g. wlan0 (in /sys/class/net)
    #[arg(short = 'w', long = "wifi")]
    pub wifi: Option<String>,
}
