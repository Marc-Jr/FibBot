    use std::env;

     fn main() {
         let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap();
         let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap().parse::<u32>().unwrap();

         println!("Enable Fibonacci: {}", enable_fib);
         println!("Max Threshold: {}", max_threshold);

         fn extract_numbers(text: &str) -> Vec<u32> {
            text.split_whitespace()
                .filter_map(|word| word.parse::<u32>().ok())
                .collect()
        }

     }

     

