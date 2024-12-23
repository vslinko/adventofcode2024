use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

const NODE_MUL: usize = 256;
const NODES: usize = 520;
const T_NODES: usize = 20;
const NODE_CONNECTIONS: usize = 13;

macro_rules! parse {
    ($input:expr, $a:expr, $b:expr) => {
        *$input.get_unchecked($a) as usize * NODE_MUL + *$input.get_unchecked($b) as usize
    };
}

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut i = 0;
    let mut t_nodes: FxHashSet<usize> =
        FxHashSet::with_capacity_and_hasher(T_NODES, FxBuildHasher::default());
    let mut nodes_ids: FxHashMap<usize, usize> =
        FxHashMap::with_capacity_and_hasher(NODES, FxBuildHasher::default());
    let mut id_counter = 0;
    let mut connections =
        vec![
            FxHashSet::with_capacity_and_hasher(NODE_CONNECTIONS, FxBuildHasher::default());
            NODES
        ];

    macro_rules! create_id {
        ($node:expr) => {
            match nodes_ids.get(&$node) {
                Some(&id) => id,
                None => {
                    let id = id_counter;
                    nodes_ids.insert($node, id);
                    id_counter += 1;
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

        if *input.get_unchecked(i) == b't' {
            t_nodes.insert(left_id);
        }
        if *input.get_unchecked(i + 3) == b't' {
            t_nodes.insert(right_id);
        }

        connections.get_unchecked_mut(left_id).insert(right_id);
        connections.get_unchecked_mut(right_id).insert(left_id);

        i += 6;
    }

    let mut result = Vec::with_capacity(2090);

    t_nodes.iter().for_each(|&node| {
        let mut cliques = Vec::new();
        let mut r = FxHashSet::with_hasher(FxBuildHasher::default());
        r.insert(node);
        let mut p = connections.get_unchecked(node).clone();
        let mut x = FxHashSet::with_hasher(FxBuildHasher::default());

        bron_kerbosch(&connections, &mut cliques, &mut r, &mut p, &mut x);

        cliques.iter_mut().for_each(|clique| match clique.len() {
            3 => {
                let clique = clique.iter().collect::<Vec<_>>();
                result.push(hash3(
                    **clique.get_unchecked(0),
                    **clique.get_unchecked(1),
                    **clique.get_unchecked(2),
                ));
            }
            n if n > 3 => {
                clique.remove(&node);
                clique.iter().tuple_combinations().for_each(|(a, b)| {
                    result.push(hash3(node, *a, *b));
                });
            }
            _ => {}
        });
    });

    result.sort_unstable();
    result.dedup();
    result.len()
}

pub fn part2(input: &str) -> String {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> String {
    let input = input.as_bytes();
    let mut i = 0;
    let mut nodes_ids = FxHashMap::with_capacity_and_hasher(NODES, FxBuildHasher::default());
    let mut ids_nodes = Vec::with_capacity(NODES);
    let mut connections =
        vec![
            FxHashSet::with_capacity_and_hasher(NODE_CONNECTIONS, FxBuildHasher::default());
            NODES
        ];

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
        let mut p = (0..ids_nodes.len()).collect();
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

        max_clique.iter().skip(1).fold(bytes, |mut bytes, &other| {
            bytes.push(b',');
            bytes.push(get_byte!(1, other));
            bytes.push(get_byte!(2, other));
            bytes
        })
    };

    String::from_utf8_unchecked(bytes)
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
        .max_by_key(|&&v| connections.get_unchecked(v).intersection(p).count())
        .unwrap_unchecked();

    p.clone()
        .difference(connections.get_unchecked(pivot))
        .for_each(|&v| {
            r.insert(v);

            let mut new_p = p
                .intersection(&connections.get_unchecked(v))
                .cloned()
                .collect();
            let mut new_x = x
                .intersection(&connections.get_unchecked(v))
                .cloned()
                .collect();

            bron_kerbosch(connections, cliques, r, &mut new_p, &mut new_x);

            r.remove(&v);
            p.remove(&v);
            x.insert(v);
        });
}

fn hash3(mut a: usize, mut b: usize, mut c: usize) -> usize {
    if a > b {
        (a, b) = (b, a);
    }
    if b > c {
        (b, c) = (c, b);
        if a > b {
            (a, b) = (b, a);
        }
    }

    a + b * NODES + c * NODES * NODES
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
