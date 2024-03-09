/* By CKKitty on 2024-Mar-09
 * Processes command line input and delegates it to preproc for
 * preprocessing, then to grammar for grammaring.
 */

mod grammar;
mod preproc;
mod syllable;

use std::io::BufRead;

pub fn main() {
    let mut args = std::env::args();
    let mut input = None;

    // skip file name
    args.next();
    while let Some(cur_arg) = args.next() {
        match cur_arg.as_ref() {
            "-s" => {
                if let Some(input_string) = args.next() {
                    input = Some(input_string);
                }
                else {
                    println!("Error! Expected argument after -s (Input string)");
                    return;
                }
            }
            _ => {
                println!(concat!("Error! Unknown option: '{}' ",
                    "(Did you forget to quote the input string?)"), cur_arg);
                return;
            }
        }
    }

    // Decide between line-based mode and regular mode

    if let Some(input) = input {
        println!("{:?}", grammar::text_parser::text(input.as_ref()));
    }
    else {
        println!("Xexparser-rs Line Based Mode\nType 'quit' to quit\n");

        let stdin = std::io::stdin();

        loop {
            let mut line = String::new();
            stdin.lock().read_line(&mut line).unwrap();

            if line.trim() == "quit" {
                return;
            }
            else {
                let parsed = grammar::text_parser::text(line.trim());

                if parsed.is_ok() {
                    println!("{:?}\n", parsed.unwrap());
                }
                else {
                    println!("ERROR: {:?}\n", parsed.unwrap_err());
                }
            }
        }
    }
}