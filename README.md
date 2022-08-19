# **witch**
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
You can also download pre-built things on [Release](https://github.com/NaruseNia/witch/releases/tag/stable).

---

## TODO
- Support multiple filenames.
- Create `--all` option.