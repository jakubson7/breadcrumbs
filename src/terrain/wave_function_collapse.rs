use rand::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn collapse_cell(rules: &HashMap<u16, HashSet<u16>>, neighbors: &Vec<u16>) -> u16 {
    let mut candidats: HashSet<u16> = HashSet::from_iter(rules.clone().into_keys());

    for n in neighbors {
        if let Some(possibilities) = rules.get(&n) {
            candidats = possibilities.intersection(&candidats).cloned().collect();
        }
    }

    if candidats.len() == 0 {
        let mut max_count = 0;
        let mut max_value = 0;

        for n in neighbors.clone().iter() {
            let count = neighbors.iter().filter(|&x| x == n).count();
            if count > max_count {
                max_value = *n;
                max_count = count;
            }
        }

        return max_value;
    }

    let result = candidats
        .iter()
        .choose(&mut rand::thread_rng())
        .unwrap_or(&0);
    return *result;
}
