// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
    let numbers = [1, 2, 3, 4, 5, 6];
    for &number in &numbers {
        println!("Is {} even? {}", number, is_even(number));
    }
}

#[cfg(test)]
mod tests {
    // Import everything from the outer module
    use super::*;

    #[test]
    fn you_can_assert() {
        // Test the function `is_even` with some values.
        assert!(is_even(2));
        assert!(!is_even(3));
        assert!(is_even(0));
        assert!(!is_even(-1));
        assert!(is_even(-2));
    }
}
