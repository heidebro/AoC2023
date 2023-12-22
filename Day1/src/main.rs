use std::fs;
use std:: collections::HashMap;

fn main() {
    let file_path = "input";
    let contents = fs::read_to_string(file_path)
    .expect("Error reading file");

    let nums = HashMap::from([
        ("one",1),
        ("two",2),
        ("three",3),
        ("four",4),
        ("five",5),
        ("six",6),
        ("seven",7),
        ("eight",8),
        ("nine",9),
    ]);

    let number_tokens = ["one",
                     "two",
                     "three",
                     "four",
                     "five",
                     "six",
                     "seven",
                     "eight",
                     "nine",
                     "1",
                     "2",
                     "3",
                     "4",
                     "5",
                     "6",
                     "7",
                     "8",
                     "9"];

    let mut total_amount = 0;

    for line in contents.lines() {
        let mut first_num = "";
        let mut last_num = "";

        let mut lowest_pos = 9999999; 
        let mut highest_pos = 0;

        for token in number_tokens {
            let pos: Vec<_> = line.match_indices(token).collect();
            if pos.len() == 0 {
                continue
            }

            if pos[0].0 < lowest_pos {
                lowest_pos = pos[0].0;
                first_num = token;
            }
            if pos[pos.len()-1].0 > highest_pos {
                highest_pos = pos[pos.len()-1].0;
                last_num = token;
            }
        }

        let first_int;
        let last_int;

        if nums.contains_key(first_num) {
            first_int = *nums.get(first_num).unwrap();
        } else {
            first_int = first_num.parse::<u32>().unwrap();
        }

        if nums.contains_key(last_num) {
            last_int = *nums.get(last_num).unwrap();
        } else {
            last_int = match last_num.parse::<u32>() {
                Ok(x) => x,
                Err(e) => first_int,
            };
        }


        let final_num = first_int * 10 + last_int;
        println!("{} = {}", line, final_num);
        total_amount += final_num;

        /*
        for c in line.chars() {
            if !first_set && "0123456789".contains(c) {
                first_num = c.to_digit(10).unwrap();
                first_set = true;
            } 
            if "0123456789".contains(c) {
                last_num = c.to_digit(10).unwrap();
            }
        }
        */
    }
    println!("Final sum: {}", total_amount);
}