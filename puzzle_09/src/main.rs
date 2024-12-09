use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

fn checksum(blocks: &Vec<Option<usize>>) -> usize {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.and_then(|v| Some((i, v))))
        .map(|(i, b)| i * b)
        .sum()
}

fn convert_map_to_blocks(disk_map: &Vec<usize>) -> Vec<Option<usize>> {
    let mut result = vec![];
    for (i, m) in disk_map.iter().enumerate() {
        let is_empty = i % 2 == 1;
        let s = if is_empty { None } else { Some(i / 2) };
        result.extend(vec![s; *m]);
    }
    result
}

fn shift_one_to_left(blocks: &mut Vec<Option<usize>>) -> Option<()> {
    let first_none = blocks.iter().position(|o| o.is_none())?;
    let last_some = blocks.iter().rposition(|o| o.is_some())?;

    if first_none > last_some {
        return None;
    }

    blocks.swap(first_none, last_some);
    Some(())
}

fn compact(disk_map: &Vec<usize>) -> usize {
    let mut blocks = convert_map_to_blocks(disk_map);
    while let Some(_) = shift_one_to_left(&mut blocks) {}
    checksum(&blocks)
}

fn compact_files(disk_map: &Vec<usize>) -> usize {
    let mut disk_map_with_idx: Vec<(usize, Option<usize>)> = disk_map
        .iter()
        .enumerate()
        .map(|(i, f)| (*f, (i % 2 == 0).then_some(i / 2)))
        .collect();
    let files_to_reorder: Vec<usize> = disk_map_with_idx
        .iter()
        .rev()
        .filter_map(|(_, i)| *i)
        .collect();
    for f in files_to_reorder {
        let cur_pos = disk_map_with_idx
            .iter()
            .position(|(_, o)| o.is_some() && o.unwrap() == f)
            .unwrap();
        let cur_ele = disk_map_with_idx[cur_pos];

        if let Some(pos_free_space) = disk_map_with_idx
            .iter()
            .enumerate()
            .filter(|(j, _)| *j < cur_pos)
            .position(|(j, e)| j % 2 == 1 && e.0 >= cur_ele.0)
        {
            let inserted_block = vec![
                (0, None),
                cur_ele,
                (disk_map_with_idx[pos_free_space].0 - cur_ele.0, None),
            ];
            disk_map_with_idx[cur_pos] = (cur_ele.0, None);
            disk_map_with_idx.splice(pos_free_space..=pos_free_space, inserted_block);
        }
    }
    let blocks: Vec<Option<usize>> = disk_map_with_idx
        .iter()
        .flat_map(|(n, f)| vec![*f; *n])
        .collect();
    checksum(&blocks)
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let line = reader.lines().next().unwrap().unwrap();
    let mut disk_map: Vec<usize> = line
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect();
    let checksum_compact = compact(&disk_map);
    println!("Checksum compact: {}", checksum_compact);
    let checksum_compact_files = compact_files(&mut disk_map);
    println!("Checksum compact files: {}", checksum_compact_files);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_1() {
        let vec = "009981118882777333644655556600000000000000"
            .chars()
            .map(|c| c.to_string().parse::<usize>().ok())
            .collect();
        let result = checksum(&vec);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_checksum_2() {
        let vec = "0099811188827773336446555566.............."
            .chars()
            .map(|c| c.to_string().parse::<usize>().ok())
            .collect();
        let result = checksum(&vec);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_checksum_3() {
        let vec = "00992111777.44.333....5555.6666.....8888.."
            .chars()
            .map(|c| c.to_string().parse::<usize>().ok())
            .collect();
        let result = checksum(&vec);
        assert_eq!(result, 2858);
    }

    #[test]
    fn test_checksum_4() {
        let vec = vec![Some(1), Some(2), None, Some(3), None];
        let result = checksum(&vec);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_checksum_5() {
        let vec = "..9.1"
            .chars()
            .map(|c| c.to_string().parse::<usize>().ok())
            .collect();
        let result = checksum(&vec);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_convert_1() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = convert_map_to_blocks(&vec);
        assert_eq!(
            result,
            vec![
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                Some(2),
                Some(2),
                Some(2),
                Some(2),
                Some(2)
            ]
        );
    }

    #[test]
    fn test_shift_one() {
        let mut vec = vec![Some(1), None, Some(2), None];
        let res = shift_one_to_left(&mut vec);
        assert_eq!(res, Some(()));
        assert_eq!(vec, vec![Some(1), Some(2), None, None]);
    }

    #[test]
    fn test_shift_one_no_change() {
        let mut vec = vec![Some(1), Some(2), None];
        let res = shift_one_to_left(&mut vec);
        assert_eq!(res, None);
        assert_eq!(vec, vec![Some(1), Some(2), None]);
    }

    #[test]
    fn test_shift_one_no_none() {
        let mut vec = vec![Some(1), Some(2)];
        let res = shift_one_to_left(&mut vec);
        assert_eq!(res, None);
        assert_eq!(vec, vec![Some(1), Some(2)]);
    }

    #[test]
    fn test_compact_file_1() {
        let mut disk_map = vec![1, 3, 2];
        let res = compact_files(&mut disk_map); // [1, 0, 2, 1]
        assert_eq!(res, 3);
    }

    #[test]
    fn test_compact_file_2() {
        let mut disk_map = vec![2, 3, 4, 1, 2];
        let res = compact_files(&mut disk_map); // [2, 0, 2, 1, 4, 1, 2]
        assert_eq!(res, 36);
    }

    #[test]
    fn test_compact_file_3() {
        let mut disk_map = vec![2, 4, 2, 3, 2, 2, 2, 2];
        let res = compact_files(&mut disk_map); // [2, 0, 2, 0, 2, 0, 2, 3, 2, 2, 2, 2]
        assert_eq!(res, 46);
    }

    #[test]
    fn test_main() {
        let reader = BufReader::new(File::open("input.txt").unwrap());
        let line = reader.lines().next().unwrap().unwrap();
        let mut disk_map: Vec<usize> = line
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect();
        let checksum_compact = compact(&disk_map);
        assert_eq!(checksum_compact, 6344673854800);
        let checksum_compact_files = compact_files(&mut disk_map);
        assert_eq!(checksum_compact_files, 6360363199987);
    }
}
