mod lib;
mod y2015;
mod y2016;
mod y2021;

fn main() {
    println!("{:?}", y2021::day20::puzzle1("src/y2021/day20/input.txt"));
    println!("{:?}", y2021::day20::puzzle2("src/y2021/day20/input.txt"));
}
