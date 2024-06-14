use std::io;
use std::io::Write;
use std::str::FromStr;

#[allow(dead_code)]

#[derive(Debug)]
enum Opt {
    Number(f64),
    OneOpt(OneOpt),
    TwoOpt(TwoOpt),
    UtilOpt(UtilOpt),
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

#[derive(Debug)]
enum UtilOpt{
    Print,
    PrintAll,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseUtilOptErr;

impl FromStr for UtilOpt{
    type Err = ParseUtilOptErr; 
    fn from_str(input: &str) -> Result<UtilOpt, Self::Err> {
        match input.trim() {
            "p" => Ok(UtilOpt::Print),
            "pa" => Ok(UtilOpt::PrintAll),
            _ => Err(ParseUtilOptErr),
        }
    }
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
        if let Ok(uo) = UtilOpt::from_str(input.trim()) {
            return Ok(Opt::UtilOpt(uo));
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
    stack : &'a mut Vec<f64>
}

impl<'a> RPNStack<'a> {

    fn new(stack : &'a mut Vec<f64> ) -> RPNStack<'a> {
        RPNStack{
            stack: stack
        }
    }

    fn print_top(&self) {
        if let Some(top) = self.stack.last() {
            println!("{}", top);
        }
    }

    fn print_all(&self) {
        for number in self.stack.iter().rev() {
            println!("{}", number);
        }
    }

    fn handle_opt(&mut self, opt : &Opt) {
        match opt {
            Opt::Number(f) => self.stack.push(f.clone()),
            Opt::OneOpt(oo) => match oo {
                OneOpt::Squared => {
                    if let Some(top) = self.stack.pop() {
                        self.stack.push(top * top);
                        self.print_top();
                    }
                },
                OneOpt::Cubed => {
                    if let Some(top) = self.stack.pop() {
                        self.stack.push(top * top * top);
                        self.print_top();
                    }
                },
                OneOpt::Sqrt => {
                    if let Some(top) = self.stack.pop() {
                        self.stack.push(top.sqrt());
                        self.print_top();
                    }
                }
            },
            Opt::TwoOpt(to) => {
                if self.stack.len() >= 2 {
                    if let Some(n2) = self.stack.pop() {
                        if let Some(n1) = self.stack.pop() {
                            match to {
                                TwoOpt::Plus => self.stack.push( n1 + n2),
                                TwoOpt::Minus => self.stack.push( n1 - n2),
                                TwoOpt::Times => self.stack.push( n1 * n2),
                                TwoOpt::Divided => if n2 < 1e-10 {
                                    self.stack.push( n1 * n2);
                                }
                                else{
                                    self.stack.push(n1);
                                    self.stack.push(n2);
                                }
                            }
                            self.print_top()
                        }
                    }
                }
            },
            Opt::UtilOpt(uo) => {
                match uo {
                    UtilOpt::Print => self.print_top(),
                    UtilOpt::PrintAll => self.print_all(),
                }
            }
        }
    }
}


fn main() {
    let mut stack = Box::new(Vec::<f64>::new());
    let mut rpnstack = RPNStack::new(&mut stack);

    println!("Welcome to your new RPN calculator written in Rust !");

    loop {
        let mut user_line = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_line).expect("Failed to read user input");

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

