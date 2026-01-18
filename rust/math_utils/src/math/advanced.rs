//! Advanced math utilities.

/// Computes `n!` (factorial) for `n >= 0`.
///
/// Returns `None` if:
/// - `n` is negative
/// - the result overflows `u128`
///
/// # Examples
/// ```
/// use math_utils::factorial;
/// assert_eq!(factorial(5), Some(120));
/// assert_eq!(factorial(-1), None);
/// ```
pub fn factorial(n: i64) -> Option<u128> {
    if n < 0 {
        return None;
    }
    let mut acc: u128 = 1;
    for k in 2..=(n as u128) {
        acc = acc.checked_mul(k)?;
    }
    return Some(acc);
}

/// Computes the greatest common divisor (GCD) of two integers using Euclid's algorithm.
///
/// The result is always non-negative. `gcd(0, 0) == 0`.
///
/// # Examples
/// ```
/// use math_utils::gcd;
/// assert_eq!(gcd(54, 24), 6);
/// assert_eq!(gcd(-54, 24), 6);
/// ```
pub fn gcd(a: i64, b: i64) -> i64 {
    let mut x = a.abs();
    let mut y = b.abs();

    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    return x;
}

/// Checks whether `n` is a prime number.
///
/// Returns `false` for `n < 2`.
///
/// # Examples
/// ```
/// use math_utils::is_prime;
/// assert!(is_prime(2));
/// assert!(is_prime(13));
/// assert!(!is_prime(1));
/// assert!(!is_prime(15));
/// ```
pub fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // trial division up to sqrt(n)
    let mut d: i64 = 3;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), Some(1));
        assert_eq!(factorial(1), Some(1));
        assert_eq!(factorial(5), Some(120));
        assert_eq!(factorial(-1), None);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(-54, 24), 6);
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 7), 7);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(-7));
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(13));
        assert!(!is_prime(15));
    }
}
