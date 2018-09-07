#[derive(Clone)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

use Direction::*;

impl Direction {
    fn opposite(&self, other: &Direction) -> bool {
        match (self, other) {
            (&NORTH, &SOUTH) | (&SOUTH, &NORTH) => true,
            (&WEST, &EAST) | (&EAST, &WEST) => true,
            _ => false,
        }
    }
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let arr = arr.to_vec();
    reduc(arr)
}

fn reduc(arr: Vec<Direction>) -> Vec<Direction> {
    for i in 1..arr.len() {
        if arr[i - 1].opposite(&arr[i]) {
            let mut arr = arr.clone();
            arr.remove(i - 1);
            arr.remove(i - 1);
            return reduc(arr);
        }
    }
    arr
}
