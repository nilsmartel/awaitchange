use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Arguments {
    #[structopt(short = "r", default_value = "2")]
    checkrate: usize,

    files: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let args = Arguments::from_args();
    let checkrate = Duration::from_secs_f32(1.0 / args.checkrate as f32);
    let mut modified = Vec::new();

    loop {
        for (i, file) in args.files.iter().enumerate() {
            let date = last_modified(file)?;
            if modified.len() > i {
                if modified[i] != date {
                    std::process::exit(1);
                }
            } else {
                modified.push(date);
            }
        }

        std::thread::sleep(checkrate);
    }
}

fn last_modified(path: &str) -> std::io::Result<SystemTime> {
    let metadata = std::fs::metadata(path)?;
    metadata.modified()
}
