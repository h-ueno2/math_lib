use super::*;

pub struct Fraction {
    /// 分子
    numerator: u32,
    /// 分母
    denominator: u32,
}

impl Fraction {
    pub fn new(numerator: u32, denominator: u32) -> Self {
        Fraction {
            numerator,
            denominator,
        }
    }

    /// 約分して初期化します。
    pub fn new_reduction(numerator: u32, denominator: u32) -> Self {
        let common_factor = gratest_common_factor(numerator, denominator);
        Fraction {
            numerator: numerator / common_factor,
            denominator: denominator / common_factor,
        }
    }

    /// 約分します
    pub fn reduction(&mut self) {
        let common_factor = gratest_common_factor(self.numerator, self.denominator);
        self.numerator = self.numerator / common_factor;
        self.denominator = self.denominator / common_factor;
    }
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

    #[test]
    fn new_reduction_test() {
        let fraction = Fraction::new_reduction(5, 10);

        let text = format!("{}", fraction);
        assert_eq!(text, "1/2");
    }

    #[test]
    fn reduction_test() {
        let mut fraction = Fraction::new(5, 10);
        fraction.reduction();
        let text = format!("{}", fraction);
        assert_eq!(text, "1/2");
    }
}
