use std::cmp::PartialOrd;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::HashMap;
use point::Point;

///Private struct that contains a point and the cost to reach it.
struct Node {
    point: Point,
    cost: u8
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
pub fn worm_search(start: Point, max_cost: u8, tile_cost: HashMap<usize, u8>, map: &Vec<Vec<usize>>) -> Vec<Point> {
    let mut open = Vec::<Node>::with_capacity(60); //Loop while we can pop a node out out of open
    open.push(Node {
        point: start,
        cost: 0
    });
    let mut closed = HashMap::with_capacity(60);

    while let Some(node) = open.pop() {
        closed.insert(node.point, node.cost); //insert the poped node into closed

        for i in node.point.check_neighbors().iter() { //for each neigbor of our parent node.point(check neighbors handles the bounds check and returns a vec of valid neigbors)
            if closed.get(i).filter(|&closed_cost| { node.cost >= *closed_cost }).is_none() { //check if a given neigbor is already in closed, and if it is, if the path from the parent node is cheaper than the cost in closed
                let (x, y) = i.get(); 

                if let Some(tile_cost) = tile_cost.get(&map[x][y]) { //get the terrain cost of transversing the tile (and panic if its missing) right now the tiles are just integers, might come up with a more complicated solution in the future
                    let total_cost = tile_cost + node.cost;

                    if total_cost < max_cost {  //if the cost of the new node is less than the maximum allowed cost of the search, push it to open so we can check its neigbors
                        open.push(Node {
                            point: *i,
                            cost: total_cost
                        });
                    //if the cost of the new node is equal to the maximum cost, then there isn't any point in checking it's neigbors so just push straight to closed
                    //if the node was already in closed but we found a better path, insert will automatically override the old value
                    //NOTE: if we end up having terrain that costs 0 to cost, then this else if should be removed
                    } else if total_cost == max_cost {
                        closed.insert(*i, total_cost);
                    }
                } else {
                    panic!("No tile cost exists for tile type {}", &map[x][y]);
                }
            }
        }
    }
    let mut rtn = Vec::with_capacity(60); //return the nodes in closed as a vec
    for i in closed.into_iter() {
        rtn.push(i.0)
    }
    rtn
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
        let mut test_vec: Vec<Vec<usize>> = vec![
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,2,2,0,0,0,0,0],
        vec![0,1,1,1,1,1,1,0,0,0],
        vec![2,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],];
        /* The map corrected. X is start
        0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 1 0 0 0 0
        0 0 0 0 0 1 0 0 0 0
        0 0 0 0 2 1 0 0 0 0
        0 0 0 0 2 1 0 0 0 0
        0 0 0 0 0 1 0 0 0 0
        0 0 0 0 0 1 0 0 0 0
        0 0 0 0 x 0 2 0 0 0
         */

        let bob = worm_search(start, 5, HashMap::from([
            (0, 1),
            (1, 99),
            (2, 2),
        ]), &test_vec);
        
        //println!("{:?}", bob.run());

        for i in bob.into_iter() {
            let (x, y) = i.get();
            test_vec[x][y] = 9;
        }
        for x in test_vec.into_iter() {
            for y in x.into_iter() {
                print!("{} ", y);
            }
            println!("");
        }
    }
}
