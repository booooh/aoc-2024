
use common::read_lines;
use itertools::Itertools;

fn part1() {
    let input = read_lines("./day09/input").unwrap().next().unwrap();
    let repr = input.chars().map(|c| c.to_digit(10).unwrap());
    let mut disk: Vec<i32> = vec![];

    for (id, mut file) in repr.chunks(2).into_iter().enumerate() {
        let file_len = file.next().unwrap();
        for _ in 0..file_len {
            disk.push(id as i32);
        }
        if let Some(free_space_len) = file.next() {
            for _ in 0..free_space_len {
                disk.push(-1);
            }
        }
    }

    // move items back
    let mut content_index = disk.len() - 1;
    let mut free_space_index = 0;

    while free_space_index < content_index {
        if disk[free_space_index] == -1 {
            disk[free_space_index] = disk[content_index]; // move content to free space
            disk[content_index] = -1; // mark as free space

            // move on to next content
            while disk[content_index] == -1 {
                content_index -= 1;
            }
        }

        // either way, move to next spot
        free_space_index += 1
    }

    let mut checksum: i64 = 0;
    for (idx, file_id) in disk.iter().enumerate() {
        if *file_id != -1 {
            checksum += (*file_id as i64) * (idx as i64)
        }
    }

    println!("{:?}", checksum);
}

fn part2() {
    let input = read_lines("./day09/input").unwrap().next().unwrap();
    let repr = input.chars().map(|c| c.to_digit(10).unwrap());

    #[derive(Debug, Clone)]
    struct Block {
        file_id: i32,
        file_len: u32,
    }
    let mut disk: Vec<Block> = vec![];

    for (file_id, mut file) in repr.chunks(2).into_iter().enumerate() {
        let file_len = file.next().unwrap();
        disk.push(Block {
            file_id: file_id as i32,
            file_len,
        });
        if let Some(free_space_len) = file.next() {
            disk.push(Block {
                file_id: -1,
                file_len: free_space_len,
            });
        }
    }

    // move items back
    let mut content_index = disk.len() - 1;

    // iterate on the block from largest to smallest
    while content_index > 0 {
        let content_len = disk[content_index].file_len;
        let mut free_space_index = 0;

        // iterate until we find an empty space that's big enough
        while disk[free_space_index].file_id != -1 || disk[free_space_index].file_len < content_len
        {
            free_space_index += 1;
        }

        // that's it we cant move any more things
        if free_space_index < content_index {
            // we found a spot, check if we need to split it
            if disk[free_space_index].file_len > content_len {
                disk.insert(
                    free_space_index + 1,
                    Block {
                        file_id: -1,
                        file_len: disk[free_space_index].file_len - content_len,
                    },
                );
                content_index += 1; // since we inserted an element, the index shifts
            }

            // move the file, and mark the previous location as free
            disk[free_space_index] = disk[content_index].clone();
            disk[content_index].file_id = -1;
        } else {
            // we couldn't move this file, try the next one
            content_index -= 1;
        }

        // try the next file
        while disk[content_index].file_id == -1 {
            content_index -= 1;
        }
    }

    let mut checksum: i64 = 0;
    let mut num_bytes = 0;
    for block in disk.iter() {
        if block.file_id != -1 {
            for idx in num_bytes..num_bytes + block.file_len {
                checksum += (block.file_id as i64) * (idx as i64)
            }
        }
        num_bytes += block.file_len;
    }

    println!("{:?}", checksum);
}
fn main() {
    part1();
    part2()
}
