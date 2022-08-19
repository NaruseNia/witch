use std::process::Command;
use clap::Parser;
use colored::Colorize;
use get_shell::get_shell_name;

#[derive(Parser, Debug)]
#[clap(author = "narusenia",
       version = "0.1",
       about = "Magical alternative to the which command.")]
struct Args {
    ///Print all matching pathnames of each argument
    //#[clap(short, long, takes_value = false)]
    //all: bool,
    
    #[clap(required = true)]
    filename: Vec<String>,
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
    let all: &str = if args.all { "-a" } else { "" };
    let cmd = if shell == "pwsh" || shell == "cmd" {
        Command::new("pwsh")
                .arg("-c")
                .arg(format!("Get-Command {} -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Definition", &args.filename.first().expect("err!")))
                .output()
                .ok().expect("Error: Failed to run command.")
    } else if shell == "bash" {
        Command::new("which")
                .arg(&args.filename.first().expect("err!"))
                .arg(&all)
                .output()
                .ok().expect("Error: Failed to run command.")
    } else {
        Command::new("bash")
                .args(["which", &args.filename.first().expect("err!"), &all])
                .output()
                .ok().expect("Error: Failed to run command.")
    };
    
    let mut res: &str = match std::str::from_utf8(&cmd.stdout) {
        Err(why) => panic!("Error: {}", why),
        Ok(res) => res,
    };

    if res.trim() == "" { res = "not executable."; }

    print!(witch!(), target = &args.filename.first().expect("err!").yellow(), where = &res.trim().cyan());
}
