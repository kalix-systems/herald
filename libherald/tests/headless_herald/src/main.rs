extern crate clap;
extern crate heraldcore;
extern crate serde_json;
pub mod parser_types;

use clap::{App, Arg};
use parser_types::*;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::process::exit;

fn main() {
    let matches = App::new("headless_herald")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("json_file")
                .help("Sets json file with configuration and actions")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("UserId")
                .short("u")
                .long("userid")
                .value_name("userid")
                .help("sets the UserId from which to issue the actions")
                .takes_value(true),
        )
        .get_matches();

    let userid = matches.value_of("userid");

    let json_file = matches
        .value_of("json_file")
        .expect("Json input failed somehow?");
    let script = if let Ok(mut f) = File::open(json_file) {
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).expect("could not read file");
        ScriptFile::new(buf)
    } else {
        eprintln!("error: file, {} not found", &json_file);
        std::process::exit(-1);
    };

    // go through top level config
    if let Some(userid) = userid {
        login_and_register(userid);
    } else {
        let userid = script
            .userid
            .expect("no userid included in json file, nor specified on the command line.");
        login_and_register(&userid);
    }

    // loop through actions in json
    script
        .actions
        .iter()
        .enumerate()
        .for_each(|(num, instruction)| match &instruction.action_type {
            Some(action) => match action.as_ref() {
                "Send" => {}
                "Await" => {}
                _ => {
                    eprintln!(
                        "instruction {} had an unknown action_type field, aborting",
                        num + 1
                    );
                    exit(i32::try_from(num + 1).unwrap());
                }
            },
            None => {
                eprintln!("instruction {} had no action_type field, aborting", num + 1);
                exit(i32::try_from(num + 1).unwrap());
            }
        });
    exit(0);
}

// helper functions

// logs in and registers user
fn login_and_register(userid: &str) {}

// send a message, or die trying

// await signal, or timeout.
