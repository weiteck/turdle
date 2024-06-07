# Turdle
### _A Wordle clone for the terminal_

![Turdle screenshot](https://files.catbox.moe/ijnh30.png)

Turdle is a clone of the popular browser-based word game Wordle by Josh Wardle, now owned by The New York Times. The (admittedly crude) name is a concatenation of **TU**I + Wo**rdle**.

A random word is selected each time the game is run. Run `turdle --help` for additional options.

### Features
* The same pool of 2,309 answers and 12,546 valid words from the original game
* Large-font gameboard reminiscent of the original game
* Animated letter reveals and invalid word feedback
* 'Keyboard' showing the state of each letter you've used
* Play today's Wordle with `turdle today`
* Play a specific date's Wordle with `turdle date <YY-MM-DD>`

### Special Keys
* Use <kbd>Tab</kbd> to toggle the keyboard layout or set `TURDLE_QWERTY_MODE=1` to always start with QWERTY layout
* Use <kbd>Page Up</kbd> and <kbd>Page Down</kbd> to cycle the background colour and <kbd>Home</kbd> to reset it

## Installation
Linux and Windows binaries are available on the [releases](https://github.com/weiteck/turdle/releases) page.

Alternatively, if you already have Rust [installed](https://rustup.rs), simply run:
```
cargo install turdle --locked
```
If you encounter a linker error, your system may be missing the required compiler toolchain:

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
Due to the large font used for the gameboard, **your terminal must be at least 34 rows high** for it to render correctly.

Turdle targets modern terminal emulators. The large font rendering is unlikely to work running in the Windows command prompt or Powershell, but it does work on later versions of [Windows Terminal](https://github.com/microsoft/terminal).

---
### License
MIT or Apache 2.0 at your discretion.
