use std::collections::HashSet;
use std::fs::read_to_string;

use itertools::Itertools;

use puzzle_04::char_grid::CharGrid;
use puzzle_04::vector::BoundVector2D;

fn get_anti_nodes(
    vec_a: &BoundVector2D,
    vec_b: &BoundVector2D,
) -> impl Iterator<Item = BoundVector2D> {
    let vec_a_s = vec_a.to_sized().unwrap();
    let vec_b_s = vec_b.to_sized().unwrap();
    let dir = (vec_a_s.0 - vec_b_s.0, vec_a_s.1 - vec_b_s.1);
    let anti_a = *vec_a + dir;
    let anti_b = *vec_b + (-1 * dir.0, -1 * dir.1);

    anti_a.into_iter().chain(anti_b)
}

fn get_harmonic_anti_nodes(vec_a: &BoundVector2D, vec_b: &BoundVector2D) -> HashSet<BoundVector2D> {
    let mut anti_nodes = HashSet::from([*vec_a, *vec_b]);
    let vec_a_s = vec_a.to_sized().unwrap();
    let vec_b_s = vec_b.to_sized().unwrap();
    let dir = (vec_a_s.0 - vec_b_s.0, vec_a_s.1 - vec_b_s.1);

    let mut v = *vec_a;
    while let Some(vec) = v + dir {
        anti_nodes.insert(vec);
        v = vec;
    }

    v = *vec_b;
    let dir_neg = (-1 * dir.0, -1 * dir.1);
    while let Some(vec) = v + dir_neg {
        anti_nodes.insert(vec);
        v = vec;
    }

    anti_nodes
}

fn get_all_anti_nodes(grid: &CharGrid) -> (HashSet<BoundVector2D>, HashSet<BoundVector2D>) {
    let mut anti_nodes = HashSet::new();
    let mut harmonic_anti_nodes = HashSet::new();
    let groups = grid.iter_indices().into_group_map_by(|idx| grid.get(*idx));
    for (char, indices) in groups.iter() {
        if char.is_none() || *char == Some(&'.') {
            continue;
        }
        for comb in indices.iter().combinations(2) {
            anti_nodes.extend(get_anti_nodes(comb[0], comb[1]));
            harmonic_anti_nodes.extend(get_harmonic_anti_nodes(comb[0], comb[1]));
        }
    }
    (anti_nodes, harmonic_anti_nodes)
}

fn main() {
    let content = read_to_string("input.txt").unwrap();
    let grid = CharGrid::from_string(&content).unwrap();
    let (anti_nodes, harm_anti_nodes) = get_all_anti_nodes(&grid);
    println!("Number of anti nodes: {}", &anti_nodes.len());
    println!("Number of harmonic anti nodes: {}", &harm_anti_nodes.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let grid = CharGrid::from_string(
            "............
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
............",
        )
        .unwrap();

        let result = get_all_anti_nodes(&grid);
        assert_eq!(result.0.len(), 14);
    }

    #[test]
    fn test_main() {
        let content = read_to_string("input.txt").unwrap();
        let grid = CharGrid::from_string(&content).unwrap();
        let (anti_nodes, harm_anti_nodes) = get_all_anti_nodes(&grid);
        assert_eq!(&anti_nodes.len(), &295);
        assert_eq!(&harm_anti_nodes.len(), &1034);
    }
}
