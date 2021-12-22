use crate::lib;
use std::collections::HashMap;

type Pair = (char, char);

fn parse_rules(input: &Vec<String>) -> HashMap<Pair, char> {
    input
        .iter()
        .map(|s| s.split("->").flat_map(|s| s.trim().chars()).collect())
        .filter(|v: &Vec<char>| v.len() == 3)
        .fold(HashMap::new(), |mut h, v| {
            h.insert((v[0], v[1]), v[2]);
            h
        })
}

fn parse_pairs(input: &String) -> Vec<Pair> {
    let chars: Vec<char> = input.chars().collect();
    chars
        .iter()
        .zip(chars.iter().skip(1))
        .map(|pair| (*pair.0, *pair.1))
        .collect()
}

fn pair_counts(pairs: &Vec<Pair>) -> HashMap<Pair, usize> {
    pairs.iter().fold(HashMap::new(), |mut h, pair| {
        *h.entry(*pair).or_default() += 1;
        h
    })
}

fn run_step(state: &HashMap<Pair, usize>, rules: &HashMap<Pair, char>) -> HashMap<Pair, usize> {
    let mut state = state.clone();
    let mut t_state: HashMap<Pair, usize> = HashMap::new();

    // Insert characters based on rules
    for (pair, c) in rules {
        if let Some(count) = state.remove(pair) {
            // Each insert doubles the count for the corresponding pair
            // as it splits each pair into 2 new pairs
            let triple = (pair.0, *c, pair.1);
            *t_state.entry((triple.0, triple.1)).or_default() += count;
            *t_state.entry((triple.1, triple.2)).or_default() += count;
        }
    }

    state.extend(t_state);

    state
}

fn char_freq(state: &HashMap<Pair, usize>, first_char: char) -> Vec<usize> {
    state
        .iter()
        .fold(
            {
                // Account for the first char in the initial template
                let mut h = HashMap::new();
                h.insert(first_char, 1);
                h
            },
            |mut h: HashMap<char, usize>, (pair, count)| {
                // Only count the last char in each pair as that will "iterate"
                // across all the chars in the resulting template
                // (not including the first char)
                *h.entry(pair.1).or_default() += count;
                h
            },
        )
        .into_values()
        .collect()
}

pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let rules: HashMap<Pair, char> = parse_rules(&input);
    let mut state: HashMap<Pair, usize> = pair_counts(&parse_pairs(&input[0]));

    for _ in 0..10 {
        state = run_step(&state, &rules);
    }

    let f = char_freq(&state, input[0].chars().next().unwrap());

    let least_common = f.iter().min().unwrap();
    let most_common = f.iter().max().unwrap();

    most_common - least_common
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let rules: HashMap<Pair, char> = parse_rules(&input);
    let mut state: HashMap<Pair, usize> = pair_counts(&parse_pairs(&input[0]));

    for _ in 0..40 {
        state = run_step(&state, &rules);
    }

    let f = char_freq(&state, input[0].chars().next().unwrap());

    let least_common = f.iter().min().unwrap();
    let most_common = f.iter().max().unwrap();

    most_common - least_common
}

#[cfg(test)]
mod tests {
    use crate::y2021::day14;

    #[test]
    fn test1() {
        assert_eq!(day14::puzzle1("src/y2021/day14/test.txt"), 1588);
        assert_eq!(day14::puzzle1("src/y2021/day14/input.txt"), 3247);
    }

    #[test]
    fn test2() {
        assert_eq!(day14::puzzle2("src/y2021/day14/test.txt"), 2188189693529);
        assert_eq!(day14::puzzle2("src/y2021/day14/input.txt"), 4110568157153);
    }
}
