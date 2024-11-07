This document describes how to setup the Zima embedded sensor.

## Setup the Zima
1. Unbox the Zima, get a miniDP cable, and let's get started.
1. Create installation media for Ubuntu Server LTS 24.04.1.
    1. The ISO image can be found [here](https://ubuntu.com/download/server).
1. Plug your usb into the Zima, and boot it to BIOS menu (probably F11).
1. Switch boot device to the usb, and install Ubuntu Server.
    1. Largely you can follow the defaults
    1. Name -> User
    1. Server's Name -> sensor\$sensorSerial\$ ex sensor1234
    1. User Acc -> sensor\$sensorSerial\$/ble-sense
    1. Install OpenSSH server
    1. We do not need any snaps
    1. Don't forget to remove installation media on reboot!

## Onboard Device to Marist Network
1. Get the MAC address of the interface you want to authenticate (Zima has eth0 and eth1, you won't need both)
    1. `ip a` or `ifconfig`
    1. Go to net.marist.edu and add this mac address to your account

## Set up rust project
1. `sudo apt install rustc cargo`
1. `mkdir sensorSrc`
1. `cd sensorSrc`
1. `git clone https://github.com/Oliver-Shariff/BLE_Sense`
    1. You will need a git token to do this.
    1. Depending on dev stage you may need to switch to sensor branch with git checkout.
1. `cd BLE_Sense/sensor_sniffer`
1. `cargo build` to build the code!

## Set up the sniffer
1. Get NRFutil executable
    1. `cd ~`
    1. `mkdir nrf`
    1. `cd nrf`
    1. `curl -L "https://files.nordicsemi.com/ui/api/v1/download?repoKey=swtools&path=external/nrfutil/executables/x86_64-unknown-linux-gnu/nrfutil&isNativeBrowsing=false" -o nrfutil`
    1. `ls` to verify it completed
    1. `head nrfutil` to make sure we got the file, not an error code (it'll be binary gibberish if it worked).
    1. `sudo chmod +x nrfutil` to make it executable
    1. `sudo cp nrfutil /usr/local/bin` to make it callable
1. Set up NRFutil
    1. `nrfutil` to verify we can call it and it works
    1. `nrfutil self-upgrade`
    1. `nrfutil install ble-sniffer device`
    1. `sudo apt install libusb-1.0-0`
    1. `curl -LO "https://github.com/NordicSemiconductor/nrf-udev/releases/download/v1.0.1/nrf-udev_1.0.1-all.deb"` 
    1. `sudo dpkg -i nrf-udev_1.0.1-all.deb`
1. Get SeggerJLink
    1. Unfortunatlely, we cannot curl this one (without simulating session cookies with curl) due to a html agreement form the user must fill out. I suggest either uploading it temporarily to github *PRIVATELY* or throwing it on a usb to get it onto the sensor.
    1. Get the DEB installer for Jlink found [here](https://www.segger.com/downloads/jlink/#J-LinkSoftwareAndDocumentationPack) onto the Zima.
    1. `sudo dpkg -i JLink_Linux_V810f_x86_64.deb`
        1. This will fail. 
        1. `sudo apt --fix-broken install`
            1. You may need to do `sudo apt install libxrender1` first.
        1. Now, `sudo dpkg -i JLink_Linux_V810f_x86_64.deb` again.
        1. This is easier than manually collecting all of the pkgs.
1. `reboot`
1. `nrfutil device list`
    1. Should say "0 supported devices" and not complain about any missing dependencies.
    1. Connect the dongle, and `nrfutil device list` again.
        1. It should recognize.

## Run the sensor!!
1. `cd ~/sensorSrc/BLE_Sense/sensor_sniffer`
1. `cargo run` Success!!
    1. Adjust the settings in config/config.ini as needed to fit desired behavior (use vi)
    1. Make sure to disable logging if you aren't going to read it (a lot of overhead) 
    1. Make sure to update the serialID
1. It now works!


