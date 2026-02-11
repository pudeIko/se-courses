//! Basic math operations.

/// Adds two `i64` values.
///
/// # Examples
/// ```
/// use math_utils::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

/// Subtracts the second `i64` value from the first.
/// # Examples
/// ```
/// use math_utils::sub;
/// assert_eq!(sub(5, 3), 2);
/// ```
pub fn sub(a: i64, b: i64) -> i64 {
    return a - b;
}

/// Multiplies two `i64` values.
/// # Examples
/// ```
/// use math_utils::mul;
/// assert_eq!(mul(2, 3), 6);
/// ```
pub fn mul(a: i64, b: i64) -> i64 {
    return a * b;
}

/// Divides the first `i64` value by the second.
/// # Examples
/// ```
/// use math_utils::div;
/// assert_eq!(div(6, 3), Some(2));
/// assert_eq!(div(5, 0), None);
/// ```
pub fn div(a: i64, b: i64) -> Option<i64> {
    if b == 0 {
        print!("\tDivision by zero, returning None\n");
        return None;
    } else {
        return Some(a / b); 
    }
}


// Configuration for tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5); 
        assert_eq!(add(-2, 3), 1);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(10, 4), 6);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(6, 7), 42);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(8, 2), Some(4));
        assert_eq!(div(8, 0), None);
    }
}