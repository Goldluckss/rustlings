fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for &element in input {
        // Multiply each element in the `input` slice by 2 and push it to the `output` vector.
        output.push(element * 2);
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // An example of collecting a vector after mapping.
    // We map each element of the `input` slice to its value plus 1.
    // If the input is `[1, 2, 3]`, the output is `[2, 3, 4]`.
    input.iter().map(|&element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // Multiply each element in the `input` slice by 2 using iterator mapping.
    input.iter().map(|&element| element * 2).collect()
}

fn main() {
    let input = [1, 2, 3];
    let result = vec_loop(&input);
    println!("vec_loop result: {:?}", result);

    let result = vec_map(&input);
    println!("vec_map result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [1, 2, 3];
        let ans = vec_loop(&input);
        assert_eq!(ans, [2, 4, 6]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [1, 2, 3];
        let ans = vec_map(&input);
        assert_eq!(ans, [2, 4, 6]);
    }
}
