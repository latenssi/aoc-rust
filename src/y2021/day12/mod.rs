use std::collections::HashSet;

use crate::lib;

type Edge = (String, String);

fn path_count(
    node: &String,
    graph: &Vec<Edge>,
    avoid: HashSet<String>,
    skipped: bool,
    path: Vec<String>,
    paths: &mut HashSet<String>,
) -> usize {
    let mut path = path;
    path.push(node.clone());

    if node == "end" {
        let r = path.join(",");
        paths.insert(r) as usize
    } else {
        let mut alt_paths = 0;

        let initial_neighbours = graph
            .iter()
            .filter(|e| e.0 == *node || e.1 == *node)
            .map(|e| if e.0 == *node { &e.1 } else { &e.0 });

        let mut avoid = avoid.clone();

        // Clone before inserting
        let avoid_2 = avoid.clone();

        if node != "start" && node.to_uppercase() != *node && avoid.insert(node.clone()) && !skipped
        {
            // node was avoided for the first time
            // split (duplicate) paths when small cave is avoided
            // to path where it is avoided and a path where it is not
            alt_paths = initial_neighbours
                .clone()
                .filter(|n| *n != "start" && !avoid_2.contains(*n))
                .map(|n| path_count(n, graph, avoid_2.clone(), true, path.clone(), paths))
                .fold(0, |acc, c| acc + c);
        }

        initial_neighbours
            .filter(|n| *n != "start" && !avoid.contains(*n))
            .map(|n| path_count(n, graph, avoid.clone(), skipped, path.clone(), paths))
            .fold(0, |acc, c| acc + c)
            + alt_paths
    }
}

pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let edges: Vec<Edge> = input
        .iter()
        .map(|s| s.split("-").map(|s| String::from(s)).collect())
        .map(|v: Vec<String>| (v[0].clone(), v[1].clone()))
        .collect();

    let mut paths: HashSet<String> = HashSet::new();
    let start = String::from("start");
    path_count(&start, &edges, HashSet::new(), true, vec![], &mut paths)
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let edges: Vec<Edge> = input
        .iter()
        .map(|s| s.split("-").map(|s| String::from(s)).collect())
        .map(|v: Vec<String>| (v[0].clone(), v[1].clone()))
        .collect();

    let mut paths: HashSet<String> = HashSet::new();
    let start = String::from("start");

    path_count(&start, &edges, HashSet::new(), false, vec![], &mut paths)
}

#[cfg(test)]
mod tests {
    use crate::y2021::day12;

    #[test]
    fn test1() {
        assert_eq!(day12::puzzle1("src/y2021/day12/test.txt"), 10);
        assert_eq!(day12::puzzle1("src/y2021/day12/test2.txt"), 19);
        assert_eq!(day12::puzzle1("src/y2021/day12/test3.txt"), 226);
        assert_eq!(day12::puzzle1("src/y2021/day12/input.txt"), 3497);
    }

    #[test]
    fn test2() {
        assert_eq!(day12::puzzle2("src/y2021/day12/test.txt"), 36);
        assert_eq!(day12::puzzle2("src/y2021/day12/test2.txt"), 103);
        assert_eq!(day12::puzzle2("src/y2021/day12/test3.txt"), 3509);
        // assert_eq!(day12::puzzle2("src/y2021/day12/input.txt"), 93686);
    }
}
