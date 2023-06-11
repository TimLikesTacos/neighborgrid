use neighborgrid::*;

#[derive(PartialEq, Debug)]
pub enum LifeStage {
    Alive,
    Dead,
}
fn main() {
    use LifeStage::*;

    // Pattern for the "glider" in Conway's game of life.
    let glider = vec![
        vec![Dead, Alive, Dead, Dead, Dead],
        vec![Dead, Dead, Alive, Alive, Dead],
        vec![Dead, Alive, Alive, Dead, Dead],
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Dead, Dead, Dead, Dead],
    ];

    let second_gen_expected = vec![
        vec![Dead, Dead, Alive, Dead, Dead],
        vec![Dead, Dead, Dead, Alive, Dead],
        vec![Dead, Alive, Alive, Alive, Dead],
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Dead, Dead, Dead, Dead],
    ];

    let third_gen_expected = vec![
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Alive, Dead, Alive, Dead],
        vec![Dead, Dead, Alive, Alive, Dead],
        vec![Dead, Dead, Alive, Dead, Dead],
        vec![Dead, Dead, Dead, Dead, Dead],
    ];

    let forth_gen_expected = vec![
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Dead, Dead, Alive, Dead],
        vec![Dead, Alive, Dead, Alive, Dead],
        vec![Dead, Dead, Alive, Alive, Dead],
        vec![Dead, Dead, Dead, Dead, Dead],
    ];

    let fifth_gen_expected = vec![
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Dead, Alive, Dead, Dead],
        vec![Dead, Dead, Dead, Alive, Alive],
        vec![Dead, Dead, Alive, Alive, Dead],
        vec![Dead, Dead, Dead, Dead, Dead],
    ];

    let sixth_gen_expected = vec![
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Dead, Dead, Alive, Dead],
        vec![Dead, Dead, Dead, Dead, Alive],
        vec![Dead, Dead, Alive, Alive, Alive],
        vec![Dead, Dead, Dead, Dead, Dead],
    ];

    let seventh_gen_expected = vec![
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Dead, Alive, Dead, Alive],
        vec![Dead, Dead, Dead, Alive, Alive],
        vec![Dead, Dead, Dead, Alive, Dead],
    ];

    let eigth_gen_expected = vec![
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Dead, Dead, Dead, Alive],
        vec![Dead, Dead, Alive, Dead, Alive],
        vec![Dead, Dead, Dead, Alive, Alive],
    ];

    let ninth_gen_expected = vec![
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Dead, Dead, Alive, Dead],
        vec![Alive, Dead, Dead, Dead, Alive],
        vec![Dead, Dead, Dead, Alive, Alive],
    ];

    let tenth_gen_expected = vec![
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Dead, Dead, Dead, Alive],
        vec![Alive, Dead, Dead, Dead, Dead],
        vec![Alive, Dead, Dead, Alive, Alive],
    ];

    let eleventh_gen_expected = vec![
        vec![Dead, Dead, Dead, Dead, Alive],
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Dead, Dead, Dead, Dead, Dead],
        vec![Alive, Dead, Dead, Alive, Dead],
        vec![Alive, Dead, Dead, Dead, Alive],
    ];

    let gridoptions = GridOptions {
        wrap_x: true,
        wrap_y: true,
        ..Default::default()
    };

    let mut grid = Grid::new(glider, Some(gridoptions.clone())).unwrap();
    next_generation(&mut grid);

    assert_eq!(
        grid,
        Grid::new(second_gen_expected, Some(gridoptions.clone())).unwrap()
    );

    next_generation(&mut grid);

    assert_eq!(
        grid,
        Grid::new(third_gen_expected, Some(gridoptions.clone())).unwrap()
    );

    next_generation(&mut grid);

    assert_eq!(
        grid,
        Grid::new(forth_gen_expected, Some(gridoptions.clone())).unwrap()
    );

    next_generation(&mut grid);

    assert_eq!(
        grid,
        Grid::new(fifth_gen_expected, Some(gridoptions.clone())).unwrap()
    );

    next_generation(&mut grid);

    assert_eq!(
        grid,
        Grid::new(sixth_gen_expected, Some(gridoptions.clone())).unwrap()
    );

    next_generation(&mut grid);

    assert_eq!(
        grid,
        Grid::new(seventh_gen_expected, Some(gridoptions.clone())).unwrap()
    );

    next_generation(&mut grid);

    assert_eq!(
        grid,
        Grid::new(eigth_gen_expected, Some(gridoptions.clone())).unwrap()
    );

    next_generation(&mut grid);

    assert_eq!(
        grid,
        Grid::new(ninth_gen_expected, Some(gridoptions.clone())).unwrap()
    );

    next_generation(&mut grid);

    assert_eq!(
        grid,
        Grid::new(tenth_gen_expected, Some(gridoptions.clone())).unwrap()
    );

    next_generation(&mut grid);

    assert_eq!(
        grid,
        Grid::new(eleventh_gen_expected, Some(gridoptions.clone())).unwrap()
    );
}

fn next_generation(grid: &mut Grid<LifeStage>) {
    use LifeStage::*;

    let next_stage: Vec<_> = (0..grid.size())
        .into_iter()
        .map(|i| {
            let neighbors = grid.all_around_neighbors(i).unwrap();
            let count = neighbors
                .iter()
                .filter(|cell| *cell == &Some(&Alive))
                .count();
            match grid.get(i).unwrap() {
                Dead if count == 3 => Alive,
                Alive if count == 2 || count == 3 => Alive,
                _ => Dead,
            }
        })
        .collect();

    for (grid, next) in grid.iter_mut().zip(next_stage.into_iter()) {
        *grid = next;
    }
}
