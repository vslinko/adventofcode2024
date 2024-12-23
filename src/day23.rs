use rustc_hash::{FxBuildHasher, FxHashSet};

const NODE_MUL: u32 = 256;
const CONN_MUL: u32 = 740;

macro_rules! parse_node_hash {
    ($input:expr, $a:expr, $b:expr) => {
        *$input.get_unchecked($a) as u32 * NODE_MUL + *$input.get_unchecked($b) as u32
    };
}

fn node_first_letter(hash: u32) -> u8 {
    (hash / NODE_MUL) as u8
}

fn node_second_letter(hash: u32) -> u8 {
    (hash % NODE_MUL) as u8
}

fn conn_hash(a: u32, b: u32) -> u32 {
    a * CONN_MUL + b
}

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut i = 0;
    let mut nodes = FxHashSet::with_capacity_and_hasher(30, FxBuildHasher::default());
    let mut connections = FxHashSet::with_capacity_and_hasher(30, FxBuildHasher::default());

    while i < input.len() {
        let left = parse_node_hash!(input, i, i + 1);
        let right = parse_node_hash!(input, i + 3, i + 4);

        nodes.insert(left);
        nodes.insert(right);

        connections.insert(conn_hash(left, right));
        connections.insert(conn_hash(right, left));

        i += 6;
    }

    let nodes = nodes.iter().collect::<Vec<_>>();

    nodes
        .iter()
        .filter(|&&&node1| node_first_letter(node1) == b't')
        .map(|&&node1| {
            nodes
                .iter()
                .filter(|&&&node2| {
                    connections.contains(&conn_hash(node1, node2))
                        && (node_first_letter(node2) != b't' || node2 > node1)
                })
                .map(|&&node2| {
                    nodes
                        .iter()
                        .filter(|&&&node3| {
                            node3 > node2
                                && connections.contains(&conn_hash(node1, node3))
                                && connections.contains(&conn_hash(node2, node3))
                                && (node_first_letter(node3) != b't' || node3 > node1)
                        })
                        .count()
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn part2(input: &str) -> String {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> String {
    let input = input.as_bytes();
    let mut i = 0;
    let mut nodes = FxHashSet::with_capacity_and_hasher(30, FxBuildHasher::default());
    let mut connections = FxHashSet::with_capacity_and_hasher(30, FxBuildHasher::default());

    while i < input.len() {
        let left = parse_node_hash!(input, i, i + 1);
        let right = parse_node_hash!(input, i + 3, i + 4);

        nodes.insert(left);
        nodes.insert(right);

        connections.insert(conn_hash(left, right));
        connections.insert(conn_hash(right, left));

        i += 6;
    }

    let mut nodes = nodes.iter().collect::<Vec<_>>();
    nodes.sort_unstable();

    let max_visited = nodes
        .iter()
        .enumerate()
        .map(|(i, &node)| {
            let mut visited = Vec::with_capacity(13);
            visited.push(node);

            nodes[i + 1..].iter().fold(visited, |mut visited, &other| {
                let connected_to_all_other = visited
                    .iter()
                    .all(|&v| connections.contains(&conn_hash(*v, *other)));

                if connected_to_all_other {
                    visited.push(other);
                }

                visited
            })
        })
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap_unchecked();

    let mut bytes = Vec::with_capacity(max_visited.len() * 3 - 1);
    bytes.push(node_first_letter(**max_visited.get_unchecked(0)));
    bytes.push(node_second_letter(**max_visited.get_unchecked(0)));

    String::from_utf8_unchecked(max_visited[1..].iter().fold(bytes, |mut bytes, &other| {
        bytes.push(b',');
        bytes.push(node_first_letter(*other));
        bytes.push(node_second_letter(*other));
        bytes
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const TEST_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_day23_part1() {
        assert_eq!(part1(&TEST_INPUT).to_string(), "7");
        let prod_input = read_to_string("./inputs/23.txt").unwrap();
        let prod_output = read_to_string("./outputs/23p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day23_part2() {
        assert_eq!(part2(&TEST_INPUT).to_string(), "co,de,ka,ta");
        let prod_input = read_to_string("./inputs/23.txt").unwrap();
        let prod_output = read_to_string("./outputs/23p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
