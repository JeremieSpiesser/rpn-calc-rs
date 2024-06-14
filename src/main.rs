use std::io;
use std::io::Write;
use std::str::FromStr;

#[allow(dead_code)]

#[derive(Debug)]
enum Opt {
    Number(f64),
    OneOpt(OneOpt),
    TwoOpt(TwoOpt),
}

#[derive(Debug)]
enum TwoOpt{
    Plus,
    Minus,
    Times,
    Divided,
}

#[derive(Debug)]
enum OneOpt{
    Squared,
    Cubed,
    Sqrt,
}
#[derive(Debug, PartialEq, Eq)]
struct ParseOneOptErr;
impl FromStr for OneOpt{
    type Err = ParseOneOptErr; 
    fn from_str(input: &str) -> Result<OneOpt, Self::Err> {
        match input.trim() {
            "**2" => Ok(OneOpt::Squared),
            "**3" => Ok(OneOpt::Cubed),
            "sqrt" => Ok(OneOpt::Sqrt),
            _ => Err(ParseOneOptErr),
        }
    }
}

#[derive(Debug, PartialEq, Eq)] struct ParseOptErr;
impl FromStr for Opt {
    type Err = ParseOptErr; 

    fn from_str(input: &str) -> Result<Opt, Self::Err> {
        if let Ok(f) = input.trim().parse::<f64>() {
            return Ok(Opt::Number(f));
        }
        if let Ok(oo) = OneOpt::from_str(input.trim()) {
            return Ok(Opt::OneOpt(oo));
        }
        if let Ok(to) = TwoOpt::from_str(input.trim()) {
            return Ok(Opt::TwoOpt(to));
        }
        Err(ParseOptErr)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseTwoOptErr;
impl FromStr for TwoOpt{
    type Err = ParseTwoOptErr; 
    fn from_str(input: &str) -> Result<TwoOpt, Self::Err> {
        match input.trim() {
            "+" => Ok(TwoOpt::Plus),
            "-" => Ok(TwoOpt::Minus),
            "*" => Ok(TwoOpt::Times),
            "/" => Ok(TwoOpt::Divided),
            _ => Err(ParseTwoOptErr),
        }
    }
}


struct RPNStack<'a> {
    stack : &'a mut Box<Vec<f64>>
}

impl<'a> RPNStack<'a> {

    fn new() {
        RPNStack{
            stack: &mut Box::new(Vec::<f64>::new())
        }
    }

    fn print(&self) {
        dbg!(self.stack);
    }

    fn handleOpt(&mut self, opt : &Opt) {
        match opt {
            Opt::Number(f) => self.stack.push(f.clone()),
            Opt::OneOpt(oo) => match oo {
                OneOpt::Squared => {
                    if let Some(top) = self.stack.pop() {
                        self.stack.push(top * top);
                    }
                },
                _ => todo!("not implemented yet"),
            }
            _ => todo!("implémenter tt les opt"),
        }
    }
}


fn main() {

    let mut stack = Box::new(Vec::<f64>::new());
    println!("Welcome to your new RPN calculator written in Rust !");

    stack.push(4.2);

    dbg!(stack.last());

    loop {
        let mut user_line = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_line).expect("Failed to read user input");
        dbg!(&user_line);

        for token in user_line.trim().split_whitespace() {
            match Opt::from_str(token) {
                Ok(opt) => match opt {
                    Opt::Number(n) => println!("Got number {} ", n),
                    Opt::OneOpt(to) => {dbg!(to) ; stack ;   () },
                    Opt::TwoOpt(to) => {dbg!(to) ; () },
                },
                Err(_) => println!("can't parse as opt"),
            }

        }



    }

}

