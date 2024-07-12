use std::io;
use std::io::Write;
use std::str::FromStr;

use rpn::opt::Opt;
mod rpn;

fn main() {
    let mut rpnstack = rpn::rpn_stack::RPNStack::new();

    println!("Welcome to your RPN calculator written in Rust !");

    loop {
        let mut user_line = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut user_line)
            .expect("Failed to read user input");

        for token in user_line.trim().split_whitespace() {
            match Opt::from_str(token) {
                Ok(opt) => {
                    rpnstack.handle_opt(&opt);
                }
                Err(_) => println!("can't parse as opt"),
            }
        }
    }
}
