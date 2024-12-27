use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day09)]
fn parse_input(inp: &str) -> Vec<usize> {
    inp.lines().flat_map(|l| l.chars().filter_map(|c| c.to_digit(10).map(|d| d as usize))).collect()
}

fn construct_disk_map(inp: &Vec<usize>, ) -> Vec<Option<usize>> {
    let mut disk_map: Vec<Option<usize>> = Vec::new();
    let mut file_id = 0;

    for (i, x) in inp.iter().enumerate() {
        let length = *x;
        if i % 2 == 0 {
            // File chunk
            for _ in 0..length {
                disk_map.push(Some(file_id));
            }
            file_id += 1;
        } else {
            // Free space chunk
            for _ in 0..length {
                disk_map.push(None);
            }
        }
    }
    disk_map
}

fn optimize_disk_map(disk_map: &mut Vec<Option<usize>>) {
    let mut i =0;
    while i < disk_map.len() {
        if disk_map[i].is_none() {
            let val = loop {
                if let Some(item) = disk_map.pop(){
                    if let Some(v) = item {
                        break v;
                    }
                    else {
                        continue;
                    }
                }
            };
            disk_map[i] = Some(val);
        }
        i += 1;
    }
}


fn calculate_checksum(disk_map: &Vec<Option<usize>>) -> usize {
    disk_map.iter()
        .enumerate()
        .map(|(i, &val)| i * val.expect("All should be Some at this point"))
        .sum()
}

fn merge_free_spans(mut free_spans: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    free_spans.sort_by_key(|x| x.0);

    let mut merged: Vec<(usize, usize)> = Vec::new();

    for (start, len) in free_spans {
        let end = start + len;
        if merged.is_empty() || merged.last().unwrap().0 + merged.last().unwrap().1 < start {
            merged.push((start, len));
        } else {
            let (last_start, last_len) = merged.pop().unwrap();
            let last_end = last_start + last_len;
            let new_end = end.max(last_end);
            merged.push((last_start, new_end - last_start));
        }
    }

    merged
}

fn find_leftmost_free_span(
    free_spans: &[(usize, usize)],
    file_start: usize,
    file_length: usize
) -> Option<usize> {
    free_spans
        .iter()
        .enumerate()
        .filter(|(_, &(span_start, span_len))| span_start + span_len <= file_start)
        .filter(|(_, &(_, span_len))| span_len >= file_length)
        .map(|(idx, _)| idx)
        .next()
}

fn compact_disk_map(disk_map: &mut Vec<Option<usize>>) {
    let mut file_spans: Vec<(usize, usize, usize)> = Vec::new();
    let mut i = 0;
    while i < disk_map.len() {
        if let Some(fid) = disk_map[i] {
            let start = i;
            let mut j = i + 1;
            while j < disk_map.len() && disk_map[j] == Some(fid) {
                j += 1;
            }
            let length = j - start;
            file_spans.push((start, length, fid));
            i = j;
        } else {
            i += 1;
        }
    }

    file_spans.sort_by(|a, b| b.2.cmp(&a.2));

    for &(file_start, file_length, file_id) in &file_spans {
        let mut free_spans: Vec<(usize, usize)> = Vec::new();
        {
            let mut maybe_start = None;
            for (idx, block) in disk_map.iter().enumerate() {
                match (block, maybe_start) {
                    (None, None) => maybe_start = Some(idx),
                    (Some(_), Some(s)) => {
                        free_spans.push((s, idx - s));
                        maybe_start = None;
                    },
                    _ => {}
                }
            }
            if let Some(s) = maybe_start {
                free_spans.push((s, disk_map.len() - s));
            }
        }
        free_spans = merge_free_spans(free_spans);

        if let Some(span_idx) = find_leftmost_free_span(&free_spans, file_start, file_length) {
            let (span_start, _) = free_spans[span_idx];

            for offset in 0..file_length {
                disk_map[span_start + offset] = Some(file_id);
            }
            
            for old_offset in file_start..file_start + file_length {
                disk_map[old_offset] = None;
            }
        }
    }
}



fn calculate_new_checksum(disk_map: &Vec<Option<usize>>) -> usize {
    disk_map.iter()
        .enumerate()
        .filter_map(|(i, &val)| val.map(|file_id| i * file_id))
        .sum()
}

#[aoc(day09, part1)]
fn part1(inp: &Vec<usize>) -> usize {
    let mut disk_map = construct_disk_map(inp);
    optimize_disk_map(&mut disk_map);
    calculate_checksum(&disk_map)
}

#[aoc(day09, part2)]
fn part2(inp: &Vec<usize>) -> usize {
    let mut disk_map = construct_disk_map(inp);
    compact_disk_map(&mut disk_map);
    println!("{:?}", disk_map);
    calculate_new_checksum(&disk_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_parse() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(input, vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2]);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 1928);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(TEST_INPUT);
        
        assert_eq!(part2(&input), 2858);
    }
}