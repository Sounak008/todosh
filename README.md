# todosh

A minimalist, high-performance ToDo/Kanban board for the terminal.

## Showcase

<video src="./Video/showcase.mp4" controls="controls" style="max-width: 100%;">
  Your browser does not support the video tag.
</video>

---

## Keybindings

| Action | Keybinding |
| :--- | :--- |
| **Navigation** | `Up` / `Down` / `Left` / `Right` Arrows |
| **New Task** | Type in input box + `Enter` |
| **Delete Task** | `Delete` |
| **Move Task Forward** | `Shift` + `Right Arrow` |
| **Move Task Backward** | `Shift` + `Left Arrow` |
| **Quit** | `q` |

---

## Data Storage

`todosh` automatically synchronizes your data to the local filesystem/storage on every change made while using the program. Data is stored in JSON format at the following OS-specific locations:

* **Linux**: `~/.config/todosh/tasks.json`
* **macOS**: `~/Library/Application Support/todosh/tasks.json`
* **Windows**: `%APPDATA%\todosh\tasks.json`

---

## Installation & Build

### Pre-made Executables
You can download and use the latest version from the [Release Page](https://github.com/Sounak008/todosh/releases/latest).
* Download the correct executable for your OS.
* Open your terminal and run todosh:
```bash
./path/to/todosh-xxxx
```

### Prerequisites
Ensure the [Rust toolchain](https://rustup.rs/) is installed on your system.

### Build from Source
To compile a release binary:

```bash
cargo build --release
```
The resulting executable will be located at `./target/release/todosh`.

### Global Installation
To install the binary to your shell's path:

```bash
cargo install --path .
```
Then, use with `todosh`.
