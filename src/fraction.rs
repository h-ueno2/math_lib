struct Fraction {
    /// 分子
    numerator: u32,
    /// 分母
    denominator: u32,
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let fraction = Fraction {
            numerator: 3,
            denominator: 4,
        };
        let text = format!("{}", fraction);
        assert_eq!(text, "3/4");
    }
}
