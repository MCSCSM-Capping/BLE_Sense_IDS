# !

# NOTE: NO LONGER RELEVANT, HOLDING UNTIL ZIMA SETUP

# !

This document shows how to set up the remote PI sensor. This was tested using PI3s from CanaKit.

# Raspberry PI Sensor Setup

### Initial Setup
1. Boot the device. Use installation media if Raspbian (based on Debian) is not already on the microSD Card.
1. Enter the 'Raspberry PI Configuration' (Menu->Preferences->). Under boot, select 'to cli' since we do not want to use the desktop when we are just using the sensor. From now on, issue `startx` to enter the desktop environment at boot.
1. Under interfaces, make sure ssh is enabled since we may want to remotely use it.
1. Make sure to set a password if ssh is enabled... we will be using the shared password "AlgozzineBLE123!" (rhymes!).
1. Also set all of the options under 'localization'

### Connecting to the network
1. Issue a `ifconfig` to see available interfaces. For the interface you want to use (probably Wi-Fi) it will come up as wlan0, as an example. Note the mac address for it (under "HWaddr").
1. Navigate to net.marist.edu and sign in with your marist credentials. 
1. Select 'add' and create a profile for the sensor. Add its mac address. 
1. Once you have done this, the PI can connect to the SSID 'foxgadget' with a psk of 'redfoxes'

### Setting up dependencies
1. Update your apt sources file located at /etc/apt/sources.list to include:
    1. deb http://raspbian.raspberrypi.org/raspbian/ buster main contrib non-free rpi
    1. deb-src http://raspbian.raspberrypi.org/raspbian/ buster main contrib non-free rpi
    1. Also uncomment the one commented out
    1. Example: `sudo nano /etc/apt/sources.list`
1. `sudo apt update`
1. `sudo apt full-upgrade` (this will take FOREVER)
1. `sudo apt install binutils libc6-dev libssl-dev pkg-config --fix-broken`


### Installing Rust
1. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
1. Select the defaults.
1. Ensure installation with `rustc --version` or `cargo --version`

### Get the Sensor Code
1. `git clone https://github.com/Oliver-Shariff/BLE_Sense` (put an access token in for password)
1. `cd BLE_Sense`
1. `git checkout Sensor-BLE-Collection`
1. `cd sensor_sniffer`
1. `cargo build` will set up the dependencies it might take awhile.

### Install NRF Tools
1. Open the browser (again, slow)
1. Install NRFUtil from [their site](https://www.nordicsemi.com/Products/Development-tools/nRF-Util).
1. Find where it downloaded to and `chmod +x nrfutil`
1. Add to system path: `sudo mv nrfutil /usr/local/bin/nrfutil`


