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



nrfutil.exe https://www.nordicsemi.com/Products/Development-tools/nRF-Util/Download#infotabs
SEGGER https://www.segger.com/downloads/jlink/#J-LinkSoftwareAndDocumentationPack

nrfutil search
nrfutil install device ble-sniffer

nrfutil device-list to confirm working
