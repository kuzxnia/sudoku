use std::{collections::HashSet, env};

use ndarray::{array, s, Array1, Array2, Array};

fn solve(array: Array2<i32>) -> (bool, Array2<i32>) {
    if !array.iter().any(|&x| x == 0) {
        return (true, array);
    }

    let ((row, col), possibilities) = array
        .indexed_iter()
        .filter(|&(_, &elem)| elem == 0)
        .map(|(cord, _)| {
            let cord = (cord.0 as i32, cord.1 as i32);
            ((cord), get_possibilites(&array, cord))
        })
        .min_by(|a, b| a.1.len().cmp(&b.1.len()))
        .unwrap();

    for possibility in possibilities.iter() {
        if valid(&array, *possibility, (row, col)) {
            let mut new_array = array.clone();
            new_array[[row as usize, col as usize]] = *possibility;

            let (solved, new_array) = solve(new_array);
            if solved {
                return (true, new_array);
            }
        }
    }
    return (false, array);
}

fn valid(array: &Array2<i32>, num: i32, cord: (i32, i32)) -> bool {
    let (x, y) = cord;
    let square_x = x / 3 * 3;
    let square_y = y / 3 * 3;

    return !array.slice(s![x, 0..]).iter().any(|&x| x == num)
        || !array.slice(s![0.., y]).iter().any(|&x| x == num)
        || !array
            .slice(s![square_x..square_x + 3, square_y..square_y + 3])
            .iter()
            .any(|&x| x == num);
}

fn get_possibilites(array: &Array2<i32>, cord: (i32, i32)) -> Vec<i32> {
    return HashSet::from_iter(1..10)
        .difference(
            &get_elements_for_row_col_square(&array, cord)
                .into_iter()
                .collect::<HashSet<i32>>(),
        )
        .cloned()
        .collect::<Vec<i32>>();
}

fn get_elements_for_row_col_square(array: &Array2<i32>, cord: (i32, i32)) -> Vec<i32> {
    let (x, y) = cord;
    let square_x = x / 3 * 3;
    let square_y = y / 3 * 3;
    return array
        .slice(s![x, 0..])
        .into_iter()
        .chain(array.slice(s![0.., y]))
        .chain(
            &array
                .slice(s![square_x..square_x + 3, square_y..square_y + 3])
                .into_iter()
                .copied()
                .collect::<Array1<i32>>(),
        )
        .copied()
        .collect();
}


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let x = 0;
    let since_the_beginning = std::time::Instant::now();
    let array = array![
        [x, 5, x, x, x, x, x, x, 9],
        [x, x, 6, x, x, 8, 5, x, x],
        [x, x, 3, 4, 5, x, 2, x, x],
        [x, 6, x, 3, 9, 7, x, x, 4],
        [x, 9, x, 2, 6, 1, x, x, 5],
        [x, 2, x, x, x, x, x, x, 3],
        [x, x, 1, 9, 4, x, 7, x, x],
        [x, 3, x, x, x, x, x, x, 1],
        [x, x, 2, x, x, 5, 3, x, x],
    ];
    let array2 = [
        [x, 2, x, x, 6, x, x, x, x],
        [x, x, x, x, 2, x, 7, x, 1],
        [x, 3, x, 8, x, x, x, x, 4],
        [1, x, x, x, x, 7, x, x, 9],
        [x, 4, x, x, 8, x, x, 7, x],
        [6, x, x, 5, x, x, x, x, 2],
        [9, x, x, x, x, 6, x, 4, x],
        [5, x, 7, x, 3, x, x, x, x],
        [x, x, x, x, 7, x, x, 9, x],
    ];

    let (success, array) = solve(array);
    let end = since_the_beginning.elapsed();
    // println!("{}", success);
    for row in array.iter() {
        println!("{:?}", row);
    }
    println!("{}ms", end.as_millis());
    // println!("{}", Array::from_vec(get_possibilites(array, (0, 0))));
}
