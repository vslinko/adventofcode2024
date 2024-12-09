pub fn part1(input: &str) -> i64 {
    // let a = std::time::Instant::now();
    let disk_map = input.trim();
    let mut memory: Vec<i64> = Vec::with_capacity(100000);
    let mut file_id: i64 = 0;
    let mut chars = disk_map.chars();

    while let Some(occupied) = chars.next() {
        let occupied_count = occupied.to_digit(10).unwrap() as usize;
        memory.extend(std::iter::repeat(file_id).take(occupied_count));
        file_id += 1;

        if let Some(free) = chars.next() {
            let free_count = free.to_digit(10).unwrap() as usize;
            if free_count > 0 {
                memory.extend(std::iter::repeat(-1).take(free_count));
            }
        }
    }
    // println!("a: {:?}", a.elapsed());

    // let b = std::time::Instant::now();
    let mut result: i64 = 0;
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
    // println!("b: {:?}", b.elapsed());

    result
}

struct MemoryBlock {
    file_id: Option<i64>,
    size: i64,
}

impl MemoryBlock {
    fn new(file_id: Option<i64>, size: i64) -> Self {
        MemoryBlock { file_id, size }
    }
}

pub fn part2(input: &str) -> i64 {
    let disk_map = input.trim();
    let mut memory: Vec<MemoryBlock> = Vec::with_capacity(disk_map.len());

    {
        let mut file_id = 0;

        let mut chars = disk_map.chars();

        while let Some(occupied) = chars.next() {
            let occupied_count = occupied.to_digit(10).unwrap() as i64;
            memory.push(MemoryBlock::new(Some(file_id), occupied_count));
            file_id += 1;

            if let Some(free) = chars.next() {
                let free_count = free.to_digit(10).unwrap() as i64;
                if free_count > 0 {
                    memory.push(MemoryBlock::new(None, free_count));
                }
            }
        }
    }

    let mut result = 0;
    {
        let mut pos = 0;
        let mut i = 0;

        while i < memory.len() {
            let memory_block = &memory[i];

            if memory_block.file_id.is_none() {
                let mut right = memory.len() - 1;

                while i < right
                    && (memory[right].file_id.is_none() || memory[right].size > memory_block.size)
                {
                    right -= 1;
                }

                if right > i {
                    let file_memory_block = &memory[right];
                    let file_id = file_memory_block.file_id.unwrap();
                    for _ in 0..file_memory_block.size {
                        result += pos * file_id;
                        pos += 1;
                    }
                    memory[i].size -= file_memory_block.size;
                    memory[right].file_id = None; // free memory of the moved file
                    if memory[i].size > 0 {
                        // check this memory block again
                        continue;
                    }
                } else {
                    pos += memory_block.size;
                }
            } else {
                let file_id = memory_block.file_id.unwrap();
                for _ in 0..memory_block.size {
                    result += pos * file_id;
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
