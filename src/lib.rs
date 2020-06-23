extern crate num_traits;

pub mod exp;
pub mod fraction;

use num_traits::Zero;
use std::cmp::PartialOrd;
use std::ops::Rem;

/// 引数`a`と`b`の最大公約数を返却します。
/// 
pub fn gratest_common_factor<T>(a: T, b: T) -> T 
where T: Zero + PartialOrd + Rem<Output = T> + Copy
{
    let big_num = if a >= b { a } else { b };
    let small_num = if a >= b { b } else { a };
    match big_num % small_num {
        i if i != T::zero() => gratest_common_factor(small_num, i),
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

    #[test]
    fn gratest_common_factor_test03() {
        let a : u128 = 2379;
        let b : u128 = 3355;
        assert_eq!(gratest_common_factor(a, b), 61);
    }
}
