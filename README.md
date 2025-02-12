# dbus api example in Rust with NetworkManager

This simple example prints out the active access SSID point for some Wi-fi device
(`wlp170s0`) and prints it out to the terminal.

This was going to be used for my [status bar](https://github.com/DMGDy/epic-bar-rs) I am working on, but I realized
dbus is far slower than interacting with the [netlink](https://man7.org/linux/man-pages/man7/netlink.7.html) mechanism
as using dbus in my case introduces 2 layers of abstraction (dbus and NetworkManager) to obtain basic information 
which eventually could just be boiled down to netlink.
