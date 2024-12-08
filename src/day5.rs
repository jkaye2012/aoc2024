use std::collections::{HashMap, HashSet};

pub struct PrintQueue {
    before_rules: HashMap<u8, HashSet<u8>>,
    after_rules: HashMap<u8, HashSet<u8>>,
    updates: Vec<Vec<u8>>,
}

impl From<&str> for PrintQueue {
    fn from(value: &str) -> Self {
        let (raw_rules, raw_updates) = value.split_once("\n\n").unwrap();
        let mut before_rules = HashMap::new();
        let mut after_rules = HashMap::new();
        raw_rules.split_whitespace().for_each(|rr| {
            let (before, after) = rr.split_once('|').unwrap();
            let (before, after) = (before.parse::<u8>().unwrap(), after.parse::<u8>().unwrap());

            if !before_rules.contains_key(&before) {
                before_rules.insert(before, HashSet::new());
            }
            before_rules.get_mut(&before).unwrap().insert(after);

            if !after_rules.contains_key(&after) {
                after_rules.insert(after, HashSet::new());
            }
            after_rules.get_mut(&after).unwrap().insert(before);
        });

        let updates = raw_updates
            .split_whitespace()
            .map(|u| u.split(',').map(|c| c.parse::<u8>().unwrap()).collect())
            .collect();

        Self {
            before_rules,
            after_rules,
            updates,
        }
    }
}

#[aoc_generator(day5)]
pub fn generate(input: &str) -> PrintQueue {
    PrintQueue::from(input)
}

#[aoc(day5, part1)]
pub fn middle_ordered_updates(queue: &PrintQueue) -> usize {
    queue
        .updates
        .iter()
        .filter_map(|us| {
            let mut seen = Vec::new();
            if us.iter().all(|u| {
                let rules = queue.before_rules.get(u);
                let result = seen
                    .iter()
                    .all(|s| !rules.map(|r| r.contains(s)).unwrap_or(false));
                seen.push(*u);
                result
            }) {
                Some(us[us.len() / 2] as usize)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day5, part2)]
pub fn reordered_invalid_updates(queue: &PrintQueue) -> usize {
    0
}
