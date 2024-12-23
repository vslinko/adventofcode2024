use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

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

unsafe fn bron_kerbosch(
    connections: &Vec<FxHashSet<usize>>,
    cliques: &mut Vec<FxHashSet<usize>>,
    r: &mut FxHashSet<usize>,
    p: &mut FxHashSet<usize>,
    x: &mut FxHashSet<usize>,
) {
    if p.is_empty() && x.is_empty() {
        if !r.is_empty() {
            cliques.push(r.clone());
        }
        return;
    }

    let pivot = *p
        .union(x)
        .max_by_key(|&&v| connections[v].intersection(p).count())
        .unwrap_unchecked();

    for &v in p.clone().difference(&connections[pivot]) {
        r.insert(v);

        let mut new_p = p.intersection(&connections[v]).cloned().collect();
        let mut new_x = x.intersection(&connections[v]).cloned().collect();

        bron_kerbosch(connections, cliques, r, &mut new_p, &mut new_x);

        r.remove(&v);
        p.remove(&v);
        x.insert(v);
    }
}

unsafe fn inner2(input: &str) -> String {
    const NODE_MUL: usize = 256;
    const NODES: usize = 520;
    const NODE_CONNECTIONS: usize = 13;

    let input = input.as_bytes();
    let mut i = 0;
    let mut nodes_ids: FxHashMap<usize, usize> =
        FxHashMap::with_capacity_and_hasher(NODES, FxBuildHasher::default());
    let mut ids_nodes = Vec::with_capacity(NODES);
    let mut connections =
        vec![
            FxHashSet::with_capacity_and_hasher(NODE_CONNECTIONS, FxBuildHasher::default());
            NODES
        ];

    macro_rules! parse {
        ($input:expr, $a:expr, $b:expr) => {
            *$input.get_unchecked($a) as usize * NODE_MUL + *$input.get_unchecked($b) as usize
        };
    }

    macro_rules! create_id {
        ($node:expr) => {
            match nodes_ids.get(&$node) {
                Some(&id) => id,
                None => {
                    let id = ids_nodes.len();
                    nodes_ids.insert($node, id);
                    ids_nodes.push($node);
                    id
                }
            }
        };
    }

    while i < input.len() {
        let left = parse!(input, i, i + 1);
        let right = parse!(input, i + 3, i + 4);

        let left_id = create_id!(left);
        let right_id = create_id!(right);

        connections.get_unchecked_mut(left_id).insert(right_id);
        connections.get_unchecked_mut(right_id).insert(left_id);

        i += 6;
    }

    let cliques = {
        let mut cliques = Vec::new();
        let mut r = FxHashSet::with_hasher(FxBuildHasher::default());
        let mut p: FxHashSet<usize> = (0..ids_nodes.len()).collect();
        let mut x = FxHashSet::with_hasher(FxBuildHasher::default());
        bron_kerbosch(&connections, &mut cliques, &mut r, &mut p, &mut x);
        cliques
    };

    let max_clique = {
        let mut max_clique = cliques
            .iter()
            .max_by_key(|c| c.len())
            .unwrap_unchecked()
            .iter()
            .map(|&node_id| *ids_nodes.get_unchecked(node_id))
            .collect::<Vec<_>>();
        max_clique.sort_unstable();
        max_clique
    };

    macro_rules! get_byte {
        (1, $node:expr) => {
            ($node / NODE_MUL) as u8
        };
        (2, $node:expr) => {
            ($node % NODE_MUL) as u8
        };
    }

    let bytes = {
        let mut bytes = Vec::with_capacity(max_clique.len() * 3 - 1);
        bytes.push(get_byte!(1, max_clique.get_unchecked(0)));
        bytes.push(get_byte!(2, max_clique.get_unchecked(0)));

        max_clique[1..].iter().fold(bytes, |mut bytes, &other| {
            bytes.push(b',');
            bytes.push(get_byte!(1, other));
            bytes.push(get_byte!(2, other));
            bytes
        })
    };

    String::from_utf8_unchecked(bytes)
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
