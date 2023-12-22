use std::fs;
use std::collections::HashMap;

fn get_possible_game_sum(cubes_map: &HashMap<i32, Vec<HashMap<&str, i32>>>) -> i32 {
    let mut total = 0;

    // Map of limit values
    let limit_map = HashMap::from([("green", 13),("red", 12),("blue",14)]);

    // Check if any pull per game has a value higher than the corresponding limit
    for (game_id, pulls) in cubes_map {
        let mut possible = true;

        'pull_loop: for pull in pulls {
            for color in ["red", "green", "blue"] {
                if pull.get(color) > limit_map.get(color) {
                    possible = false;
                    break 'pull_loop;
                }
            }
        }

        if possible {
            total += game_id;
        }
    }

    return total;
}

fn get_power_of_minimal_sets(cubes_map: &HashMap<i32, Vec<HashMap<&str, i32>>>) -> i32 {
    let mut total = 0;

    // Find the maximum value for each color out of all pulls and multiply them
    for (_, pulls) in cubes_map {
        let mut max_color_amount = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        for pull in pulls {
            for color in ["red", "green", "blue"] {
                if pull.get(color) > max_color_amount.get(color) {
                    let entry = max_color_amount.entry(color).or_insert(0);
                    *entry = *pull.get(color).unwrap();
                }
            }
        }
        
        let power = *max_color_amount.get("red").unwrap() *
                    *max_color_amount.get("green").unwrap() *
                    *max_color_amount.get("blue").unwrap(); 

        total += power;
    }

    return total;
}

fn main() {
    // Read input
    let file_path = "input";
    let contents = fs::read_to_string(file_path)
                   .expect("Error reading file");


    // Split at ":" to get a map of game ID to list of pull strings
    let games = contents.lines()
                        .map( |line| {
                                (line.split(":").nth(0).unwrap()
                                    .split(" ").nth(1).unwrap(),
                                line.split(":").nth(1).unwrap()
                                    .split(";").map(|s|s.trim()).collect::<Vec<&str>>())
                            });
    let games_map: HashMap<&str, Vec<&str>> = HashMap::from_iter(games);

    // Convert pull strings into maps
    let cubes = games_map.iter().map( |(game_num, pulls)| {

        let pulls_map = pulls.iter().map( |pull| {
            let cubes_vec = pull.split(", ").map( |cube_color|
                (cube_color.split(" ").nth(1).unwrap(),
                cube_color.split(" ").nth(0).unwrap().parse::<i32>().unwrap())
            );
            let cubes_map: HashMap<&str, i32> = HashMap::from_iter(cubes_vec);
            cubes_map
        });

        (game_num.parse::<i32>().unwrap(), pulls_map.collect())
    });
    let cubes_map: HashMap<i32, Vec<HashMap<&str, i32>>> = HashMap::from_iter(cubes);

    // Get solutions
    let possible_game_sum = get_possible_game_sum(&cubes_map);
    let minimal_set_power = get_power_of_minimal_sets(&cubes_map);

    println!("Sum of possible game IDs: {}\n Power of minimal sets: {}", possible_game_sum, minimal_set_power);

}
