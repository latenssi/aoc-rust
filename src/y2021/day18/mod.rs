use crate::lib;

#[derive(Debug, Clone)]
enum Num {
    R(usize),              // Regular
    P(Box<Num>, Box<Num>), // Pair
}

impl Num {
    fn from_str(s: &str) -> Result<Num, String> {
        Num::from_chars(&s.replace(" ", "").chars().collect::<Vec<char>>());
        Ok(Num::R(0))
    }

    fn from_str_2(s: &str) -> Result<Num, String> {
        Num::from_chars_2(&s.replace(" ", "").chars().collect::<Vec<char>>(), 0, 0);
        Ok(Num::R(0))
    }

    fn from_chars(chars: &[char]) -> Option<Num> {
        let mut depth = 0;
        let mut vals: Vec<(usize, Option<u32>)> = vec![];

        for i in 0..chars.len() {
            match chars[i] {
                '[' => depth += 1,
                ']' => depth -= 1,
                c @ '0'..='9' => vals.push((depth, c.to_digit(10))),
                _ => (),
            }
        }

        Num::P(Box::new(Num::R(0)), Box::new(Num::R(0)));

        println!("vals: {:?}", vals);

        None
    }

    fn from_chars_2(chars: &[char], i: usize, depth: usize) -> Option<Num> {
        match chars[i] {
            '[' => Num::from_chars_2(chars, i + 1, depth + 1),
            ']' => Num::from_chars_2(chars, i + 1, depth - 1),
            c @ '0'..='9' => {
                if let Some(n) = c.to_digit(10) {
                    Some(Num::R(n as usize))
                } else {
                    None
                }
            }
            _ => Num::from_chars_2(chars, i + 1, depth),
        }
    }

    fn mag(&self) -> usize {
        match self {
            Num::R(v) => *v,
            Num::P(l, r) => 3 * l.mag() + 2 * r.mag(),
        }
    }

    fn split(&mut self) -> Result<(), String> {
        match self {
            Num::R(v) => {
                let v = *v;
                *self = Num::P(Box::new(Num::R(v / 2)), Box::new(Num::R(v / 2 + v % 2)));
                Ok(())
            }
            _ => Err("can not split".to_string()),
        }
    }

    fn add(&mut self, a: usize) -> Result<(), String> {
        match self {
            Num::R(v) => {
                *v += *v + a;
                Ok(())
            }
            _ => Err("can not add".to_string()),
        }
    }

    fn explode(&mut self) -> Result<(usize, usize), String> {
        match self {
            Num::P(l, r) => {
                if let (Num::R(lv), Num::R(rv)) = (*l.clone(), *r.clone()) {
                    *self = Num::R(0);
                    Ok((lv, rv))
                } else {
                    Err("can not explode".to_string())
                }
            }
            _ => Err("can not explode".to_string()),
        }
    }
}

pub fn puzzle1(input_filename: &str) -> isize {
    let input = lib::read_lines(input_filename);
    // let n = input
    //     .iter()
    //     .filter_map(|s| Num::from_str(s).ok())
    //     .collect::<Vec<Num>>();

    // println!("{:?}", n);

    Num::from_str(&input[7]);
    Num::from_str_2(&input[7]);

    0
}

pub fn puzzle2(input_filename: &str) -> isize {
    let input = lib::read_lines(input_filename);
    0
}

#[cfg(test)]
mod tests {
    use crate::y2021::day18;

    #[test]
    fn test1() {
        assert_eq!(day18::puzzle1("src/y2021/day18/test.txt"), 4140);
        // assert_eq!(day18::puzzle1("src/y2021/day18/input.txt"), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(day18::puzzle2("src/y2021/day18/test.txt"), 1);
        assert_eq!(day18::puzzle2("src/y2021/day18/input.txt"), 1);
    }
}
