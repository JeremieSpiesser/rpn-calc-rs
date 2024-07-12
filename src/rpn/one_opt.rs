use std::str::FromStr;

#[derive(Debug)]
pub enum OneOpt {
    Squared,
    Cubed,
    Sqrt,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseOneOptErr;

impl FromStr for OneOpt {
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
