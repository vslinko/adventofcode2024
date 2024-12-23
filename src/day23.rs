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

struct Graph {
    adj_list: Vec<FxHashSet<usize>>,
    vertices: usize,
}

impl Graph {
    fn new(vertices: usize) -> Self {
        Graph {
            adj_list: vec![FxHashSet::with_hasher(FxBuildHasher::default()); vertices],
            vertices,
        }
    }

    fn add_edge(&mut self, v1: usize, v2: usize) {
        self.adj_list[v1].insert(v2);
        self.adj_list[v2].insert(v1);
    }

    fn find_maximal_cliques(&self) -> Vec<FxHashSet<usize>> {
        let mut cliques = Vec::new();
        let mut r = FxHashSet::with_hasher(FxBuildHasher::default()); // Current clique being built
        let mut p: FxHashSet<usize> = (0..self.vertices).collect(); // Potential vertices
        let mut x = FxHashSet::with_hasher(FxBuildHasher::default()); // Excluded vertices

        self.bron_kerbosch(&mut cliques, &mut r, &mut p, &mut x);
        cliques
    }

    fn bron_kerbosch(
        &self,
        cliques: &mut Vec<FxHashSet<usize>>,
        r: &mut FxHashSet<usize>,
        p: &mut FxHashSet<usize>,
        x: &mut FxHashSet<usize>,
    ) {
        // If no more vertices to process and no excluded vertices,
        // we found a maximal clique
        if p.is_empty() && x.is_empty() {
            if !r.is_empty() {
                cliques.push(r.clone());
            }
            return;
        }

        // Find pivot - vertex with maximum number of connections
        let pivot = self.find_pivot(p, x);
        let p_copy = p.clone();

        // For each vertex in P that's not connected to pivot
        for &v in p_copy.difference(&self.adj_list[pivot]) {
            // Add vertex to current clique
            r.insert(v);

            // Create new sets for recursive call
            let neighbors: FxHashSet<_> = self.adj_list[v].iter().cloned().collect();

            // Recursive call with updated sets
            let mut new_p: FxHashSet<_> = p.intersection(&neighbors).cloned().collect();
            let mut new_x: FxHashSet<_> = x.intersection(&neighbors).cloned().collect();

            self.bron_kerbosch(cliques, r, &mut new_p, &mut new_x);

            // Move v from P to X
            r.remove(&v);
            p.remove(&v);
            x.insert(v);
        }
    }

    fn find_pivot(&self, p: &FxHashSet<usize>, x: &FxHashSet<usize>) -> usize {
        let union: FxHashSet<_> = p.union(x).cloned().collect();

        // Find vertex with maximum connections to P
        union
            .iter()
            .max_by_key(|&&v| self.adj_list[v].intersection(p).count())
            .cloned()
            .unwrap_or(0)
    }
}

unsafe fn inner2(input: &str) -> String {
    let input = input.as_bytes();
    let mut i = 0;
    let mut nodes_ids: FxHashMap<usize, usize> =
        FxHashMap::with_capacity_and_hasher(30, FxBuildHasher::default());
    let mut ids_nodes: FxHashMap<usize, usize> =
        FxHashMap::with_capacity_and_hasher(30, FxBuildHasher::default());
    let mut connections = vec![];
    let mut node_id = 0;

    macro_rules! parse_node_hash {
        ($input:expr, $a:expr, $b:expr) => {
            *$input.get_unchecked($a) as usize * 256 + *$input.get_unchecked($b) as usize
        };
    }

    fn node_first_letter(hash: usize) -> u8 {
        (hash / 256) as u8
    }

    fn node_second_letter(hash: usize) -> u8 {
        (hash % 256) as u8
    }

    while i < input.len() {
        let left = parse_node_hash!(input, i, i + 1);
        let right = parse_node_hash!(input, i + 3, i + 4);

        if !nodes_ids.contains_key(&left) {
            nodes_ids.insert(left, node_id);
            ids_nodes.insert(node_id, left);
            node_id += 1;
        }
        if !nodes_ids.contains_key(&right) {
            nodes_ids.insert(right, node_id);
            ids_nodes.insert(node_id, right);
            node_id += 1;
        }

        connections.push((
            *nodes_ids.get(&left).unwrap_unchecked(),
            *nodes_ids.get(&right).unwrap_unchecked(),
        ));

        i += 6;
    }

    let mut graph = Graph::new(nodes_ids.len());
    for (a, b) in connections {
        graph.add_edge(a, b);
    }

    let cliques = graph.find_maximal_cliques();
    let max_clique = cliques.iter().max_by_key(|c| c.len()).unwrap_unchecked();

    let mut max_clique = max_clique
        .iter()
        .map(|&node_id| *ids_nodes.get(&node_id).unwrap_unchecked())
        .collect::<Vec<usize>>();

    max_clique.sort_unstable();

    let mut bytes = Vec::with_capacity(max_clique.len() * 3 - 1);
    bytes.push(node_first_letter(*max_clique.get_unchecked(0)));
    bytes.push(node_second_letter(*max_clique.get_unchecked(0)));

    String::from_utf8_unchecked(max_clique[1..].iter().fold(bytes, |mut bytes, &other| {
        bytes.push(b',');
        bytes.push(node_first_letter(other));
        bytes.push(node_second_letter(other));
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
