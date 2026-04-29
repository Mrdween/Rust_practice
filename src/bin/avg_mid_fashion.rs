use std::collections::HashMap;
use std::io;

fn average(nums: Option<&Vec<usize>>)  -> u32 {
    match nums {
        None => return 0,
        Some(values) => {
            let sum: usize = values.iter().sum();
            (sum / values.len()) as u32
        },
    }
}

fn middle(value: Option<&Vec<usize>>) -> u32 {
    match value {
        None => return 0,
        Some(values) => {
            if  values.len() % 2 == 1 {
                let mid = (values.len() - 1) / 2;
                values[mid] as u32
            } else {
                println!("Среднего значения не найдено");
                0
            }
        },
    }
}
fn fashion(moda: Option<&Vec<usize>>) -> u32 {
    match moda {
        None => return 0,
        Some(values) => {
            let mut counts = HashMap::new();
            for v in values {
                *counts.entry(v).or_insert(0) += 1;
            }
            counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(_, value)| value as u32)
            .unwrap_or(0)

        },
    }
}

fn user_input() -> Vec<usize> {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Строка не распознана");
    input
    .split_whitespace()
    .filter_map(|s| s.parse::<usize>().ok())
    .collect()
}
fn main() {
    let input = user_input();
    middle(Some(&input));
    average(Some(&input));
    fashion(Some(&input));
}
