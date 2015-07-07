extern crate type_printer;

// Problem #11
//
// In the 20×20 grid below, four numbers along a diagonal line have been marked in red.
//
// 08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
// 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
// 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
// 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
// 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
// 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
// 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
// 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
// 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
// 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
// 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
// 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
// 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
// 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
// 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
// 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
// 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
// 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
// 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
// 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
// The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
//
// What is the greatest product of four adjacent numbers
//  in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?

pub fn result() {
    println!("\nProblem #11 coming soon!");
    grid();
}

fn grid() {
  let grid: Vec<Vec<i32>> = vec![vec![1, 2], vec![2, 3]];

  let grid: Vec<Vec<i32>> = vec![
    vec![08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08],
    vec![49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00],
    vec![81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65],
    vec![52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91],
    vec![22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80],
    vec![24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50],
    vec![32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70],
    vec![67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21],
    vec![24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72],
    vec![21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95],
    vec![78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92],
    vec![16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57],
    vec![86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58],
    vec![19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40],
    vec![04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66],
    vec![88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69],
    vec![04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36],
    vec![20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16],
    vec![20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54],
    vec![01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48]
  ];

  let mut horizontal_sums: Vec<Vec<i32>> = vec![];
  for num in 0..grid.len() {
      let result = find_horizontal_sums(&grid[num]);
      horizontal_sums.push(result);
  }

  let highest_horizontal_sum = find_highest_in_grid(horizontal_sums);
  println!("Highest Horizontal Sum: {}", highest_horizontal_sum);


  let mut vertical_grid: Vec<Vec<i32>> = vec![];
  for num in 0..grid.len() {
    let vertical_row = vertical_row_extractor(&grid, 0);
    vertical_grid.push(vertical_row);
  }

  let mut vertical_sums: Vec<Vec<i32>> = vec![];
  for num in 0..grid.len() {
      let result = find_horizontal_sums(&vertical_grid[num]);
      vertical_sums.push (result);
  }

  let highest_vertical_sum = find_highest_in_grid(vertical_sums);
  println!("Highest Vertical Sum: {}", highest_vertical_sum);

  let mut diagonal_pairs: Vec<Vec<i32>> = vec![];
  let mut grid_index = 0;
  let mut row_index = 0;

  for outer in 0..(20 - 4) {
    for inner in 0..(20 - 4) {
      let mut diagonal_pair: Vec<i32> = vec![];
      let mut grid_index = outer;
      let mut row_index = inner;

      for index in 0..4 {
        let value = grid[grid_index][row_index].clone();
        diagonal_pair.push(value);
        grid_index += 1;
        row_index += 1;
      }

      diagonal_pairs.push(diagonal_pair);
    }
  }

  // So Now I need to sum them all making a grid
  // then find the highest in the grid
  println!("diagonal_pairs: {:?}", diagonal_pairs);
}

fn find_highest_in_grid(grid: Vec<Vec<i32>>) -> i32 {
    let mut highest_for_each: Vec<i32> = vec![];

    for mut row in grid {
        row.sort();
        let high: i32 = row.last().unwrap().clone();
        highest_for_each.push(high);
    }

    highest_for_each.last().unwrap().clone()
}

fn find_horizontal_sums(row: &Vec<i32>) -> Vec<i32> {
  let row_len = row.len();
  let mut lower_limit = 0;
  let mut upper_limit = 4;
  let mut pairs: Vec<Vec<i32>> = vec![];

  loop {

    {
      let four_pair = (lower_limit..upper_limit).map(|num| row[num]).collect::<Vec<i32>>();
      pairs.push(four_pair);
    }

    lower_limit += 1;
    upper_limit += 1;

    if upper_limit > row_len { break }
  }

  let mut sums: Vec<i32> = vec![];

  for pair in pairs {
    let result = pair.iter().fold(0, |acc, num| acc + num);
    sums.push(result);
  }

  sums
}

fn vertical_row_extractor(grid: &Vec<Vec<i32>>, index: usize) -> Vec<i32> {
    let mut vertical_row: Vec<i32> = vec![];

    for row in grid {
      vertical_row.push(row[index])
    }

    vertical_row
}
