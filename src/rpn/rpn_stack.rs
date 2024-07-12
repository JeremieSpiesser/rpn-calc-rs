use super::{one_opt::OneOpt, opt::Opt, two_opt::TwoOpt, util_opt::UtilOpt};

pub struct RPNStack {
    stack: Vec<f64>,
}

impl RPNStack {
    pub fn new() -> RPNStack {
        RPNStack {
            stack: Vec::<f64>::new(),
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

    pub fn handle_opt(&mut self, opt: &Opt) {
        match opt {
            Opt::Number(f) => self.stack.push(f.clone()),
            Opt::OneOpt(oo) => match oo {
                OneOpt::Squared => {
                    if let Some(top) = self.stack.pop() {
                        self.stack.push(top * top);
                        self.print_top();
                    }
                }
                OneOpt::Cubed => {
                    if let Some(top) = self.stack.pop() {
                        self.stack.push(top * top * top);
                        self.print_top();
                    }
                }
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
                                TwoOpt::Plus => self.stack.push(n1 + n2),
                                TwoOpt::Minus => self.stack.push(n1 - n2),
                                TwoOpt::Times => self.stack.push(n1 * n2),
                                TwoOpt::Divided => {
                                    if n2 < 1e-10 {
                                        self.stack.push(n1 * n2);
                                    } else {
                                        self.stack.push(n1);
                                        self.stack.push(n2);
                                    }
                                }
                            }
                            self.print_top()
                        }
                    }
                }
            }
            Opt::UtilOpt(uo) => match uo {
                UtilOpt::Print => self.print_top(),
                UtilOpt::PrintAll => self.print_all(),
                UtilOpt::PrintHelp => {
                    println!("Here are the available commands : ");
                    println!("h : print this help menu");
                    println!("p : print the top of the stack");
                    println!("pa : print all the contents of the stack");
                }
            },
        }
    }
}
