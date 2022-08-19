# **Witch**
## "Magical alternative to the which command" 
![witch_command](https://i.imgur.com/2UnPTs3.png)

If you've been coding until midnight and accidentally typo'd the `which` command with the `witch` command, no worries!
A kind **witch** will teach you instead!

Maybe on Windows it could be used as a `which` command! (It takes up a bit of line, though...)

---

## Usage
```
witch [OPTIONS] <FILENAME>
```
Options:
```
-a, --all        Print all matching pathnames of each argument
-h, --help       Print help information
-V, --version    Print version information
```

---

## Build / Install

First, clone this repository and move to the directory.
```bash
git clone https://github.com/NaruseNia/witch.git && cd witch
```

And build or install!!
```bash
cargo build --release
```
or 
```bash
cargo install --path .
```
