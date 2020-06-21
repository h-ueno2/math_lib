pub mod exp;
pub mod fraction;

/// 引数`a`と`b`の最大公約数を返却します。
/// 
pub fn gratest_common_factor(a: u32, b: u32) -> u32 {
    let mut big_num = if a >= b { a } else { b };
    let mut small_num = if a >= b { b } else { a };
    let mut remainder = big_num % small_num;
    while remainder != 0 {
        big_num = small_num;
        small_num = remainder;
        remainder = big_num % small_num;
    }
    small_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gratest_common_factor_test() {
        let a = 3355;
        let b = 2379;

        assert_eq!(gratest_common_factor(a, b), 61);
    }
}
