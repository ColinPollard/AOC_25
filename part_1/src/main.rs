use std::fs;
use std::io;

fn main() -> io::Result<()>{
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)?;

    let mut lock_pos: i32 = 50;
    let mut num_zeros: i32 = 0;

    for line in contents.lines() {
        let prefix: &str = &line[0..1];
        let suffix: &str = &line[1..];

        let num: i32 = suffix.parse().unwrap_or(0);
        if prefix == "R" {  
            lock_pos += num;

            if lock_pos > 99 {
                lock_pos -= (lock_pos / 100) * 100
            }
        }
        else {
            lock_pos -= num;

            if lock_pos < 0 {
                lock_pos -= (lock_pos / 100) * 100
            }
        }

        if lock_pos == 0 {
            num_zeros += 1;
        }
    }

    println!("Num zeros: {}, lock_pos: {}", num_zeros, lock_pos);
    Ok(())
}
