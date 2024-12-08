pub fn part1(input: &str) -> u16 {
    let (m, n, antennas_by_type) = parse(input);

    let mut grid = vec![0u16; (m * n) as usize];

    for antennas in antennas_by_type.iter().filter(|v| v.len() > 1) {
        for (i, a) in antennas.iter().enumerate() {
            for b in antennas.iter().skip(i + 1) {
                let x1 = a.0 + (b.0 - a.0) * 2;
                let y1 = a.1 + (b.1 - a.1) * 2;
                let x2 = b.0 + (a.0 - b.0) * 2;
                let y2 = b.1 + (a.1 - b.1) * 2;

                if x1 >= 0 && y1 >= 0 && y1 < n && x1 < m {
                    grid[(y1 * m + x1) as usize] = 1;
                }
                if x2 >= 0 && y2 >= 0 && y2 < n && x2 < m {
                    grid[(y2 * m + x2) as usize] = 1;
                }
            }
        }
    }

    grid.iter().sum()
}

pub fn part2(input: &str) -> u16 {
    let (m, n, antennas_by_type) = parse(input);

    let mut grid = vec![0u16; (m * n) as usize];

    for antennas in antennas_by_type.iter().filter(|v| v.len() > 1) {
        for (i, a) in antennas.iter().enumerate() {
            for b in antennas.iter().skip(i + 1) {
                {
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
                {
                    let dx = a.0 - b.0;
                    let dy = a.1 - b.1;

                    let mut rx = b.0 + dx;
                    let mut ry = b.1 + dy;

                    while rx >= 0 && ry >= 0 && ry < n && rx < m {
                        grid[(ry * m + rx) as usize] = 1;
                        rx += dx;
                        ry += dy;
                    }
                }
            }
        }
    }

    grid.iter().sum()
}

#[inline(always)]
fn vec_index(c: u8) -> usize {
    match c {
        b'0'..=b'9' => (c - b'0') as usize,
        b'A'..=b'Z' => (c - b'A' + 10) as usize,
        b'a'..=b'z' => (c - b'a' + 36) as usize,
        _ => panic!("Invalid character"),
    }
}

fn parse(input: &str) -> (i32, i32, Vec<Vec<(i32, i32)>>) {
    let mut antennas = vec![Vec::with_capacity(32); 62];
    let mut m = 0;
    let mut x = 0;
    let mut y = 0;

    for &char in input.as_bytes() {
        match char {
            b'\n' => {
                m = x;
                x = 0;
                y += 1;
            }
            b'.' => {
                x += 1;
            }
            c => {
                antennas[vec_index(c)].push((x, y));
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
