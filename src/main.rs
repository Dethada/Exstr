use std::fs::File;
use std::io::Read;
use clap::{Arg, App, crate_version};
use std::io::ErrorKind;
use std::process::exit;

fn main() {
    let matches = App::new("Exstr")
                        .version(crate_version!())
                        .author("David Z. <david@dzhy.dev>")
                        .about("Get strings from file")
                        .arg(Arg::with_name("file")
                            .short("f")
                            .long("file")
                            .required(true)
                            .help("The target file")
                            .takes_value(true))
                        .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let mut file = File::open(file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("File not found.");
        } else {
            println!("There was a problem opening the file.");
        }
        exit(1)
    });
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let mut count: u64 = 0;
    let mut last_index = 0;
    for (i, byte) in buf.iter().enumerate() {
        let c = *byte;
        if (c > 31 && c < 127) || (c == 9 || c == 13) {
            count += 1;
        } else {
            if count > 3 {
                println!("{}", String::from_utf8_lossy(&buf[last_index..i-1]));
            }
            count = 0;
            last_index = i + 1;
        }
    }
    if count > 3 {
        println!("{}", String::from_utf8_lossy(&buf[last_index..]));
    }
}
