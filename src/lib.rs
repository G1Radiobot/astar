use std::cmp::PartialOrd;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::HashMap;
use point::Point;
use binary_heap::BinaryHeap;
use rand::*;
use std::ops::{Add, Mul};
use rand;

///Private struct that contains a point and the cost to reach it.
#[derive(Clone, Copy)] 
struct Node {
    point: Point,
    prev_point: Option<Point>,
    cost_with_heur: u32,
    cost_without_heur: u32
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost_with_heur.cmp(&other.cost_with_heur)
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost_with_heur == other.cost_with_heur
    }
}

pub fn manhattan_heuristic(start: &Point, goal: &Point) -> u32 {
    let (sx, sy) = start.get();
    let (gx, gy) = goal.get();
    let rtn = sx.abs_diff(gx).add(sy.abs_diff(gy)).mul(100) as u32;
    rtn
}

pub fn no_heuristic(_: &Point, _: &Point) -> u32 {
    0
}

pub fn random_tiebreaker(_: &Point, _: &Point) -> u32 {
    thread_rng().gen_range(1..5)
}

pub fn no_tiebreaker(_: &Point, _: &Point) -> u32 {
    0
}

///Todo Update this description
///Calculates the shortest path between the starting point and the goal.   
///If heuristic == 0, then this is just Dijtstras. Arguments for the heuristic should be the starting point and goal.
pub fn worm_search(
    start: Point, 
    goal: Point, 
    tile_costs: HashMap<char, u32>, 
    map: &Vec<Vec<char>>, 
    heuristic: fn(&Point, &Point) -> u32, 
    tiebreaker: fn(&Point, &Point) -> u32
) -> Option<Vec<Point>> {

    let mut open = BinaryHeap::<Node>::with_capacity(60);
    open.push(Node {
        point: start,
        prev_point: None,
        cost_with_heur: 0,
        cost_without_heur: 0
    });
    let mut closed = HashMap::with_capacity(60);
    let mut path: Vec<Point>;
    let mut test = 0;
    if loop {
        test = test + 1;
        if let Some(cur_node) = open.pop() {
            if !closed.contains_key(&cur_node.point) {
                closed.insert(cur_node.point, cur_node);
                if cur_node.point == goal {
                    break true
                }
                for i in cur_node.point.check_neighbors() {
                    if !closed.contains_key(&i) {
                        let (x, y) = i.get();
                        let total_cost_without_heur;
                        let total_cost_with_heur;
                        if let Some(move_cost) = tile_costs.get(&map[x][y]) {
                            if let Some(cost_before_heur) = cur_node.cost_without_heur.checked_add(*move_cost) {
                                total_cost_without_heur = cost_before_heur;
                                total_cost_with_heur = total_cost_without_heur + heuristic(&cur_node.point, &goal) + tiebreaker(&cur_node.point, &goal);
                            } else {continue}
                        } else {
                            panic!("Terrain type {} is missing from cost hashmap.", &map[x][y]);
                        }
                        open.push(Node {
                            point: i,
                            prev_point: Some(cur_node.point),
                            cost_with_heur: total_cost_with_heur,
                            cost_without_heur: total_cost_without_heur
                        })
                    }
                }
            }
        } else {
            break false
        }
    } {
        let mut cur_point = goal;
        path = vec![goal];
        while let Some(prev_point) = closed.get(&cur_point).and_then(|closed_node| {
            closed_node.prev_point
        }) {
            cur_point = prev_point;
            path.push(cur_point);
        }
        println!("{}", test);
        Some(path)
    } else {
        println!("{}", test);
        None
    }
}


#[cfg(test)]
mod test {
    use super::HashMap;
    use super::worm_search;
    use super::Point;
    use super::manhattan_heuristic;
    #[allow(unused_imports)]
    use super::no_heuristic;
    #[allow(unused_imports)]
    use super::no_tiebreaker;
    use super::random_tiebreaker;
    
    #[test]
    fn add_sub_point() {
        let point_builder = Point::builder(10, 10);
        let start = point_builder.build(4, 0);
        let goal = point_builder.build(5, 8);
        let mut test_vec: Vec<Vec<char>> = vec![
        vec!['f','f','f','f','f','f','f','f','f','f'],
        vec!['e','e','e','e','e','e','e','e','e','e'],
        vec!['e','e','e','e','e','e','e','e','e','e'],
        vec!['e','e','e','f','e','f','e','e','e','e'],
        vec!['e','e','e','f','f','f','e','e','e','e'],
        vec!['e','w','w','w','w','w','w','e','e','e'],
        vec!['f','e','e','e','e','e','e','e','e','e'],
        vec!['e'; 10],
        vec!['e'; 10],
        vec!['e'; 10],];
        /* The map corrected. X is start, G is goal
        f e e e e e e e e e
        f e e e e G e e e e
        f e e e e e e e e e
        f e e e e w e e e e
        f e e f f w e e e e
        f e e e f w e e e e
        f e e f f w e e e e
        f e e e e w e e e e
        f e e e e w e e e e
        f e e e x e f e e e
         */
        let bob = worm_search(
            start, 
            goal, 
            HashMap::from([
                (char::from('e'), 100),
                (char::from('w'), u32::MAX),
                (char::from('f'), 200),
            ]), 
            &test_vec, 
            manhattan_heuristic,
            random_tiebreaker
        );
        
        for i in bob.unwrap().into_iter() {
            
            let (x, y) = i.get();
            test_vec[x][y] = '*';
        }
        for x in test_vec.into_iter() {
            for y in x.into_iter() {
                print!("{} ", y);
            }
            println!("");
        }
    }
}
