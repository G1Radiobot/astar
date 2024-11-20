use std::cmp::PartialOrd;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::HashMap;
use point::Point;
use binary_heap::BinaryHeap;

///Private struct that contains a point and the cost to reach it.
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


///Generates the list of points reachable from the given starting point.   
///`Node` consists of a point, and the cost of reaching that point   
///Closed is a hashmap to make it really fast to check if a point is in it.
pub fn worm_search(start: Point, goal: Point, _max_cost: u16, tile_cost: HashMap<char, u16>, map: &Vec<Vec<char>>) -> Option<Vec<Point>> {
    let mut open = BinaryHeap::<Node>::with_capacity(60); //Loop while we can pop a node out out of open
    open.push(Node {
        point: start,
        prev_point: None,
        cost: 0
    });
    let mut closed = HashMap::with_capacity(60);

    if let Some(_) = loop {
        if let Some(node) = open.pop() {
            closed.insert(node.point, (node.cost, node.prev_point)); //insert the poped node into closed
            if node.point == goal {break Some(node.point)};

            for i in node.point.check_neighbors().iter() { //for each neigbor of our parent node.point(check neighbors handles the bounds check and returns a vec of valid neigbors)
                let (x, y) = i.get(); 
                if let Some(tile_cost) = tile_cost.get(&map[x][y]) { //get the terrain cost of transversing the tile (and panic if its missing) right now the tiles are just chars, might come up with a more complicated solution in the future
                    let total_cost = tile_cost + node.cost;
                    //TODO algorithim isn't taking lowest cost possible path. Probably a problem right here
                    //WARNING making node.cost > instead of >= causes an infinite loop
                    if closed.get(i).filter(|&closed_cost| { total_cost >= closed_cost.0 }).is_none() { //check if a given neigbor is already in closed, and if it is, if the path from the parent node is cheaper than the cost in closed
                        open.push(Node {
                            point: *i,
                            prev_point: Some(node.point),
                            cost: total_cost
                        });
                        }
                } else {
                    panic!("No tile cost exists for tile type {}", &map[x][y]);
                }
                
            }
        } else {
            break None;
        }
    } {
        let mut rtn = Vec::with_capacity(60);
        rtn.push(goal);
        let mut cur_point = goal;
        loop {
            if let Some(prev_node_tuple) = closed.get(&cur_point) {
                if let Some(prev_node) = prev_node_tuple.1 {
                    rtn.push(prev_node);
                    cur_point = prev_node;
                } else {
                    break
                }
            } else {
                let (x, y) = cur_point.get();
                panic!("Point: {{{}, {}}} is not present in closed.", x, y);
            }
        }
        Some(rtn)
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
        vec!['e','e','e','f','e','e','e','e','e','e'],
        vec!['e','e','e','f','f','e','e','e','e','e'],
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
        f e e e e w e e e e
        f e e e f w e e e e
        f e e e f w e e e e
        f e e e e w e e e e
        f e e e e w e e e e
        f e e e x e f e e e
         */

        let bob = worm_search(start, goal, 50, HashMap::from([
            (char::from('e'), 10),
            (char::from('w'), 1000),
            (char::from('f'), 20),
        ]), &test_vec);
        
        //println!("{:?}", bob.run());

        for i in bob.unwrap().into_iter() {
            let (x, y) = i.get();
            test_vec[x][y] = 'P';
        }
        for x in test_vec.into_iter() {
            for y in x.into_iter() {
                print!("{} ", y);
            }
            println!("");
        }
    }
}
