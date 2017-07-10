# Macros
This reposity will contain the controller, `actkbd` configuration file, and a template for a systemd service file for `actkbd`.

Suggested use with an external usb keypad.

See: https://github.com/thkala/actkbd

```bash
#Some commands used for setting up; incomplete and in no particular order

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
