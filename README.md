# **Witch**
## "Magical alternative to the which command" 
![witch_command](https://i.imgur.com/2UnPTs3.png)

If you've been coding until midnight and accidentally typo'd the `which` command with the `witch` command, no worries!
A kind **witch** will teach you instead!

---

## Build

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

Also, you can install witch like this

**Bash**
```bash
sudo cp ./target/release/witch /usr/bin/witch
```

**Windows(PowerShell)**
```pwsh
$ENV:Path+=";path\to\witch\target\release\witch.exe"
```

**Windows(Cmd)**
```cmd
set PATH=%PATH%;path\to\witch\target\release\witch.exe
```