use crate::lib;

// 2 = 1
// 3 = 7
// 4 = 4
// 5 = 2,3,5
// 6 = 0,6,9
// 7 = 8

fn vec_diff<T: PartialEq + Copy + Clone>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let mut difference: Vec<T> = vec![];
    for i in a {
        if !b.contains(i) {
            difference.push(*i);
        }
    }
    difference
}

fn mapping<T: PartialEq + Ord + Copy + Clone>(s: &Vec<T>, decoded: &[Vec<T>; 10]) -> Option<usize> {
    for (i, t) in decoded.iter().enumerate() {
        if *s == *t {
            return Some(i);
        }
    }
    None
}

pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let outputs: Vec<&str> = input
        .iter()
        .filter_map(|s| s.split(" | ").last())
        .flat_map(|s| s.split_whitespace())
        .collect();

    outputs.iter().fold(0, |acc, s| match s.len() {
        2 | 3 | 4 | 7 => acc + 1,
        _ => acc,
    })
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);

    let mut sum: usize = 0;

    for l in input.iter() {
        let ss: Vec<(Vec<char>, Vec<usize>)> = l
            .split_whitespace()
            .filter(|s| *s != "|")
            .map(|s| {
                let mut v: Vec<char> = s.chars().collect();
                v.sort();
                v
            })
            .map(|v| match v.len() {
                2 => (v, vec![1]),
                3 => (v, vec![7]),
                4 => (v, vec![4]),
                5 => (v, vec![2, 3, 5]),
                6 => (v, vec![0, 6, 9]),
                7 => (v, vec![8]),
                _ => (v, vec![]),
            })
            .collect();

        let one: Vec<char> = ss
            .clone()
            .into_iter()
            .filter(|x| x.1.contains(&1))
            .map(|x| x.0)
            .last()
            .unwrap();

        let four: Vec<char> = ss
            .clone()
            .into_iter()
            .filter(|x| x.1.contains(&4))
            .map(|x| x.0)
            .last()
            .unwrap();

        let seven: Vec<char> = ss
            .clone()
            .into_iter()
            .filter(|x| x.1.contains(&7))
            .map(|x| x.0)
            .last()
            .unwrap();

        let eight: Vec<char> = ss
            .clone()
            .into_iter()
            .filter(|x| x.1.contains(&8))
            .map(|x| x.0)
            .last()
            .unwrap();

        let six: Vec<char> = ss
            .clone()
            .into_iter()
            .filter(|x| x.1.contains(&6) && vec_diff(&one, &x.0).len() == 1)
            .map(|x| x.0)
            .last()
            .unwrap();

        let nine: Vec<char> = ss
            .clone()
            .into_iter()
            .filter(|x| x.1.contains(&9) && vec_diff(&x.0, &four).len() == 2)
            .map(|x| x.0)
            .last()
            .unwrap();

        let zero: Vec<char> = ss
            .clone()
            .into_iter()
            .filter(|x| x.1.contains(&0) && x.0 != nine && x.0 != six)
            .map(|x| x.0)
            .last()
            .unwrap();

        let five: Vec<char> = ss
            .clone()
            .into_iter()
            .filter(|x| x.1.contains(&5) && vec_diff(&six, &x.0).len() == 1)
            .map(|x| x.0)
            .last()
            .unwrap();

        let three: Vec<char> = ss
            .clone()
            .into_iter()
            .filter(|x| x.1.contains(&3) && vec_diff(&one, &x.0).len() == 0)
            .map(|x| x.0)
            .last()
            .unwrap();

        let two: Vec<char> = ss
            .clone()
            .into_iter()
            .filter(|x| x.1.contains(&2) && x.0 != three && x.0 != five)
            .map(|x| x.0)
            .last()
            .unwrap();

        let decoded = [zero, one, two, three, four, five, six, seven, eight, nine];

        let output: Vec<usize> = l
            .split(" | ")
            .last()
            .unwrap()
            .split_whitespace()
            .map(|s| {
                let mut v: Vec<char> = s.chars().collect();
                v.sort();
                v
            })
            .filter_map(|s| mapping(&s, &decoded))
            .collect();

        sum += output[0] * 1000 + output[1] * 100 + output[2] * 10 + output[3];
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::y2021::day08;

    #[test]
    fn test1() {
        assert_eq!(day08::puzzle1("src/y2021/day08/test.txt"), 26);
        assert_eq!(day08::puzzle1("src/y2021/day08/input.txt"), 452);
    }

    #[test]
    fn test2() {
        assert_eq!(day08::puzzle2("src/y2021/day08/test.txt"), 61229);
        assert_eq!(day08::puzzle2("src/y2021/day08/input.txt"), 1096964);
    }
}
