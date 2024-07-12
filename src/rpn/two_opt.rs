use std::str::FromStr;

#[derive(Debug)]
pub enum TwoOpt {
    Plus,
    Minus,
    Times,
    Divided,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTwoOptErr;

impl FromStr for TwoOpt {
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
