use neighborgrid::{Grid, GridOptions, Origin};
/// This is a demostration of how the itertors and the grid can be used in a sudoku puzzle.
/// The following code is not very efficient and organized, but it is done just to demostrate how it works and allows
/// for changing the coordinate or number to test different success or fails for placement.
fn main() {
    let sudoku_vec = vec![
        vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
        vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
        vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
        vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
        vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
        vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
        vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
        vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
        vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    let gridoptions = GridOptions {
        origin: Origin::UpperLeft,
        inverted_y: true,
        neighbor_ybased: false,
        ..GridOptions::default()
    };

    let sudoku = Grid::new(sudoku_vec, Some(gridoptions)).expect("Could not import the 2D vec");

    // Lets check if we can place an 8 in (row: 1, column:1) (zero indexed rows / cols)
    let coord = (1, 1);
    let number = 8;
    let mut placable = true;
    for (i, v) in sudoku.row_iter(coord).enumerate() {
        if v == &number {
            placable = false;
        }
        if !placable {
            println!(
                "Cannot place a {} in the row of {},{}. There is a {} in the #{} cell",
                number, coord.0, coord.1, number, i
            )
        }
    }
    if placable {
        println!("The row does not prevent placing the {}", number);
    }
    placable = true;

    for (i, v) in sudoku.col_iter(coord).enumerate() {
        if v == &number {
            placable = false;
        }
        if !placable {
            println!(
                "Cannot place a {} in the col of {},{}. There is a {} in the #{} cell",
                number, coord.0, coord.1, number, i
            )
        }
    }
    if placable {
        println!("The col does not prevent placing the {}", number);
    }
    placable = true;

    for (i, v) in sudoku.nrant_iter(3, coord).enumerate() {
        if v == Some(&number) {
            placable = false;
        }
        if !placable {
            println!(
                "Cannot place a {} in the 3x3 box of {},{}. There is a {} in the #{} cell",
                number, coord.0, coord.1, number, i
            )
        }
    }
    if placable {
        println!("The box does not prevent placing the {}", number);
        println!("we can place {} in coord {},{}", number, coord.0, coord.1);
    }
}
