use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use serde::{Serialize};
use serde_json::{Value};


#[wasm_bindgen]
#[derive(Copy, Clone,Debug,Hash,Eq,PartialEq,Serialize)]
struct Coord {
    x: usize,
    y: usize
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug,Serialize)]
struct Node {
    traversable: bool,
    estm_cost: Option<f32>,
    cost: Option<i32>,
    from: Option<Coord>,
}


#[wasm_bindgen]
pub fn dijkstra(sx:usize, sy:usize, width: usize, height: usize, map:&[u8]) -> String {
   let start = Coord{x: sx, y: sy};
    let MAX_ITER = width*height;

    let mut grid = vec![vec![Node{
        traversable: true,
        estm_cost: None,
        cost: None,
        from: None,
    }; width]; height];

    //Set up traversables
    for x in 0..width{
        for y in 0..height{
            grid[x][y].traversable = map[x*height + y] == 1;
        }
    }

    grid[start.x][start.y].cost = Some(0i32);

    let mut open:HashSet<Coord> = HashSet::new();
    open.insert(Coord{x:start.x, y: start.y});

    for _i in 0..MAX_ITER {
        let mut sorted_open: Vec<Coord> = open
            .clone()
            .into_iter()
            .collect();
        sorted_open
            .sort_by_key(|a,|
                grid[a.x][a.y].cost.unwrap_or(std::i32::MAX)
            );
        let coord_option = sorted_open.get(0);

        if let Some(coord) = coord_option {
            let mut lowest_cost = grid[coord.x][coord.y].cost.unwrap_or(std::i32::MAX);
            let mut lowest_from = grid[coord.x][coord.y].from;
            let mut added_from_this_node:Vec<Coord> = vec!{};
            for dx in 0..3 {
                let x:i32 = (coord.x as i32) + dx - 1;
                if x < 0 {continue;}
                if x >= (width as i32)  {continue;}
                for dy in 0..3{
                    if (dx-1)*(dx-1) +  (dy-1)*(dy-1) != 1 {continue;}
                    let y = (coord.y as i32) + dy -1;
                    if y < 0  {continue;}
                    if y >= (height as i32) {continue;}
                    if !grid[x as usize][y as usize].traversable  {continue;}
                    if let Some(from_cost) = grid[x as usize][y as usize].cost {
                        let cost = from_cost + 1;
                        if cost < lowest_cost {
                            lowest_cost = cost;
                            lowest_from = Some(Coord{x:x as usize, y:y as usize});
                        }
                    }else{
                        added_from_this_node.push(Coord{x: x as usize, y: y as usize});
                        open.insert(Coord{x: x as usize, y: y as usize});
                    }
                }
            }
            grid[coord.x][coord.y].cost = Some(lowest_cost);
            grid[coord.x][coord.y].from = lowest_from;
            for added_coord in added_from_this_node{
                grid[added_coord.x][added_coord.y].from = Some(*coord);
                grid[added_coord.x][added_coord.y].cost = Some(lowest_cost + 1);
            }
            open.remove(&coord);

        } else {
            break;
        }
    }
    return serde_json::json!(grid).to_string();

}