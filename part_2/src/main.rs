use std::fs;
use std::io;

fn main() -> io::Result<()>{
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)?;

    let mut invalid_ids: i32 = 0;
    let mut sum_inv_ids: u128 = 0;

    for range_item in contents.split(",") {
        let mut num_it = range_item.split("-");
        let first: u64 = num_it.next().unwrap_or("0").parse().unwrap_or(0);
        let last: u64 = num_it.next().unwrap_or("0").parse().unwrap_or(0);

        for num in first..=last {
            // Test number for repeats. May be easier as a string tbh.
            let num_str: String = num.to_string();
            let len = num_str.len();
            
            if len % 2 != 0 {
                continue;
            }
            
            let (prefix, suffix) = num_str.split_at(len / 2);            
            if prefix == suffix {
                invalid_ids += 1;
                sum_inv_ids += num as u128;

                // println!("FOUND {}, {}, {}", num, first, last);
            }
        }
    }

    println!("Num thangs {}, sum: {}", invalid_ids, sum_inv_ids);
    Ok(())
}
