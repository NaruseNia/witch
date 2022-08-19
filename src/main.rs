use std::process::Command;
use clap::Parser;
use colored::Colorize;
use get_shell::get_shell_name;

#[derive(Parser)]
struct Args {
    executable: String
}

// You can do the same thing with the {command} command!
macro_rules! witch {
() => (
"
 ⠀⠀⠀⣠⣦⣄⡀
⠀⠀⠀⠴⠛⢻⣿⣿⣷⣦⡀
⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣦⡀
⠀⠀⠀⠀⠀⢠⣿⣯⣭⣤⣷⣷⣶⣶⣶⣦⣤⣄
⠀⠀⢀⣴⣾⣿⡿⣿⣿⣿⡿⠉⣽⣿⢿⠉
⠠⢾⣿⣛⣽⠿⢻⢿⠟⠀⠀⠐⢸⡻⠎⠀<{target} is {where}
⠀⠀⠀⠀⢫⠐⣗⣧⣝⢦⣈⣫⣿⡟⡇
⠀⠀⠀⣠⣾⠿⠙⣿⢈⢷⣛⢿⢏⢣⢧
⠀⣠⣞⡵⣍⡀⠀⡇⡆⣿⣿⣳⢤⡟⠺
⠸⣻⣋⣼⣿⣿⣿⡇⡇⣿⣽⣿⣱⣊⣶⡀
⠀⠻⣾⡻⣿⣿⣿⣷⡇⢟⣯⣻⣿⢻⣯⣿⡄
⠀⠀⠈⠙⠶⣶⣾⣷⣿⣜⢮⠛⠧⣿⡿⠋⠁
⠀⠀⠀⠀⠀⠀⠀⠉⠛⠛⠈⠛⠛⠁⠀
"
)
}

fn main() {
    let args: Args = Args::parse();
                    
    let shell = get_shell_name().unwrap();
    let cmd = if shell == "pwsh" || shell == "cmd" {
        Command::new("pwsh")
                .arg("-c")
                .arg(format!("Get-Command {} -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Definition", &args.executable))
                .output()
                .ok().expect("Error: Failed to run command.")
    } else if shell == "bash" {
        Command::new("which")
                .arg(&args.executable)
                .output()
                .ok().expect("Error: Failed to run command.")
    } else {
        Command::new("bash")
                .args(["which", &args.executable])
                .output()
                .ok().expect("Error: Failed to run command.")
    };
    
    let mut res: &str = match std::str::from_utf8(&cmd.stdout) {
        Err(why) => panic!("Error: {}", why),
        Ok(res) => res,
    };

    if res.trim() == "" { res = "not executable."; }

    print!(witch!(), target = &args.executable.yellow(), where = &res.trim().cyan());
}
