<div align="center">
  <img src="assets/banner.png" alt="ArchCleaner Banner" width="800">

  # ArchCleaner v0.1
  
  *A sleek, minimalistic, and high-performance TUI system cleaner exclusively designed for Arch Linux.*

  [![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/)
</div>

---

## What is ArchCleaner?

ArchCleaner is a blazingly fast Terminal User Interface (TUI) tool designed to help you regain disk space on your Arch Linux (or CachyOS) machine. It provides a unified checklist of all the common areas where useless files, cached downloads, and orphaned packages accumulate, allowing you to clean them seamlessly without ever leaving your terminal.

## Features

- **Pacman & Paru Cleanup**: Automatically detects and clears old package caches (keeping the 1 latest version by default) and removes unused orphaned packages.
- **AUR Build Caches**: Safely purges leftover build files from AUR helpers like `paru`.
- **Systemd Journal Vacuum**: Vacuums old systemd journal logs to strictly keep the last 2 weeks, saving potentially gigabytes of disk space.
- **Flatpak & Snap Runtimes**: Automatically removes unused Flatpak runtimes and disabled Snap packages.
- **User Cache**: Clears out heavy `.cache` directories (e.g. `yay`, `google-chrome`, `mozilla/firefox`, `thumbnails`, `electron` caches).
- **Secure Privilege Escalation**: Includes an intuitive password prompt overlay. It pipes your password directly via standard input to `sudo -S`, ensuring a fluid workflow without throwing you back to the raw shell.

## Installation

Ensure you have Rust and Cargo installed, then clone and build the project:

```bash
git clone https://github.com/yourusername/ArchCleaner.git
cd ArchCleaner
cargo build --release
```

The executable will be located at `target/release/CachyOSCleaner`. You can move this to your `~/.local/bin` or anywhere in your `$PATH`.

```bash
cp target/release/CachyOSCleaner ~/.local/bin/archcleaner
```

## Usage

Simply run the tool from your terminal:

```bash
archcleaner
```

### Controls:
- `Up / Down` or `k / j`: Navigate the checklist.
- `Enter`: Toggle selection for an item.
- `c`: Initiate the cleaning process for selected items.
- `q` or `Esc`: Quit the application.

## Requirements

- **OS**: Arch Linux (or Arch-based distros like CachyOS, EndeavourOS, Manjaro)
- **Tools**: `pacman`, `sudo`, `journalctl`. Optional: `paru`, `flatpak`, `snap`.

## Contributing

Contributions are always welcome! Feel free to open an issue or submit a pull request if you want to add new cleaning categories or improve the UI.

## License

MIT License. See `LICENSE` for more information.
