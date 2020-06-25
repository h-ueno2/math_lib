use num_traits::Num;
use std::fmt::Display;

/// 累乗を表す構造体
struct Exponentiation<T>
where
    T: Num + Copy + Display,
{
    base: T,
    exponent: T,
}

impl<T> std::fmt::Display for Exponentiation<T> 
where T: Num + PartialOrd + Copy + Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}^{}", self.base, self.exponent)
    }
}

/// 引数の二乗を返却します。
///
///
pub fn squared<T>(num: T) -> T
where
    T: Num + Copy,
{
    num * num
}

/// 引数の三乗を返却します。
///
///
pub fn cubed<T>(num: T) -> T
where
    T: Num + Copy,
{
    squared(num) * num
}

/// 引数`num`の`index`乗した値を返却します。
///
pub fn power<T>(num: T, index: u32) -> T
where
    T: Num + Copy,
{
    match index {
        i if i == 1 => num,
        i if i % 2 == 0 => squared(power(num, i / 2)),
        i if i % 2 == 1 => squared(power(num, i / 2)) * num,
        _ => num,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let fraction = Exponentiation {
            base: 3,
            exponent: 4,
        };
        let text = format!("{}", fraction);
        assert_eq!(text, "3^4");
    }

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
