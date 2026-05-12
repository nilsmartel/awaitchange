use clap::Parser;
use last_update_time::last_update_time;
use std::time::{Duration, SystemTime};

#[derive(Parser)]
struct Arguments {
    /// Determines how many times per second each file should be checked
    /// Usually 2 times seems reasonable
    #[arg(
        short = 'r',
        default_value = "2",
        help = "How many times per second files should get checked for updates"
    )]
    checkrate: usize,

    /// List of files to be watched.
    /// If any of these files changes, event will be fired.
    #[arg(help = "Files to be watched")]
    files: Vec<String>,

    /// Command to be executed on filechange.
    /// If unset, awaitchange simply exits on filechange
    /// and yields controll to the programm next in line.
    /// The symbol {} can be used and
    /// will be replaced by the name of the file that has changed upon execution
    #[arg(long = "exec", short = 'e')]
    exec: Option<String>,
    // /// Determines whether or not the screen should be cleared
    // /// before an command gets executed
    // #[arg(long = "clear")]
    // clear: bool,

    // /// determines whether or not the command should be executed
    // /// before any further actions
    // #[arg(long = "run")]
    // initial_run: bool,
}

fn main() -> std::io::Result<()> {
    let Arguments {
        checkrate,
        files,
        exec,
    } = Arguments::parse();
    let checkrate = Duration::from_secs_f32(1.0 / checkrate as f32);

    let mut modified = SystemTime::now();
    loop {
        let (last, file) = files
            .iter()
            .map(|f| {
                let t = last_update_time(f).unwrap_or_else(|error| {
                    use std::process::*;
                    eprintln!("checking file {}: {}", f, error.to_string());
                    exit(1)
                });
                (t, f)
            })
            .max_by_key(|(a, _)| *a)
            .unwrap();

        if last != modified {
            modified = last;
            onchange(&exec, &file);
        }

        std::thread::sleep(checkrate);
    }
}

fn onchange(command: &Option<String>, file: &str) {
    match command {
        None => std::process::exit(1),
        Some(command) => {
            let command = command.replace("{}", file);

            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(command)
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
