use clap::{App, Arg};
use hotwatch::{Event, Hotwatch};

fn main() {
    let matches = App::new("awaitchange")
        .version("1.0")
        .author("Nils Martel - <nilsmartel@yahoo.de>")
        .about("Waits for file changes and exits")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("File to watch")
                .takes_value(true),
        )
        .get_matches();

    if let Some(filename) = matches.value_of("file") {
        let mut watch = Hotwatch::new().unwrap();
        watch
            .watch(filename, |event: Event| match event {
                Event::Write(_) | Event::NoticeWrite(_) => {
                    std::process::exit(0);
                }
                _ => std::process::exit(-1),
            })
            .unwrap();
    } else {
        eprintln!("{}", matches.usage.expect("Internal error."));
    }
}
