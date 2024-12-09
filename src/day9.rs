pub fn part1(input: &str) -> i64 {
    // let a = std::time::Instant::now();
    let mut memory: Vec<i64> = Vec::with_capacity(100000);
    {
        let disk_map = input.trim();
        let mut file_id: i64 = 0;
        let bytes = disk_map.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let occupied_count = (bytes[i] - b'0') as usize;
            memory.extend(std::iter::repeat(file_id).take(occupied_count));
            file_id += 1;

            i += 1;

            if i < bytes.len() {
                let free_count = (bytes[i] - b'0') as usize;
                if free_count > 0 {
                    memory.extend(std::iter::repeat(-1).take(free_count));
                }
            }

            i += 1;
        }
    }
    // println!("a: {:?}", a.elapsed());

    // let b = std::time::Instant::now();
    let mut result: i64 = 0;
    {
        let mut left = 0;
        let mut right = memory.len() - 1;

        while left <= right {
            if memory[left] == -1 {
                while left < right && memory[right] == -1 {
                    right -= 1;
                }
                if right > left {
                    result += memory[right] * (left as i64);
                }
                right -= 1;
            } else {
                result += memory[left] * (left as i64);
            }
            left += 1;
        }
    }
    // println!("b: {:?}", b.elapsed());

    result
}

pub fn part2(input: &str) -> i64 {
    let disk_map = input.trim();
    let mut memory: Vec<(i64, i64)> = Vec::with_capacity(disk_map.len());

    {
        let mut file_id = 0;

        let bytes = disk_map.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let occupied_count = (bytes[i] - b'0') as i64;
            memory.push((file_id, occupied_count));
            file_id += 1;

            i += 1;

            if i < bytes.len() {
                let free_count = (bytes[i] - b'0') as i64;
                if free_count > 0 {
                    memory.push((-1, free_count));
                }
            }

            i += 1;
        }
    }

    let mut result = 0;
    {
        let mut pos = 0;
        let mut i = 0;
        let mut max_right = memory.len() - 1;

        while i < memory.len() {
            let memory_block = &memory[i];

            if memory_block.0 == -1 {
                let mut right = max_right;

                let mut file_met = false;
                while i < right {
                    if memory[right].0 >= 0 {
                        if !file_met {
                            max_right = right;
                            file_met = true;
                        }

                        if memory[right].1 <= memory_block.1 {
                            break;
                        }
                    }
                    right -= 1;
                }

                if right > i {
                    let file_memory_block = &memory[right];
                    for _ in 0..file_memory_block.1 {
                        result += pos * file_memory_block.0;
                        pos += 1;
                    }
                    memory[i].1 -= file_memory_block.1;
                    memory[right].0 = -1; // free memory of the moved file
                    if memory[i].1 > 0 {
                        // check this memory block again
                        continue;
                    }
                } else {
                    pos += memory_block.1;
                }
            } else {
                for _ in 0..memory_block.1 {
                    result += pos * memory_block.0;
                    pos += 1;
                }
            }
            i += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_day9_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_day9_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 2858);
    }
}
