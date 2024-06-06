# Turdle
_A **TU**I Wo**rdle** clone for the terminal_

Turdle is a clone of the popular browser-based word game Wordle by Josh Wardle, now owned by The New York Times.
A random word is selected each time the game is run.

### ⭐ Features
* The same pool of 2,309 answers and 12,546 valid words from the original game
* Large-font gameboard reminiscent of the original game
* Animated letter reveals and invalid word feedback
* 'Keyboard' showing the state of each letter you've used

### ⌨️ Special Keys
* Use <kbd>Tab</kbd> to toggle the keyboard layout or set `TURDLE_QWERTY_MODE=1` to start with QWERTY layout
* Use <kbd>Page Up</kbd> and <kbd>Page Down</kbd> to cycle the background colour
* Use <kbd>Home</kbd> to reset the background colour

### 📸 Screenshot
![Turdle screenshot](https://files.catbox.moe/ijnh30.png)

## Installation
Linux and Windows binaries are available on the [releases](https://github.com/weiteck/turdle/releases) page.

Alternatively, if you have Rust [installed](https://rustup.rs), you can install by running:
```
cargo install turdle --locked
```
If you encounter a missing linker error, your system may be missing the required compiler toolchain:

_Arch_
```
sudo pacman -S base-devel
```

_Debian/Ubuntu_
```
sudo apt install build-essential
```

_Centos_
```
sudo yum install gcc
````
### Compatibility
Testing has been on modern terminal emulators. The large font rendering may not work on Windows command prompt or Powershell, but should work on later versions of [Windows Terminal](https://github.com/microsoft/terminal).

---
### License
MIT or Apache 2.0 at your discretion.
