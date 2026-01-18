//! Math utilities library.
//!
//! Public API is re-exported at the crate root, e.g. `math_utils::add(2, 3)`.

mod math; // internal module (not exposed as math_utils::math)

// Re-export the functions you want at the top level:
pub use math::basic::{add, sub, mul, div};
pub use math::advanced::{factorial, gcd, is_prime};
