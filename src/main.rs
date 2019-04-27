use std::fs::File;
use std::io::Read;
use clap::{Arg, App};
use std::io::ErrorKind;
use std::process::exit;

fn main() {
    let matches = App::new("chars")
                        .version("0.1")
                        .author("David Z. <david@dzhy.dev>")
                        .about("Display printable characters in file")
                        .arg(Arg::with_name("file")
                            .short("f")
                            .long("file")
                            .required(true)
                            .help("The target file")
                            .takes_value(true))
                        .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let file = File::open(file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("File not found.");
        } else {
            println!("There was a problem opening the file.");
        }
        exit(1)
    });

    for byte in file.bytes() {
        let c = byte.unwrap();
        if (c > 31 && c < 127) || (c == 9 || c == 10 || c == 13) {
            print!("{}", c as char);
        }
    }
}
