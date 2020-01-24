use std::time::SystemTime;
fn main() -> std::io::Result<()> {
    let names = std::env::args().skip(1).collect::<Vec<_>>();
    let mut modified = Vec::new();

    loop {
        for (i, file) in names.iter().enumerate() {
            let date = last_modified(file)?;
            if modified.len() > i {
                if modified[i] != date {
                    std::process::exit(1);
                }
            } else {
                modified.push(date);
            }
        }
    }
}

fn last_modified(path: &str) -> std::io::Result<SystemTime> {
    let metadata = std::fs::metadata(path)?;
    metadata.modified()
}
