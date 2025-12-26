use std::fs;
use std::io;

fn main() -> io::Result<()>{
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)?;

    let mut lock_pos: i32 = 50;
    let mut num_zeros: i128 = 0;

    for line in contents.lines() {
        let prefix: &str = &line[0..1];
        let suffix: &str = &line[1..];
        println!("{}, {}, {}", lock_pos, prefix, suffix);

        let num: i32 = suffix.parse().unwrap_or(0);

        let dumb_flag: bool = lock_pos == 0;

        if prefix == "R" {  

            // double counting if it starts at 0 and moves...
            lock_pos += num;

            if lock_pos > 99 {
                let rotations = lock_pos / 100;
                num_zeros += rotations as i128;
                lock_pos -= rotations * 100;
                println!("baump: {}", num_zeros);
            }
        }
        else {
            lock_pos -= num;

            if lock_pos < 0 {
                let rotations = (lock_pos / 100) - 1; // this one is wrong still. If 53 - 953 -> 10 rotations end at 0 not 100
                num_zeros -= rotations as i128;
                lock_pos -= rotations * 100;

                // Please no one look at this code, I too am ashamed.
                if lock_pos == 100 {
                    lock_pos = 0;
                }

                if dumb_flag {
                    num_zeros -= 1;
                }
                println!("baump {}", num_zeros);
            }
            else if lock_pos == 0 {
                num_zeros += 1;
                println!("baump 3: {}", num_zeros);
            }
        }

        // println!("Position: {}, num_zeros: {}", lock_pos, num_zeros);
    }

    println!("Num zeros: {}, lock_pos: {}", num_zeros, lock_pos);
    Ok(())
}
