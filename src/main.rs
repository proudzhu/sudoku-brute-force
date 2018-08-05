struct Loc {
    row: usize,
    col: usize,
}

fn find_unassigned_location(grid: &Vec<Vec<u8>>) -> Option<Loc> {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 0 { return Some(Loc{ row, col }); }
        }
    }
    None
}

fn used_in_row(grid: &Vec<Vec<u8>>, row: usize, num: u8) -> bool {
    for col in 0..grid[row].len() {
        if grid[row][col] == num { return true; }
    }
    false
}

fn used_in_col(grid: &Vec<Vec<u8>>, col: usize, num: u8) -> bool {
    for row in 0..grid.len() {
        if grid[row][col] == num { return true; }
    }
    false
}

fn used_in_box(grid: &Vec<Vec<u8>>, row_start: usize, col_start: usize, num: u8) -> bool {
    for row in 0..3 {
        for col in 0..3 {
            if grid[row+row_start][col+col_start] == num { return true; }
        }
    }
    false
}

fn is_safe(grid: &Vec<Vec<u8>>, row: usize, col: usize, num: u8) -> bool {
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

fn print_grid(grid: &Vec<Vec<u8>>) {
    for row in 0..grid.len() {
        println!("{:?}", grid[row]);
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
