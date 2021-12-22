use crate::lib;

const LEGAL_OPEN: [char; 4] = ['(', '[', '{', '<'];
const LEGAL_CLOSE: [char; 4] = [')', ']', '}', '>'];
const ERR_SCORE_CORRUPT: [usize; 4] = [3, 57, 1197, 25137];
const ERR_SCORE_INCOMPLETE: [usize; 4] = [1, 2, 3, 4];

fn line_error_score(l: &String) -> Option<(Option<usize>, Option<usize>)> {
    let mut stack: Vec<char> = vec![];

    for c in l.chars() {
        if LEGAL_OPEN.contains(&c) {
            // Got an opening char
            stack.push(c);
        } else if LEGAL_CLOSE.contains(&c) {
            // Got a closing char
            let o = stack.last().unwrap();

            if (c as i32 - *o as i32).abs() <= 2 {
                // Legal close
                stack.pop();
            } else {
                // Incorrect close => corrupted
                let err_idx = LEGAL_CLOSE.iter().position(|p| p == &c).unwrap();
                let corruption_score = ERR_SCORE_CORRUPT[err_idx];
                return Some((Some(corruption_score), None));
            }
        }
    }

    if stack.len() > 0 {
        // Stack is not empty => incomplete
        stack.reverse();
        let incomplete_score = stack.iter().fold(0, |sum, c| {
            let err_idx = LEGAL_OPEN.iter().position(|p| p == c).unwrap();
            sum * 5 + ERR_SCORE_INCOMPLETE[err_idx]
        });
        return Some((None, Some(incomplete_score)));
    }

    None
}
pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);

    input
        .iter()
        .filter_map(line_error_score)
        .filter_map(|l| l.0)
        .fold(0, |sum, e| sum + e)
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);

    let mut scores: Vec<usize> = input
        .iter()
        .filter_map(line_error_score)
        .filter_map(|l| l.1)
        .collect();

    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use crate::y2021::day10;

    #[test]
    fn test1() {
        assert_eq!(day10::puzzle1("src/y2021/day10/test.txt"), 26397);
        assert_eq!(day10::puzzle1("src/y2021/day10/input.txt"), 392139);
    }

    #[test]
    fn test2() {
        assert_eq!(day10::puzzle2("src/y2021/day10/test.txt"), 288957);
        assert_eq!(day10::puzzle2("src/y2021/day10/input.txt"), 4001832844);
    }
}
