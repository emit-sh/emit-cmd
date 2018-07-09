extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Emit")
        .version("0.1")
        .author("Marek Counts <countsmarek@gmail.com>")
        .about("Command line untility to assist with using the emit.sh service")
        .arg(
            Arg::with_name("input")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("email")
                .required(false)
                .help("Ends the emited link to the above email")
                .index(2),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

        if let Some(i) = matches.value_of("input") {
            println!("File to be saved: {}", i);
        }

        if let Some(e) = matches.value_of("email") {
            println!("Email address to send to: {}", e);
        }
}
