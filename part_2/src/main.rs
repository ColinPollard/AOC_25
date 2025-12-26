use std::fs;
use std::io;
use itertools::{Itertools, Position};

fn main() -> io::Result<()>{
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)?;

    let mut invalid_ids: i32 = 0;
    let mut sum_inv_ids: u128 = 0;

    for range_item in contents.split(",") {
        let mut num_it = range_item.split("-");
        let first: u128 = num_it.next().unwrap_or("0").parse().unwrap_or(0);
        let last: u128 = num_it.next().unwrap_or("0").parse().unwrap_or(0);
        
        'num_loop: for num in first..=last {
            // Test number for repeats. May be easier as a string tbh.
            let num_str: String  = num.to_string();
            let len = num_str.len();
            // println!("---------- INPUT: {}", num_str);

            for i in 1..len {
                // take 0..i segment
                let test_str: &str = &num_str[0..i];

                // Check if len(string) % len(0..i) == 0 -> repeats possible
                if len % test_str.len() == 0 {
                    // Iterate 
                    for (pos, element) in num_str.chars().chunks(test_str.len()).into_iter().with_position() {
                        match pos {
                            Position::Last | Position::Only => {
                                let result: String = element.collect();
                                if result == test_str {
                                    invalid_ids += 1;
                                    sum_inv_ids += num as u128;

                                    continue 'num_loop;
                                }
                            }
                            Position::First | Position::Middle => {
                                let result: String = element.collect();
                                if result != test_str {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Num thangs {}, sum: {}", invalid_ids, sum_inv_ids);
    Ok(())
}