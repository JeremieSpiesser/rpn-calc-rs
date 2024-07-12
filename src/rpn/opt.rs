use std::str::FromStr;

use super::{one_opt::OneOpt, two_opt::TwoOpt, util_opt::UtilOpt};

#[derive(Debug)]
pub enum Opt {
    Number(f64),
    OneOpt(OneOpt),
    TwoOpt(TwoOpt),
    UtilOpt(UtilOpt),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseOptErr;

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
