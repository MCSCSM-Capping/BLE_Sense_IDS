This document shows how to set up the environment to run the sensor locally.

# Test Environment Setup
### Install Rust
1. Since the sensor is written in rust, you will need the appropriate tools to build and run the program.
2. If your environment has a package manager, rust can likely be installed that way. We are going to use their installer. 
3. Download it from their [site](https://www.rust-lang.org/tools/install).
4. You can also get it via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
5. Run the installer. Defaults are fine.
6. Add rust to your path if the installer failed to do so.
7. `rustc --version` or `cargo --version` after a terminal restart to ensure installation success.

### Clone the repository
1. Assuming you have git installed, clone https://github.com/Oliver-Shariff/BLE_Sense/tree/Sensor-BLE-Collection.
2. For now, you need to switch to the Sensor-BLE-Collection branch (using git checkout).
3. Once you are on the correct branch, cd into Sensor_Sniffer.
4. `cargo build ` will install all dependencies. This process should succeed.

### Set up the NRF Sniffer
Additional documentation can be found on [their site](https://docs.nordicsemi.com/bundle/nrfutil/page/README.html).
1. Download the nrfutil executable from [their site](https://www.nordicsemi.com/Products/Development-tools/nRF-Util/Download#infotabs). Put this executable in your system path or `chmod +x nrfutil`.
2. Download [SEGGER j-link](https://www.segger.com/downloads/jlink/#J-LinkSoftwareAndDocumentationPack) and follow the defaults when running the installer. Select "Install legacy USB Driver just in case. 
   #### Windows
   1.  Install [Microsoft Visual C++ Redistributable](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#visual-studio-2015-2017-2019-and-2022).
   1. Install [nrf-device-lib driver](https://github.com/NordicSemiconductor/pc-nrfconnect-launcher/blob/main/build/drivers/nrf-device-lib-driver-installer.exe)
   #### Linux/Mac
   1. `sudo apt install libusb-1.0-0`
   1. Download [nrf-udev](https://github.com/NordicSemiconductor/nrf-udev), then `sudo dpkg -i nrf-udev_1.0.1-all.deb`
1. nrfutil comes bare bones on install. We will get the additional functionality we need.
   1. `nrfutil self-upgrade`
   2. `nrfutil search` to see available tools to install
   3. `nrfutil install device ble-sniffer` to install the tools we are going to use
   4. `nrfutil device-list` will confirm it is all working and should display the port of your sniffer (granted it is connected).

### Setting the Configuration
1. Open the config/config.ini file.
1. Set the settings to match the behavior you desire. The file comments will help!

### Running the sensor
1. The sensor has differnet logging levels. Ex: trace to see all packets, info to see just high level system messages.
1. Run the sensor with your chosen log level: `RUST_LOG=info cargo run`
1. Since the logging crate writes to stderr, you need to capture that as well if you are piping the output to a file.
   1. Ex. `RUST_LOG=info cargo run > output.log 2>&1` 
1. The config.ini file (config/config.ini) contains settings for the sniffer to use. Alter this file to change the configuration. For example, if you want a pcapng capture file to be made, change 'PCAPNG' to 'TRUE'.  




