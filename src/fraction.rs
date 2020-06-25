use super::*;
use num_traits::Num;
use std::fmt::Display;

pub struct Fraction<T>
where
    T: Num + Copy + Display,
{
    /// 分子
    numerator: T,
    /// 分母
    denominator: T,
}

impl<T> Fraction<T> 
where T: Num + PartialOrd + Copy + Display
{
    pub fn new(numerator: T, denominator: T) -> Self 
    where T: Num
    {
        Fraction {
            numerator,
            denominator,
        }
    }

    /// 約分して初期化します。
    pub fn new_reduction(numerator: T, denominator: T) -> Self {
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

impl<T> std::fmt::Display for Fraction<T> 
where T: Num + PartialOrd + Copy + std::fmt::Display
{
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
