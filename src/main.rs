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
    #[structopt(help = "Files to be watched")]
    watch: Vec<String>,

    /// Command to be executed on filechange.
    /// If unset, awaitchange simply exits on filechange
    /// and yields controll to the programm next in line.
    #[structopt(long = "do")]
    command: Option<String>,
    // /// Determines whether or not the screen should be cleared
    // /// before an command gets executed
    // #[structopt(long = "clear")]
    // clear: bool,

    // /// determines whether or not the command should be executed
    // /// before any further actions
    // #[structopt(long = "run")]
    // initial_run: bool,
}

fn main() -> std::io::Result<()> {
    let args = Arguments::from_args();
    let checkrate = Duration::from_secs_f32(1.0 / args.checkrate as f32);
    let mut modified = Vec::new();

    loop {
        for (i, file) in args.watch.iter().enumerate() {
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
        Some(command) => {
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(&command)
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
