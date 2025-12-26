use std::fs;
use std::io;

fn find_first_largest_instance<T: Ord + Copy>(slice: &[T]) -> Option<(usize, T)> {
    let len = slice.len();
    slice
        .iter()
        // Reverse iteration to make the 'rightmost' default behavior of max_by_key 
        // find the first occurrence from the original start
        .rev() 
        .enumerate()
        .max_by_key(|&(_, &value)| value)
        .map(|(rev_idx, &value)| {
            // Calculate the original index
            let original_idx = len - 1 - rev_idx;
            (original_idx, value)
        })
}

fn main() -> io::Result<()>{
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)?;

    let mut total_joltage: u128 = 0;

    for bank in contents.lines() {
        // Create a vector of ints from the bank.
        let nums: Vec<i32> = bank.chars()
            .map(|c| c.to_string().parse::<i32>().unwrap()).collect();

        let mut cells: Vec<i32> = Vec::new();
        let mut last_idx = 0;

        for i in (0..12).rev() {
            // println!("i:{}, last_idx:{}", i, last_idx);
            let (idx, val) = find_first_largest_instance(&nums[last_idx..nums.len()-i]).expect("whatever");
            last_idx = idx + 1 + last_idx;
            cells.push(val);
        }
        
        let bank_joltage_str: String = cells.iter().map(|num| num.to_string()).collect();
        total_joltage += bank_joltage_str.parse::<u128>().expect("oh well");
    }

    println!("{}", total_joltage);
    Ok(())
}
