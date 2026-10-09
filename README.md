# Macros
A macro system that uses an external USB keypad to trigger hot-swappable exectuable tasks.

This system uses [actkbd](https://github.com/thkala/actkbd) for low level evdev bindings to the device, and a custom built controller (made with Rust) to perform task dispatch.

This system is run as a user-level systemd process, and fully lives in user-space.

> [!NOTE]
> This repo contains my first piece of published Rust code!

## Setup

These commands are incomplete, and roughly detail the installation process:
1. Installing the udev rules so the USB device can be controlled from user-space.
2. Building the high-level controller
3. Installing the controller, actkbd configuration, and systemd unit file to ~/etc/macros
4. Starting the system

```bash
set -e

# Input group, reboot for effects
sudo groupadd -f input
sudo gpasswd -a $USER input

# Copy udev rules for input group
echo TODO

# Build controller
cd ./controller/
cargo build --release
cd ..

# Install to ~/etc/macros/
mkdir -p ~/etc/macros
cp ./controller/target/release/controller ~/etc/macros/controller
echo TODO: Copy specific service file
echo TODO: Copy specific actkbd.conf file
cd ~/etc/macros
mkdir -p ./by-exe

# Link actkbd.service
mkdir -p ~/.config/systemd/user
ln -s ${PWD}/actkbd.service ~/.config/systemd/user/actkbd.service

# Start service
systemctl --user enable actkbd.service
systemctl --user start actkbd.service
```
