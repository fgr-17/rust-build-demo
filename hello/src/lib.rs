//! # MyCrate
//!
//! Example of lib doing basic stuff and its tests
//!

/// Simple example trait to add two numbers
#[cfg_attr(test, mockall::automock)]
pub trait Summable {
    fn sum(&self, a: i32, b: i32) -> i32;
}

pub struct RealSum;

/// Implementation of trait Summable on RealSum struc
/// ## Example
/// ```rust
/// use hello::Summable;
/// use hello::RealSum;
/// let rs = RealSum;
/// assert_eq!(rs.sum(10, 5), 15);
/// ```
impl Summable for RealSum {
    fn sum(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

pub fn sum_vec<C: Summable>(vec: Vec<i32>, calc: &C) -> i32 {
    let mut acc = 0;
    for item in vec.iter() {
        acc = calc.sum(acc, *item);
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_positive_numbers() {
        let rs = RealSum;
        assert_eq!(rs.sum(10, 5), 15);
    }

    #[test]
    fn test_sum_vec_mocked() {
        let mut mock_sum = MockSummable::new();

        mock_sum.expect_sum().times(3).returning(|a, b| a + b);

        let res = sum_vec(vec![1, 2, 3], &mock_sum);
        assert_eq!(res, 6);
    }
}
