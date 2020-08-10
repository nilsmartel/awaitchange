use std::time::{Duration, SystemTime};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Arguments {
    /// Determines how many times per second each file should be checked
    /// Usually 2 times seems reasonable
    #[structopt(
        short = "r",
        default_value = "2",
        help = "How many times per second files should get checked for updates"
    )]
    checkrate: usize,

    /// List of files to be watched.
    /// If any of these files changes, event will be fired.
    #[structopt(long = "files", help = "Files to be watched")]
    files: Vec<String>,

    /// Command to be executed on filechange.
    /// If unset, awaitchange simply exits on filechange
    /// and yields controll to the programm next in line.
    #[structopt(long = "do", default_value = "")]
    command: Option<Vec<String>>,
}

fn main() -> std::io::Result<()> {
    let args = Arguments::from_args();
    let checkrate = Duration::from_secs_f32(1.0 / args.checkrate as f32);
    let mut modified = Vec::new();

    loop {
        for (i, file) in args.files.iter().enumerate() {
            let date = last_modified(file);
            if modified.len() > i {
                if modified[i] != date {
                    onchange(&args);
                    modified[i] = date;
                }
            } else {
                modified.push(date);
            }
        }

        std::thread::sleep(checkrate);
    }
}

fn onchange(args: &Arguments) {
    match &args.command {
        None => std::process::exit(1),
        Some(args) => {
            let command = &args[0];
            let arguments = &args[1..];
            let output = std::process::Command::new(command)
                .args(arguments)
                .output()
                .expect("failed to execute command");
            print!("{}", String::from_utf8(output.stdout).unwrap());
        }
    }
}

fn last_modified(path: &str) -> SystemTime {
    let metadata = std::fs::metadata(path).unwrap();
    metadata
        .modified()
        .expect(&format!("Failed to watch for updates in file {}", path))
}
