use std::cmp::PartialOrd;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::HashMap;
use point::Point;
use binary_heap::BinaryHeap;

///Private struct that contains a point and the cost to reach it.
#[derive(Clone, Copy)] 
struct Node {
    point: Point,
    prev_point: Option<Point>,
    cost: u16
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

///Todo Update this description
///Generates the list of points reachable from the given starting point.   
///`Node` consists of a point, and the cost of reaching that point   
///Closed is a hashmap to make it really fast to check if a point is in it.
pub fn worm_search(start: Point, goal: Point, _max_cost: u16, tile_costs: HashMap<char, u16>, map: &Vec<Vec<char>>) -> Option<Vec<Point>> {
    let mut open = BinaryHeap::<Node>::with_capacity(60);
    open.push(Node {
        point: start,
        prev_point: None,
        cost: 0
    });
    let mut closed = HashMap::with_capacity(60);
    let mut path: Vec<Point>;
    if loop {
        if let Some(cur_node) = open.pop() {
            if !closed.contains_key(&cur_node.point) {
                closed.insert(cur_node.point, cur_node);
                if cur_node.point == goal {
                    break true
                }
                for i in cur_node.point.check_neighbors() {
                    if !closed.contains_key(&i) {
                        let (x, y) = cur_node.point.get();
                        let total_cost;
                        if let Some(move_cost) = tile_costs.get(&map[x][y]) {
                            total_cost = cur_node.cost + move_cost;
                        } else {
                            panic!("Terrain type {} is missing from cost hashmap.", &map[x][y]);
                        }
                        open.push(Node {
                            point: i,
                            prev_point: Some(cur_node.point),
                            cost: total_cost
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
        Some(path)
    } else {
        None
    }
}


#[cfg(test)]
mod test {
    use super::HashMap;
    use super::worm_search;
    use super::Point;

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

        let bob = worm_search(start, goal, 50, HashMap::from([
            (char::from('e'), 1),
            (char::from('w'), 1000),
            (char::from('f'), 2),
        ]), &test_vec);
        
        //println!("{:?}", bob.run());
        let mut it = 0;
        for i in bob.unwrap().into_iter() {
            
            let (x, y) = i.get();
            test_vec[x][y] = char::from_digit(it, 10).unwrap();
            it += 1;
        }
        for x in test_vec.into_iter() {
            for y in x.into_iter() {
                print!("{} ", y);
            }
            println!("");
        }
    }
}
