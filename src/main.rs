use std::env;

fn main() {
    let enable_fib = env::var("INPUT_ENABLE_FIB").expect("Environment variable INPUT_ENABLE_FIB is not set");
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").expect("Environment variable INPUT_MAX_THRESHOLD is not set")
        .parse::<u32>().expect("INPUT_MAX_THRESHOLD is not a valid non-negative integer");

    println!("Enable Fibonacci: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    if enable_fib == "true" {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            eprintln!("Usage: fibbot <number>");
            return;
        }

        let n: u32 = match args[1].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Error: Please provide a valid non-negative integer.");
                return;
            }
        };

        if n > max_threshold {
            eprintln!("Error: Number exceeds the maximum threshold of {}.", max_threshold);
            return;
        }

        println!("Fibonacci of {} is {}", n, fibonacci(n));
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
         fn test_fibonacci() {
             assert_eq!(fibonacci(0), 0);
             assert_eq!(fibonacci(1), 1);
             assert_eq!(fibonacci(10), 55);
         }
     }