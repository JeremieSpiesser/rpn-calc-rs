use std::str::FromStr;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::rpn::one_opt::{OneOpt, ParseOneOptErr};

    #[test]
    fn test_from_str_squared_ok() {
        let s1 = "**2";
        assert_eq!(OneOpt::from_str(s1).unwrap(), OneOpt::Squared);
        let s1 = "  **2  ";
        assert_eq!(OneOpt::from_str(s1).unwrap(), OneOpt::Squared);
    }

    #[test]
    fn test_from_str_cube_ok() {
        let s1 = "**3";
        assert_eq!(OneOpt::from_str(s1).unwrap(), OneOpt::Cubed);
        let s1 = " **3   ";
        assert_eq!(OneOpt::from_str(s1).unwrap(), OneOpt::Cubed);
    }

    #[test]
    fn test_from_str_sqrt_ok() {
        let s1 = "sqrt  ";
        assert_eq!(OneOpt::from_str(s1).unwrap(), OneOpt::Sqrt);
        let s1 = " sqrt  ";
        assert_eq!(OneOpt::from_str(s1).unwrap(), OneOpt::Sqrt);
    }

    #[test]
    fn test_from_str_fails() {
        let v = vec!["a**2", "sqrat", "**2 **3", "sqrt(2)"];
        for t in v {
            assert_eq!(OneOpt::from_str(t).err().unwrap(), ParseOneOptErr);
        }
    }
}
