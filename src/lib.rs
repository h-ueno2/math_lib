pub mod exp;
pub mod fraction;

/// 引数`a`と`b`の最大公約数を返却します。
/// 
pub fn gratest_common_factor(a: u32, b: u32) -> u32 {
    let big_num = if a >= b { a } else { b };
    let small_num = if a >= b { b } else { a };
    match big_num % small_num {
        i if i != 0 => gratest_common_factor(small_num, i),
        _ => small_num,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gratest_common_factor_test01() {
        assert_eq!(gratest_common_factor(3355, 2379), 61);
    }
    #[test]
    fn gratest_common_factor_test02() {
        assert_eq!(gratest_common_factor(2379, 3355), 61);
    }
}
