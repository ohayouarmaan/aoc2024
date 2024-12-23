use std::{
    fs,
    collections::hash_map
};


fn day1part1(l1: &Vec<u32>, l2: &Vec<u32>) -> u32 {
    let mut total_length = 0;
    for i in 0..l1.len() {
        total_length += l1[i].abs_diff(l2[i]);
    }
    return total_length;
}

fn day1part2(l1: &Vec<u32>, l2: &Vec<u32>) -> u32 {
    let mut similarity_score = 0;
    let mut visited: hash_map::HashMap<u32, u32> = hash_map::HashMap::new();
    for item in l2 {
        if let Some(_) = visited.keys().find(|x| **x == *item) {
            visited.insert(*item, visited.get(item).expect("") + 1);
        } else {
            visited.insert(*item, 1);
        }
    }
    for item in l1 {
        if let Some(_) = visited.keys().find(|x| **x == *item) {
            similarity_score += item * visited.get(item).expect("");
        }
    }

    return similarity_score;
}

fn main() {
    let input = fs::read_to_string("./inputs/day1input")
                    .expect("Error while opening the file.")
                    .split("\n")
                    .into_iter()
                    .map(|x| x.split("   ")
                        .filter(|x| *x != "")
                        .map(|x| {
                            return x.parse::<u32>()
                            .expect("Error while converting to integer")
                        })
                        .collect::<Vec<_>>())
                    .collect::<Vec<_>>();

    let mut l1: Vec<u32> = Vec::new();
    let mut l2: Vec<u32> = Vec::new();

    for inp in input {
        if inp != [] {
            l1.push(inp[0]);
            l2.push(inp[1]);
        }
    }

    l1.sort();
    l2.sort();
    println!("total_length: {:?}", day1part2(&l1, &l2));
}
