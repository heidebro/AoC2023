use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    // Read input
    let file_path = "input";
    let contents: String = fs::read_to_string(file_path)
                   .expect("Error reading file");

    // Go through file line by line
    // Always Look at current line, line before and after
    // Filter out all numbers and their position in the current line
    // Filter out all symbols and their locations in all 3 lines
    // Check which numbers fulfill criteria
        // Possible inclusion criteria are any symbols in the following spots: n = number location
        // current line: n+len(n), n-1,
        // prev/next line: n-1, n, n+1, n+2, ..., n+len(n)-1, n+len(n)

    
    let digit_pattern = Regex::new(r"\d+").unwrap();

    let mut numbers_per_line: Vec<HashMap<i32, &str>> = Vec::new();
    let mut symbols_per_line: Vec<HashMap<i32, char>> = Vec::new();

    let mut total = 0;
    
    for line in contents.lines() {
        let numbers = digit_pattern.find_iter(line).map(|m| m.as_str())
                                   .map(|n| line.match_indices(n).collect::<Vec<(usize, &str)>>())
                                   .flatten()
                                   .map(|(i, n)| (i as i32, n));
        let number_pos_map: HashMap<i32, &str> = HashMap::from_iter(numbers);
        
        let symbols = line.chars()
                          .enumerate()
                          .filter(|(_, c)| !c.is_digit(10) && *c != '.' )
                          .map(|(pos, c)| (pos as i32, c));
        let symbol_pos_map: HashMap<i32, char> = HashMap::from_iter(symbols);


        numbers_per_line.push(number_pos_map);
        symbols_per_line.push(symbol_pos_map);
    };

    for (linenum, numbers) in numbers_per_line.iter().enumerate() {
        // Get symbol positions for current, previous and next line
        let symbols_cur: &HashMap<i32, char> = &symbols_per_line[linenum];

        // For all numbers in 
        for (position, number) in numbers {
            let number_len: i32 = number.len() as i32;
            // Check current line
            let mut is_partnum: bool = symbols_cur.contains_key(&(position-1)) || symbols_cur.contains_key(&(position+number_len));

            // Check prev line
            if linenum > 0 {
                let symbols_prev = &symbols_per_line[linenum-1];
                // How to do this properly starting from -1?
                let positions: Vec<i32> = (0..number_len+2).map(|n| position+n-1).collect();
                is_partnum = is_partnum || positions.iter().fold(false, |acc, p| symbols_prev.contains_key(p) || acc );
            }

            // Check next line
            if linenum < numbers_per_line.len()-1 {
                let symbols_next = &symbols_per_line[linenum+1];
                let positions: Vec<i32> = (0..number_len+2).map(|n| position+n-1).collect();
                is_partnum = is_partnum || positions.iter().fold(false, |acc, p| symbols_next.contains_key(p) || acc );
            }
            
            if is_partnum {
                total += number.parse::<usize>().unwrap();
            }

        }
    }

    println!("{}", total);

}
