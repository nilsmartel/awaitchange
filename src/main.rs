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
    #[structopt(long = "watch", help = "Files to be watched")]
    files: Vec<String>,

    /// Command to be executed on filechange.
    /// If unset, awaitchange simply exits on filechange
    /// and yields controll to the programm next in line.
    #[structopt(long = "do")]
    command: Option<Vec<String>>,
}

impl Arguments {
    fn get_command(&self) -> Option<(&str, &[String])> {
        match &self.command {
            None => None,
            Some(args) => {
                if args.len() == 0 {
                    None
                } else {
                    Some((&args[0], &args[1..]))
                }
            }
        }
    }
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
    match args.get_command() {
        None => std::process::exit(1),
        Some((command, arguments)) => {
            let output = std::process::Command::new(command)
                .args(arguments)
                .output()
                .expect("failed to execute command");
            // unsafe can easily be avoided here
            // but it's the easiest thing to do
            print!("{}", unsafe { String::from_utf8_unchecked(output.stdout) });

            // also print stderr
            print!("{}", unsafe { String::from_utf8_unchecked(output.stderr) });
        }
    }
}

fn last_modified(path: &str) -> SystemTime {
    let metadata = std::fs::metadata(path).unwrap();
    metadata
        .modified()
        .expect(&format!("Failed to watch for updates in file {}", path))
}
