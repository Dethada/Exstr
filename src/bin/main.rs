use std::fs::File;
use std::io::Read;
use clap::{Arg, App, crate_version};
use std::io::ErrorKind;
use std::process::exit;
use exstr;

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
        exit(1);
    });
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let result = exstr::get_strings(buf, 4);
    println!("{}", result);
}
