use crate::lib;
use std::fmt;

#[derive(Debug, Clone)]
enum Num {
    R(usize),              // Regular number
    P(Box<Num>, Box<Num>), // Pair
    Nil,
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Num::R(n) => n.fmt(f),
            Num::P(l, r) => unsafe { write!(f, "[{},{}]", l, r) },
            Num::Nil => "nil".fmt(f),
        }
    }
}

impl Num {
    fn from_str(s: &str) -> Result<Num, String> {
        let n = Num::from_chars_recursive(&s.replace(" ", "").chars().collect::<Vec<char>>());
        Ok(n)
    }

    fn from_chars_recursive(chars: &[char]) -> Num {
        let mut left = Num::Nil;
        let mut right = Num::Nil;
        let mut current_depth: usize = 0;
        let mut i: usize = 0;
        while i < chars.len() && (matches!(left, Num::Nil) || matches!(right, Num::Nil)) {
            match chars[i] {
                '[' => {
                    current_depth += 1;
                    // current_depth == 1 means we are at the start of a new pair
                    if current_depth == 1 {
                        left = Num::from_chars_recursive(&chars[i + 1..]);
                    }
                }
                ',' => {
                    // current_depth == 1 means we are at the end of the left side of a pair
                    if current_depth == 1 {
                        right = Num::from_chars_recursive(&chars[i + 1..]);
                    }
                }
                ']' => {
                    current_depth -= 1;
                }
                c @ '0'..='9' => {
                    // current_depth == 0 means we are at the start of a new number
                    if current_depth == 0 {
                        let val = c.to_digit(10).unwrap() as usize;
                        return Num::R(val);
                    }
                }
                _ => (),
            }
            i += 1;
        }

        Num::P(Box::new(left), Box::new(right))
    }

    fn mag(&self) -> usize {
        match self {
            Num::R(v) => *v,
            Num::P(l, r) => 3 * l.mag() + 2 * r.mag(),
            Num::Nil => 0,
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

    let r = Num::from_str(&input[7]).expect("isgood");

    unsafe {
        println!("{}", r);
    }

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
