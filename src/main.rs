use std::env;

fn main() {
    println!("Hello, world!");


    let enable_fib = env::var("INPUT_ENABLE_FIB").expect("Environment variable INPUT_ENABLE_FIB is not set");
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").expect("Environment variable INPUT_MAX_THRESHOLD is not set")
        .parse::<u32>().expect("INPUT_MAX_THRESHOLD is not a valid non-negative integer");

    println!("Enable Fibonacci: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    if enable_fib == "true" {
        let pr_content = "This PR fixes issue 123 and updates 456"; // Simulated PR content
        let numbers = extract_numbers(pr_content);

        for num in numbers {
            if num <= max_threshold {
                println!("Fibonacci({}): {}", num, fibonacci(num));
            }
        }
    }
}

fn extract_numbers(text: &str) -> Vec<u32> {
    text.split_whitespace()
        .filter_map(|word| word.parse::<u32>().ok())
        .collect()
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers() {
        assert_eq!(extract_numbers("This PR fixes issue 123 and updates 456"), vec![123, 456]);
        assert_eq!(extract_numbers("No numbers here"), Vec::<u32>::new());
        assert_eq!(extract_numbers("123, 456, 789"), vec![123, 456, 789]);
        assert_eq!(extract_numbers("123.456 789"), vec![123, 456, 789]);
        assert_eq!(extract_numbers("123 456 789"), vec![123, 456, 789]);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }
}
