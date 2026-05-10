use last_update_time::last_update_time;
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
    files: Vec<String>,

    /// Command to be executed on filechange.
    /// If unset, awaitchange simply exits on filechange
    /// and yields controll to the programm next in line.
    /// The special character {} can be inserted and
    /// will be replaced by the name of the file that has changed upon execution
    #[structopt(long = "exec", short = "-e")]
    exec: Option<Vec<String>>,
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
    let Arguments {
        checkrate,
        files,
        exec,
    } = Arguments::from_args();
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

fn onchange(command: &Option<Vec<String>>, file: &str) {
    match command {
        None => std::process::exit(1),
        Some(command) => {
            let command = command.iter().map(|val| if val == "{}" {file} else {val}).collect::<Vec<_>>();
            let output = std::process::Command::new(&command[0])
                .args(&command[1..])
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
