pub fn part1(input: &str) -> u16 {
    let (m, n, antennas_by_type) = parse(input);

    let mut grid = vec![0u16; (m * n) as usize];

    for antennas in &antennas_by_type {
        for a in antennas {
            for b in antennas {
                if a == b {
                    continue;
                }

                let rx = a.0 + (b.0 - a.0) * 2;
                let ry = a.1 + (b.1 - a.1) * 2;

                if rx >= 0 && ry >= 0 && ry < n && rx < m {
                    grid[(ry * m + rx) as usize] = 1;
                }
            }
        }
    }

    grid.iter().sum()
}

pub fn part2(input: &str) -> u16 {
    let (m, n, antennas_by_type) = parse(input);

    let mut grid = vec![0u16; (m * n) as usize];

    for antennas in &antennas_by_type {
        for a in antennas {
            for b in antennas {
                if a == b {
                    continue;
                }

                let dx = b.0 - a.0;
                let dy = b.1 - a.1;

                let mut rx = a.0 + dx;
                let mut ry = a.1 + dy;

                while rx >= 0 && ry >= 0 && ry < n && rx < m {
                    grid[(ry * m + rx) as usize] = 1;
                    rx += dx;
                    ry += dy;
                }
            }
        }
    }

    grid.iter().sum()
}

// 48-57, 65-90, 97-122
fn vec_index(c: &u8) -> usize {
    match c {
        48 => 0,
        49 => 1,
        50 => 2,
        51 => 3,
        52 => 4,
        53 => 5,
        54 => 6,
        55 => 7,
        56 => 8,
        57 => 9,
        65 => 10,
        66 => 11,
        67 => 12,
        68 => 13,
        69 => 14,
        70 => 15,
        71 => 16,
        72 => 17,
        73 => 18,
        74 => 19,
        75 => 20,
        76 => 21,
        77 => 22,
        78 => 23,
        79 => 24,
        80 => 25,
        81 => 26,
        82 => 27,
        83 => 28,
        84 => 29,
        85 => 30,
        86 => 31,
        87 => 32,
        88 => 33,
        89 => 34,
        90 => 35,
        97 => 36,
        98 => 37,
        99 => 38,
        100 => 39,
        101 => 40,
        102 => 41,
        103 => 42,
        104 => 43,
        105 => 44,
        106 => 45,
        107 => 46,
        108 => 47,
        109 => 48,
        110 => 49,
        111 => 50,
        112 => 51,
        113 => 52,
        114 => 53,
        115 => 54,
        116 => 55,
        117 => 56,
        118 => 57,
        119 => 58,
        120 => 59,
        121 => 60,
        122 => 61,
        _ => panic!("Invalid character: {}", c),
    }
}

fn parse(input: &str) -> (i32, i32, Vec<Vec<(i32, i32)>>) {
    let mut antennas = vec![Vec::new(); 62];
    let mut m = 0;
    let mut x = 0;
    let mut y = 0;

    for char in input.as_bytes() {
        match char {
            b'\n' => {
                m = x;
                x = 0;
                y += 1;
            }
            b'.' => {
                x += 1;
            }
            char => {
                antennas[vec_index(char)].push((x, y));
                x += 1;
            }
        }
    }

    let n = if x == 0 { y } else { y + 1 };

    (m, n, antennas)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 34);
    }
}
