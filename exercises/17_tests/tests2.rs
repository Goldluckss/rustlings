// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
#[allow(dead_code)]
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
    let exponents = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    for &exp in &exponents {
        println!("2^{} = {}", exp, power_of_2(exp));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // Test the function `power_of_2` with some values.
        assert_eq!(power_of_2(0), 1);
        assert_eq!(power_of_2(1), 2);
        assert_eq!(power_of_2(2), 4);
        assert_eq!(power_of_2(3), 8);
        assert_eq!(power_of_2(4), 16);
        assert_eq!(power_of_2(5), 32);
        assert_eq!(power_of_2(6), 64);
        assert_eq!(power_of_2(7), 128);
        assert_eq!(power_of_2(8), 256);
    }
}
