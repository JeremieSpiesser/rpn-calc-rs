use std::str::FromStr;

#[derive(Debug)]
pub enum UtilOpt {
    Print,
    PrintAll,
    PrintHelp,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseUtilOptErr;

impl FromStr for UtilOpt {
    type Err = ParseUtilOptErr;
    fn from_str(input: &str) -> Result<UtilOpt, Self::Err> {
        match input.trim() {
            "p" => Ok(UtilOpt::Print),
            "pa" => Ok(UtilOpt::PrintAll),
            "h" => Ok(UtilOpt::PrintHelp),
            _ => Err(ParseUtilOptErr),
        }
    }
}
