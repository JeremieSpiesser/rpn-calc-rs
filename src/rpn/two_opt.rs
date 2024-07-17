use std::str::FromStr;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::rpn::two_opt::{ParseTwoOptErr, TwoOpt};

    #[test]
    fn test_from_str_plus_ok() {
        let s1 = "+";
        assert_eq!(TwoOpt::from_str(s1).unwrap(), TwoOpt::Plus);
        let s1 = "  +  ";
        assert_eq!(TwoOpt::from_str(s1).unwrap(), TwoOpt::Plus);
    }

    #[test]
    fn test_from_str_minus_ok() {
        let s1 = "-";
        assert_eq!(TwoOpt::from_str(s1).unwrap(), TwoOpt::Minus);
        let s1 = " -   ";
        assert_eq!(TwoOpt::from_str(s1).unwrap(), TwoOpt::Minus);
    }

    #[test]
    fn test_from_str_times_ok() {
        let s1 = "*  ";
        assert_eq!(TwoOpt::from_str(s1).unwrap(), TwoOpt::Times);
        let s1 = " *  ";
        assert_eq!(TwoOpt::from_str(s1).unwrap(), TwoOpt::Times);
    }

    #[test]
    fn test_from_str_divided_ok() {
        let s1 = "/  ";
        assert_eq!(TwoOpt::from_str(s1).unwrap(), TwoOpt::Divided);
        let s1 = " /  ";
        assert_eq!(TwoOpt::from_str(s1).unwrap(), TwoOpt::Divided);
    }

    #[test]
    fn test_from_str_fails() {
        let v = vec!["1+1", "2-3", "a*b", "**", "qlkjsdfh", "2 / 4"];
        for t in v {
            assert_eq!(TwoOpt::from_str(t).err().unwrap(), ParseTwoOptErr);
        }
    }
}
