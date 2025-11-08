use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[arg(short = 'b', long = "bat", default_value = "BAT1")]
    pub battery: String,

    #[arg(short = 'g', long = "gpu", default_value = "card0")]
    pub gpu: String,

    #[arg(short = 't', long = "thermal", default_value = "thermal_zone0")]
    pub thermal: String,

    #[arg(short = 'w', long = "wifi", default_value = "wlp98s0")]
    pub wifi: String,
}
