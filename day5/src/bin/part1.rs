use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Number {
    /// Numbers that must come before this number
    before: HashSet<u8>,

    /// Numbers that must come after this number
    after: HashSet<u8>,
}

impl Number {
    fn new(num_before: Option<u8>, num_after: Option<u8>) -> Self {
        let mut number = Number {
            before: HashSet::new(),
            after: HashSet::new(),
        };

        match (num_before, num_after) {
            (None, None) => (),
            (None, Some(a)) => {
                number.after.insert(a);
            }
            (Some(b), None) => {
                number.before.insert(b);
            }
            (Some(b), Some(a)) => {
                number.before.insert(b);
                number.after.insert(a);
            }
        }

        number
    }
}

fn main() -> Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut string = String::new();

    let _ = file.read_to_string(&mut string)?;

    let mut split = string.split("\n\n");
    let orders = split.next().ok_or(anyhow!("No order data"))?;
    let sequences = split.next().ok_or(anyhow!("No sequence data"))?;

    let mut number_set: HashMap<u8, Number> = HashMap::new();

    for order in orders.split("\n") {
        let mut order_split = order.split("|");

        let order_small = order_split
            .next()
            .ok_or(anyhow!("Couldn't get smaller order"))?
            .parse::<u8>()?;
        let order_big = order_split
            .next()
            .ok_or(anyhow!("Couldn't get bigger order"))?
            .parse::<u8>()?;

        if let Some(small_collection) = number_set.get_mut(&order_small) {
            small_collection.after.insert(order_big);
        } else {
            let number = Number::new(None, Some(order_big));
            number_set.insert(order_small, number);
        }

        if let Some(big_collection) = number_set.get_mut(&order_big) {
            big_collection.before.insert(order_small);
        } else {
            let number = Number::new(Some(order_small), None);
            number_set.insert(order_big, number);
        }
    }

    let result: u32 = sequences
        .split("\n")
        .filter_map(|sequence| {
            let split = sequence
                .split(",")
                .map(|sequence_item| sequence_item.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();

            let is_ok = !split
                .iter()
                .enumerate()
                .map(|(i, sequence_item)| {
                    let seq_item_orders = number_set
                        .get(&sequence_item)
                        .ok_or(anyhow!("Couldn't find sequence item in set"))
                        .unwrap();

                    let split_before = split.clone();
                    let before_ok = !split_before
                        .iter()
                        .take_while(|smaller_number| smaller_number != &sequence_item)
                        .map(|smaller_number| seq_item_orders.after.contains(&smaller_number))
                        .any(|b| b);

                    let split_after = split.clone();
                    let after_ok = !split_after
                        .iter()
                        .skip(i + 1)
                        .map(|bigger_number| seq_item_orders.before.contains(&bigger_number))
                        .any(|b| b);

                    before_ok && after_ok
                })
                .any(|b| !b);

            if is_ok {
                return Some(*split.iter().nth(((split.len() + 1) / 2) - 1).unwrap() as u32);
            } else {
                return None;
            }
        })
        .sum();

    println!("{result}");

    Ok(())
}
