use std::collections::HashSet;


#[derive(Copy, Clone,Debug,Hash,Eq,PartialEq)]
struct Coord {
    x: usize,
    y: usize
}

#[derive(Copy, Clone, Debug,)]
struct Node {
    traversable: bool,
    estm_cost: Option<f32>,
    cost: Option<i32>,
    from: Option<Coord>,
}



fn main() {
    let start = Coord{x: 0, y:1};
    let end = Coord{x: 3, y:3};
    let width = 4;
    let height = 4;
    let MAX_ITER = 100;

    let mut grid = vec![vec![Node{
        traversable: true,
        estm_cost: None,
        cost: None,
        from: None
    }; width]; height];

    //Set up traversables
    grid[2][2].traversable = false;

    grid[start.x][start.y].cost = Some(0i32);

    let mut open:HashSet<Coord> = HashSet::new();
    open.insert(start);

    for _i in 0..MAX_ITER {
        let coord_option = open.iter().next().cloned();
        if let Some(coord) = coord_option {

            println!("Studying {:#?}", coord);
            for dx in 0..3 {
                let x:i32 = (coord.x as i32) + dx - 1;
                if x < 0 {continue;}
                if x >= (width as i32)  {continue;}
                for dy in 0..3{
                    if dx == 1 && dy == 1 {continue;}
                    let y = (coord.y as i32) + dy -1;
                    if y < 0  {continue;}
                    if y >= (height as i32) {continue;}
                    if !grid[x as usize][y as usize].traversable  {continue;}
                    if grid[x as usize][y as usize].cost.is_some()  {continue;}
                    open.insert(Coord{x: x as usize, y: y as usize});
                }
            }

            grid[coord.x][coord.y].cost = Some(10i32);
            open.remove(&coord);

        } else {
            break;
        }
    }
    assert_eq!(open.len(), 0);

}