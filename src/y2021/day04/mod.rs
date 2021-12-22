use crate::lib;
use std::collections::HashSet;

const GRID_SIZE: usize = 5;

#[derive(Debug)]
struct Board {
    numbers: [usize; GRID_SIZE * GRID_SIZE],
    marked: [bool; GRID_SIZE * GRID_SIZE],
}

#[derive(Debug)]
struct Game {
    numbers: Vec<usize>,
    boards: Vec<Board>,
    next_number_idx: usize,
    wins: Vec<(usize, usize, u8)>, // 1 = row, 2 = col
}

fn buf_to_board_numbers(buf: &Vec<String>) -> [usize; GRID_SIZE * GRID_SIZE] {
    let mut board_numbers = [0; GRID_SIZE * GRID_SIZE];
    for (i, n) in buf.join(" ").split_whitespace().enumerate() {
        board_numbers[i] = n.parse().unwrap();
    }
    board_numbers
}

fn parse_game(input: &Vec<String>) -> Game {
    let game_numbers = input[0]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];

    let mut buf: Vec<String> = vec![];

    for l in &input[1..] {
        if l == "" {
            continue;
        }

        buf.push(l.to_string());

        if buf.len() >= GRID_SIZE {
            boards.push(Board {
                numbers: buf_to_board_numbers(&buf),
                marked: [false; GRID_SIZE * GRID_SIZE],
            });
            buf.clear()
        }
    }

    Game {
        numbers: game_numbers,
        boards: boards,
        next_number_idx: 0,
        wins: vec![],
    }
}

fn game_round(game: &mut Game, number: usize) {
    for i in 0..game.boards.len() {
        let board = &mut game.boards[i];
        for j in 0..board.numbers.len() {
            if board.numbers[j] == number {
                board.marked[j] = true;
            }
        }
    }
}

fn game_check_round(game: &mut Game) -> Option<HashSet<usize>> {
    let mut winners: HashSet<usize> = HashSet::new();

    for i in 0..game.boards.len() {
        let board = &game.boards[i];
        for j in 0..GRID_SIZE {
            let mut row_ok = true;
            let mut col_ok = true;

            for k in 0..GRID_SIZE {
                row_ok &= board.marked[j * GRID_SIZE + k]; // j = row
                col_ok &= board.marked[j + k * GRID_SIZE]; // j = col
            }

            if row_ok && game.wins.iter().find(|w| w.0 == i && w.1 == j && w.2 == 1) == None {
                game.wins.push((i, j, 1));
                winners.insert(i);
            }

            if col_ok && game.wins.iter().find(|w| w.0 == i && w.1 == j && w.2 == 2) == None {
                game.wins.push((i, j, 2));
                winners.insert(i);
            }
        }
    }

    if winners.len() > 0 {
        return Some(winners);
    }

    None
}

impl Iterator for Game {
    type Item = (usize, HashSet<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.next_number_idx..self.numbers.len() {
            let number = self.numbers[i];
            game_round(self, number);
            if i >= 4 {
                if let Some(winners) = game_check_round(self) {
                    self.next_number_idx = i + 1;
                    return Some((number, winners));
                }
            }
        }
        None
    }
}

pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let mut game = parse_game(&input);

    if let Some((number, winners)) = game.next() {
        for i in winners.iter() {
            let board = &game.boards[*i];
            let sum_not_marked = board
                .marked
                .iter()
                .zip(board.numbers.iter())
                .filter(|x| !*x.0)
                .fold(0, |v, x| v + x.1);
            return number * sum_not_marked;
        }
    }

    0
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let mut game = parse_game(&input);

    let mut all_winners: HashSet<usize> = HashSet::new();
    let mut last_winner_sum: usize = 0;

    while let Some((number, winners)) = game.next() {
        for i in winners.iter() {
            if all_winners.insert(*i) {
                let board = &game.boards[*i];
                let sum_not_marked = board
                    .marked
                    .iter()
                    .zip(board.numbers.iter())
                    .filter(|x| !*x.0)
                    .fold(0, |v, x| v + x.1);
                last_winner_sum = number * sum_not_marked;
            }
        }
    }

    last_winner_sum
}

#[cfg(test)]
mod tests {
    use crate::y2021::day04;

    #[test]
    fn test1() {
        assert_eq!(day04::puzzle1("src/y2021/day04/test.txt"), 4512);
        assert_eq!(day04::puzzle1("src/y2021/day04/input.txt"), 6592);
    }

    #[test]
    fn test2() {
        assert_eq!(day04::puzzle2("src/y2021/day04/test.txt"), 1924);
        assert_eq!(day04::puzzle2("src/y2021/day04/input.txt"), 31755);
    }
}
