extern crate bellperson;
extern crate paired;
extern crate clap;
extern crate anyhow;

use bellperson::groth16::Parameters;
use paired::bls12_381::Bls12;
use std::fs::File;
use std::io::BufReader;
use clap::{App, Arg, ArgMatches};
use anyhow::{Result, Error};
use std::process::exit;

fn load_groth16_params(path: &String) -> Parameters<Bls12> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let params = Parameters::read(reader, true).unwrap();

    params
}

fn print_groth16_params() {

}

fn run(matches: &ArgMatches) -> Result<()> {
    if matches.is_present("path") {
        let path = matches.value_of("path").unwrap();
        load_groth16_params(&path.to_string());

        return Ok(())
    }

    Err(Error::msg("path is not specified"))
}

fn main() {
    let matches = App::new("fil-params-parser")
        .version("0.1")
        .arg(
            Arg::with_name("path")
                .value_name("PATH")
                .takes_value(true)
                .short("p")
                .long("path")
                .help("the path in which the Groth 16 parameter file is located")
        ).get_matches();

    match run(&matches) {
        Ok(_) => println!("done"),
        Err(err) => {
            println!("fatal error: {}", err);
            exit(1);
        }
    }
}
