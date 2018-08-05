struct Loc {
    row: usize,
    col: usize,
}

fn find_unassigned_location(grid: &[Vec<u8>]) -> Option<Loc> {
    for (row, row_grid) in grid.iter().enumerate() {
        for (col, &item) in row_grid.iter().enumerate() {
            if item == 0 { return Some(Loc{ row, col }); }
        }
    }
    None
}

fn used_in_row(grid: &[Vec<u8>], row: usize, num: u8) -> bool {
    grid[row].iter().any(|&x| x == num)
}

fn used_in_col(grid: &[Vec<u8>], col: usize, num: u8) -> bool {
    grid.iter().any(|x| x[col] == num)
}

fn used_in_box(grid: &[Vec<u8>], row_start: usize, col_start: usize, num: u8) -> bool {
    grid.iter().skip(row_start).take(3).any(|x| x.iter().skip(col_start).take(3).any(|&y| y == num))
}

fn is_safe(grid: &[Vec<u8>], row: usize, col: usize, num: u8) -> bool {
    !used_in_row(grid, row, num) &&
    !used_in_col(grid, col, num) &&
    !used_in_box(grid, row / 3 * 3, col / 3 * 3, num)
}

fn solve_sudoku(grid: &mut Vec<Vec<u8>>) -> bool {
    let loc = match find_unassigned_location(grid) {
        Some(loc) => loc,
        None => return true,
    };

    for num in 1..10 {
        if is_safe(grid, loc.row, loc.col, num) {
            grid[loc.row][loc.col] = num;

            if solve_sudoku(grid) { return true; }

            grid[loc.row][loc.col] = 0;
        }
    }
    false
}

fn print_grid(grid: &[Vec<u8>]) {
    for row_grid in grid.iter() {
        println!("{:?}", row_grid);
    }
    println!();
}

fn main() {
    let mut grid = vec![
        vec![3, 0, 6, 5, 0, 8, 4, 0, 0],
        vec![5, 2, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 8, 7, 0, 0, 0, 0, 3, 1],
        vec![0, 0, 3, 0, 1, 0, 0, 8, 0],
        vec![9, 0, 0, 8, 6, 3, 0, 0, 5],
        vec![0, 5, 0, 0, 9, 0, 6, 0, 0],
        vec![1, 3, 0, 0, 0, 0, 2, 5, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 7, 4],
        vec![0, 0, 5, 2, 0, 6, 3, 0, 0],
    ];

    print_grid(&grid);

    if solve_sudoku(&mut grid) {
        print_grid(&grid);
    } else {
        println!("No solution exists");
    }
}
