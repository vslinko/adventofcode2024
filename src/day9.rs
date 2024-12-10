pub fn part1(input: &str) -> i64 {
    let mut numbers: Vec<i64> = input
        .trim_end()
        .bytes()
        .map(|b| (b - b'0') as i64)
        .collect();

    let mut result = 0;
    let mut pos = 0;
    let mut left = 0;
    let mut right = numbers.len() - 1;
    let mut left_id = 0;
    let mut right_id = (numbers.len() / 2) as i64;

    while left <= right {
        if numbers[left] > 0 {
            result += (pos + pos + numbers[left] - 1) * numbers[left] / 2 * left_id;
            pos += numbers[left];
            left_id += 1;
        }

        left += 1;

        if left > right {
            break;
        }

        let mut remaining = numbers[left];

        while remaining > 0 && left <= right {
            let process_count = if remaining < numbers[right] {
                remaining
            } else {
                numbers[right]
            };
            result += (pos + pos + process_count - 1) * process_count / 2 * right_id;
            remaining -= process_count;
            numbers[right] -= process_count;

            if numbers[right] == 0 {
                right -= 2;
                right_id -= 1;
            }

            pos += process_count;
        }

        left += 1;
    }

    result
}

pub fn part2(input: &str) -> i64 {
    let disk_map = input.trim_end();
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

            if i >= bytes.len() {
                break;
            }

            let free_count = (bytes[i] - b'0') as i64;
            if free_count > 0 {
                memory.push((-1, free_count));
            }

            i += 1;
        }
    }

    let mut result = 0;
    {
        let mut pos = 0;
        let mut left = 0;
        let mut max_right = memory.len() - 1;

        while left <= max_right {
            let memory_block = &memory[left];

            if memory_block.1 == 0 {
                left += 1;
                continue;
            }

            if memory_block.0 == -1 {
                let mut right = max_right;

                let mut file_met = false;
                while left < right {
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

                if right > left {
                    let file_memory_block = &memory[right];
                    result += (pos + pos + file_memory_block.1 - 1) * file_memory_block.1 / 2
                        * file_memory_block.0;
                    pos += file_memory_block.1;
                    memory[left].1 -= file_memory_block.1;
                    memory[right].0 = -1; // free memory of the moved file
                    if memory[left].1 == 0 {
                        left += 1;
                    }
                    // check this memory block again
                } else {
                    pos += memory_block.1;
                    left += 1;
                }
            } else {
                result += (pos + pos + memory_block.1 - 1) * memory_block.1 / 2 * memory_block.0;
                pos += memory_block.1;
                left += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day9_part1() {
        let prod_input = read_to_string("./inputs/9.txt").unwrap();
        let prod_output = read_to_string("./outputs/9p1.txt").unwrap();
        assert_eq!(part1("2333133121414131402"), 1928);
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day9_part2() {
        let prod_input = read_to_string("./inputs/9.txt").unwrap();
        let prod_output = read_to_string("./outputs/9p2.txt").unwrap();
        assert_eq!(part2("2333133121414131402"), 2858);
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
