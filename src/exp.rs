use std::ops::Mul;

/// 引数の二乗を返却します。
///
///
pub fn squared<T>(num: T) -> T
where
    T: Mul + Mul<Output = T> + Copy,
{
    num * num
}

/// 引数の三乗を返却します。
///
///
pub fn cubed<T>(num: T) -> T
where
    T: Mul + Mul<Output = T> + Copy,
{
    squared(num) * num
}

/// 引数`num`の`index`乗した値を返却します。
///
pub fn power<T>(num: T, index: u32) -> T
where
    T: Mul + Mul<Output = T> + Copy,
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
