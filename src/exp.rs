use num_traits::Num;
use std::fmt::Display;

/// 累乗を表す構造体
pub struct Exponentiation<T>
where
    T: Num + Copy + Display,
{
    base: T,
    exponent: u32,
}

impl<T> Exponentiation<T> where T: Num + Copy + Display {
    pub fn new(base: T, exponent: u32) -> Self {
        Exponentiation {
            base,
            exponent,
        }
    }

    /// 冪乗を計算した結果の自然数を返却します。
    pub fn calc_result(&self) -> T {
        power(self.base, self.exponent)
    }
}

impl<T> std::fmt::Display for Exponentiation<T>
where
    T: Num + PartialOrd + Copy + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}^{}", self.base, self.exponent)
    }
}

/// 引数の二乗を返却します。
///
///
pub fn squared<T>(base: T) -> T
where
    T: Num + Copy,
{
    base * base
}

/// 引数の三乗を返却します。
///
///
pub fn cubed<T>(base: T) -> T
where
    T: Num + Copy,
{
    squared(base) * base
}

/// 引数`num`の`index`乗した値を返却します。
///
pub fn power<T>(base: T, index: u32) -> T
where
    T: Num + Copy,
{
    match index {
        i if i == 1 => base,
        i if i % 2 == 0 => squared(power(base, i / 2)),
        i if i % 2 == 1 => squared(power(base, i / 2)) * base,
        _ => base,
    }
}

#[cfg(test)]
mod exponentiation_test {
    use super::*;

    #[test]
    fn display() {
        let exp = Exponentiation {
            base: 3,
            exponent: 4,
        };
        let text = format!("{}", exp);
        assert_eq!(text, "3^4");
    }


    #[test]
    fn calc_result() {
        let exp = Exponentiation {
            base: 2,
            exponent: 4,
        };
        assert_eq!(exp.calc_result(), 16);
        let exp = Exponentiation {
            base: 2,
            exponent: 10,
        };
        assert_eq!(exp.calc_result(), 1024)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squared_test() {
        assert_eq!(squared(3), 9);
    }

    #[test]
    fn cubed_test() {
        assert_eq!(cubed(2), 8);
    }

    #[test]
    fn power_test() {
        assert_eq!(power(2, 4), 16);
        assert_eq!(power(2, 10), 1024);
        assert_eq!(power(2, 24), 16777216);
    }
}
