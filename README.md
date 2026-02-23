# Bread Content
status program that emits output to be rendered using bread

for more information, visit [bread](https://github.com/netfri25/bread)

#### Installation
if you don't have Rust on your system, [install it](https://rustup.rs)
```shell
cargo install --git https://github.com/netfri25/bread-content
```

#### Usage
intended to be used with [bread](https://github.com/netfri25/bread)

```shell
bread-content | bread --font "Iosevka-Custom"
```

for more configuration options, check out `bread-content --help`

#### Shown Components
 - focused app id
 - focused app title
 - time, day of week, date
 - colorful GPU usage bar with 25 colors
 - colorful CPU usage bars (one for each core) with 25 colors
 - temperature in Celsius with 4 different colors for 0-40, 40-50, 50-70, >=70
 - RAM usage in MB
 - WIFI status & signal strength
 - battery percentage & charge status (-,+,o) & time left for charge/discharge (considering charge limits!)

#### Additional Features
 - reduced movements, less distraction. only the charge/discharge time disappears when battery is full. other than that, everything else stays the same size.
 - focused app title character limit
