use std::{
    fs,
    collections::{
        hash_map,
        VecDeque
    }
};


fn read_day1_data() -> (Vec<u32>, Vec<u32>) {
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

    return (l1, l2)
}

fn day1part1(l1: &Vec<u32>, l2: &Vec<u32>) -> u32 {
    let mut total_length = 0;
    for i in 0..l1.len() {
        total_length += l1[i].abs_diff(l2[i]);
    }
    return total_length;
}

fn read_day2_data(data_location: &str) -> Vec<VecDeque<i32>> {
    fs::read_to_string(data_location)
        .expect("Can't find the input file")
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| {
            return x.split(" ")
            .map(|y| y.parse::<i32>()
                .expect("Error while converting to int"))
            .collect::<VecDeque<_>>()
        }).collect::<Vec<_>>()
}

fn day2part1() -> u32 {
    let data = read_day2_data("./inputs/day2input");
    let mut safe_data_count = 0;

    for i in 0..data.len() {
        let d = &data[i];
        let mut is_descending = -1;
        let mut is_current_safe = true;
        for j in 1..d.len() {
            let diff = d[j] - d[j - 1];
            if diff < 0  && is_descending == -1{
                is_descending = 1;
            } else if diff > 0 && is_descending == -1 {
                is_descending = 0;
            }

            if (is_descending == 1 && diff > 0) || (is_descending == 0 && diff < 0) {
                is_current_safe = false;
                break;
            } 

            if diff == 0{
                is_current_safe = false;
                break;
            }

            if (diff <= -4 || diff == 0) && is_descending == 1 {
                is_current_safe = false;
                break;
            } else if (diff >= 4 || diff == 0) && is_descending == 0 {
                is_current_safe = false;
                break;
            }
        }

        if is_current_safe {
            safe_data_count += 1;
        }
    }

    return safe_data_count;
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
    println!("safe_data_count: {:?}", day2part1());
}
