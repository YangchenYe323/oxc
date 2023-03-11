use std::{
    io::{stderr, Write},
    process::{exit, Command},
};

use oxc_parser::generate_keyword_table;

fn main() {
    let mut args = std::env::args();
    args.next();
    let output_path = args.next().expect("Please specify an output path");
    let mut file = std::fs::File::create(&output_path)
        .map_err(|err| {
            writeln!(stderr(), "Couldn't Open Path: {} because of {}", output_path, err).unwrap();
            exit(-1)
        })
        .unwrap();

    let code = generate_keyword_table();

    write!(&mut file, "{}", code)
        .map_err(|err| {
            writeln!(stderr(), "Write to file Failed: {}", err).unwrap();
            exit(-1)
        })
        .unwrap();

    Command::new("cargo").arg("fmt").arg("--").arg(&output_path).output().unwrap();
}
