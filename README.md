# Anyrun Cliphist Plugin

This repository contains a plugin for [anyrun](https://github.com/anyrun-org/anyrun) and [cliphist](https://github.com/aa11653/anyrun-cliphist-plugin) that allows users to select / delete clipboard items and wipe history.

## Demo

<video width="800" controls>
    <source src="assets/demo.mp4" type="video/mp4">
    Your browser does not support the video tag.
</video>

After the plugin correctly configured, you can

- `Ctrl + C`: Copy text or image to clipboard, it will be stored in cliphist
- `Super + V`: Open anyrun clipboard manager, select items, or wipe history
- `Ctrl + V`: Paste the selected item


## Quick Start for Arch Linux + Hyprland

### Build and Install anyrun, cliphist and this plugin

Install anyrun from AUR. 

Install `extra/cliphist` from Pacman
```bash
sudo pacman -S cliphist
```

Build this plugin, now the `.so` file is in `target/release/libanyrun_cliphist.so`
```bash
cargo build --release
```

### Configure this plugin

Copy Example configuration in `config_example/*` to `~/.config/anyrun_cliphist/*`. And copy the compiled `.so` file to `~/.config/anyrun/plugins/`

> *It is weird that the plugin will not be found when placed in custom config directory, therefore, we have to copy plugin to the original directory.*

```bash
# Create config directory if not exists
mkdir -p ~/.config/anyrun_cliphist
# Copy the example config and style files
cp config_example/* ~/.config/anyrun_cliphist/
# Copy the plugin .so file to anyrun plugins directory
cp target/release/libanyrun_cliphist.so ~/.config/anyrun/plugins/
```

Modify the `style.css`, `config.ron` and `cliphist.ron` files in `~/.config/anyrun_cliphist/` to your needs.

### Configure hyprland

Add the following lines to your `~/.config/hypr/hyprland.conf` file for watching clipboard changes and storing them in cliphist.

See [Hyprland Wiki](https://wiki.hypr.land/Useful-Utilities/Clipboard-Managers/#cliphist) for more details about cliphist and hyprland integration.

```
exec-once = wl-paste --type text --watch cliphist store
exec-once = wl-paste --type image --watch cliphist store
```

Add the following lines to your `~/.config/hypr/hyprland.conf` file to set up keybindings for the plugin:

```
bind = $mainMod, V, exec, anyrun --config-dir ~/.config/anyrun_cliphist
```

> *Note that if you use a older version of anyrun, the `show_result_immediately` will not work well, see issue https://github.com/anyrun-org/anyrun/pull/239*
