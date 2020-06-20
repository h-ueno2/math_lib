use std::ops::Mul;

/// 引数の二乗を返却します。
/// 
/// 
pub fn squared<T: Mul + Mul<Output = T> + Copy>(num: T) -> T {
    num * num
}

/// 引数の三乗を返却します。
/// 
/// 
pub fn cubed<T: Mul + Mul<Output = T> + Copy>(num: T) -> T {
    squared(num) * num
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
}
