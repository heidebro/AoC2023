use std::collections::HashSet;
use std::fs;

#[cfg(test)]
mod tests {
    #[test]
    fn split_numbers() {
        let line: &str = "48 68 49  4  7 33  6 73  1 95";
        let numbers: Vec<&str> = line.split(" ").filter(|n| !n.is_empty()).collect();
        println!("{:?}", numbers);

        let expected: Vec<&str> =
            Vec::from(["48", "68", "49", "4", "7", "33", "6", "73", "1", "95"]);

        assert_eq!(numbers, expected);
    }
}

// Parse cards, returning a List of lists
// Each entry is a list with 2 elements, the first being the substring of winning nummbers
// the second being the substring of our numbers
fn parse_cards<'a>(contents: &'a String) -> Vec<Vec<&'a str>> {
    contents
        .lines()
        .map(|line| line.split(":").last().unwrap().trim())
        .map(|line| line.split("|").map(|s| s.trim()).collect::<Vec<&str>>())
        .collect()
}

// Return a list of intersecting numbers for each game
fn get_intersection_winning_own_numbers<'a>(cards: &Vec<Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    cards
        .iter()
        .map(|v| {
            let winning_num: HashSet<&str> =
                HashSet::from_iter(v[0].split(" ").filter(|n| !n.is_empty()));
            let my_num: HashSet<&str> =
                HashSet::from_iter(v[1].split(" ").filter(|n| !n.is_empty()));

            Vec::from_iter(winning_num.intersection(&my_num).map(|s| *s))
        })
        .collect()
}

// Calculate points for Part 1
fn get_total_points(contents: &String) -> i32 {
    let cards = parse_cards(&contents);
    get_intersection_winning_own_numbers(&cards)
        .iter()
        .fold(0, |topacc, v| {
            v.iter()
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc + acc })
                + topacc
        })
}

// Get total Scratchcard number for part 2
fn get_total_scratchcards(contents: &String) -> i32 {
    // Get hashmap of card:#matching nums
    let cards = parse_cards(&contents);
    let matches_per_card: Vec<i32> = get_intersection_winning_own_numbers(&cards)
        .iter()
        .map(|card_inter| card_inter.len() as i32)
        .collect();

    // Go through hashmap and count up multipliers of each card (occurrences) in new hashmap
    let num_cards = contents.lines().count();

    let mut occurences: Vec<i32> = (0..num_cards).map(|_| 1).collect();

    for (cardid, matches_cnt) in matches_per_card.iter().enumerate() {
        (1..*matches_cnt + 1).for_each(|m| {
            occurences[cardid + (m as usize)] += 1 * occurences[cardid];
        });
    }
    // Add together all values of second hashmap + original
    occurences.iter().fold(0, |acc, n| acc + n)
}

fn main() {
    let file_path = "input";
    let contents: String = fs::read_to_string(file_path).expect("Error reading file");

    println!("Part1: {}", get_total_points(&contents));
    println!("Part2: {}", get_total_scratchcards(&contents));
}
