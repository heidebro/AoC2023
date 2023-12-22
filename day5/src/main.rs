use std::fs;
use std::cmp;

fn part1(seeds: &Vec<u64>, maps: &Vec<Vec<Vec<u64>>>) -> u64 {
    let mut final_ids: Vec<u64> = Vec::new();

    for seed in seeds {
        let mut id: u64 = *seed;

        for map in maps {
            for m in map.iter().filter(|m| !m.is_empty()) {
                if id >= m[1] && id < m[1] + m[2] {
                    id = m[0] + (id - m[1]);
                    break;
                } 
            }
        }


        final_ids.push(id);
    }

    let minimal_id = final_ids.iter().fold(std::u64::MAX, |acc, n| if n < &acc { *n } else {acc});
    minimal_id
}

struct SeedRange {
    start: u64,
    length: u64
}

fn part2(seeds: &Vec<u64>, maps: &Vec<Vec<Vec<u64>>>) -> u64 {
    let mut final_ids: Vec<u64> = Vec::new();


    /*
        Here we need to consider multiple seed ranges. So the seed list needs
        to be converted first.
        Then, while iterating, the ranges will split into a part that is mapped and one that is 
        not. Thus for each map, we iterate through all seed ranges of this round, test them,
        split them, collect the newly split ones in an array of seed ranges which then is 
        used in the following map loop
     */    
    let ranges: Vec<SeedRange> = (0..seeds.len()).step_by(2).map(|i| SeedRange{start: seeds[i], length: seeds[i+1]}).collect();

    for range in ranges {
        let mut id: u64 = range.start;
        let mut seed_range: u64 = range.length;
        let mut loc_range: Vec<SeedRange> = Vec::from([range]);

        for map in maps {
            for m in map.iter().filter(|m| !m.is_empty()) {
                // We have a mapping overlap
                if (m[1] <= id + seed_range) && (m[1] + m[2] > id) {
                    if m[1] >= id {
                        // |--------|
                        //     |-------|

                    } else if id >= m[1] {
                        //     |-------|
                        // |--------|

                    }

                    if id > m[1] && id + seed_range < m[1] + m[2]  {
                        // |--------|
                        //  |------|
                    }
                    // Determine submap (starting id of overlap, length of overlap)
                    let s = SeedRange {
                        start: cmp::max(m[1], id),
                        length: cmp::min(m[1] + m[2], id + seed_range) - id,
                    };
                    // TODO: Add this to the list of ranges (Since non-mapped ids may still map later on)


                    break;
                }
            }
        }


        final_ids.push(id);
    }

    let minimal_id = final_ids.iter().fold(std::u64::MAX, |acc, n| if n < &acc { *n } else {acc});
    minimal_id
}

fn main() {
    let path = "input";
    let contents = fs::read_to_string(path).expect("Error reading file");

    // Parse Sections
    let mut sections: Vec<String> = Vec::new();

    let mut section: String = String::new();
    for line in contents.lines() {
        if line == "" {
            sections.push(section);
            section = String::new();
        }

        section.push_str(line);
        section.push('\n');
    }

    sections.push(section);

    let mut sections_iter = sections.iter();

    // Extract Seed IDs
    let seeds: Vec<u64> = sections_iter.next().unwrap()
                             .split(":").nth(1).unwrap()
                             .trim().split(" ")
                             .map(|n| n.parse().unwrap())
                             .collect();

    // Parse Maps
    let maps: Vec<Vec<Vec<u64>>> = sections_iter.map(|n| {
        n.split(":").nth(1).unwrap()
                    .lines().map(|line| line.split(" ")
                                            .filter(|n| !n.is_empty())
                                            .map(|n| n.parse().unwrap()).collect()
                    )
                    .collect()
    }).collect();



    // For each seed
        // Check if the seed ID is in between the source range start and source range start + Range
        // Keep doing that for each map
    println!("part1: {}", part1(&seeds, &maps));
    println!("part2: {}", part2(&seeds, &maps));
}
