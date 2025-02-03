pub mod q64x96;
pub mod tick_math;
pub mod liquidity;
pub mod fees;
pub mod sqrt_price;

// Re-export commonly used items
pub use q64x96::Q64x96;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
