
use std::cmp::PartialOrd;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::fmt;

struct PointBuilder(usize, usize);

impl PointBuilder {
    pub fn new(&self, x: usize, y: usize) -> Point {
        Point(x, y, self.0, self.1)
    }
}

///Coordinate struct where the first two fields are x and y, and the second two fields are x_bound and y_bound.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Point(usize, usize, usize, usize);

impl Point {
    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).zip(self.1.checked_add(other.1)).map(|result|{
            let (x, y) = result;
            Self(x, y, self.2, self.3)
        }).filter(|&result| {
            let Point(x, y, _, _) = result;
            x < self.2 || y < self.3
        })
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).zip(self.1.checked_sub(other.1)).map(|result|{
            let (x, y) = result;
            Self(x, y, self.2, self.3)
        })
    }

    pub fn get(self) -> (usize, usize) {
        (self.0, self.1)
    }

    ///Returns a vector of the points in each cardinal direction. Returns None if no direction has an in-bounds point.
    pub fn check_neighbors(self) -> Vec<Point> {
        let mut rtn = Vec::with_capacity(4);
        if let Some(result) = self.north() {rtn.push(result);}
        if let Some(result) = self.south() {rtn.push(result);}
        if let Some(result) = self.east() {rtn.push(result);}
        if let Some(result) = self.west() {rtn.push(result);}
        if rtn.is_empty() {panic!("Point {{{}, {}}} has no neigbors in bounds: {{{}, {}}}.", self.0, self.1, self.2, self.3)}
        rtn
    }

    pub fn north(self) -> Option<Self> {
        self.checked_add(Point(0, 1, 0, 0))
    }

    pub fn south(self) -> Option<Self> {
        self.checked_sub(Point(0, 1, 0, 0))
    }

    pub fn east(self) -> Option<Self> {
        self.checked_add(Point(1, 0, 0, 0))
    }

    pub fn west(self) -> Option<Self> {
        self.checked_sub(Point(1, 0, 0, 0))
    }
}

impl Add for Point {
    type Output = Self;
   
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2, self.3)
    }
}

impl Sub for Point {
    type Output = Self;
   
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2, self.3)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

impl Default for Point {
    fn default() -> Self {
        Point(0, 0, 0, 0)
    }
}

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
///Node consists of a point, and the cost of reaching that point   
///closed is a hashmap to make it really fast to check if a point is in it. Let me know if this is dumb
///
///TODO Doesn't need to be a distinct object, refactor into just a function
pub fn worm_search(start: Point, max_cost: u8, tile_cost: HashMap<usize, u8>, map: &Vec<Vec<usize>>) -> Vec<Point> {
    //Loop while we can pop a node out out of open
    let mut open = Vec::<Node>::with_capacity(60);
    open.push(Node {
        point: start,
        cost: 0
    });
    let mut closed = HashMap::with_capacity(60);

    while let Some(node) = open.pop() {
        //insert the poped node into closed
        closed.insert(node.point, node.cost);

        //for each neigbor of our parent node.point(check neighbors handles the bounds check and returns a vec of valid neigbors)
        for i in node.point.check_neighbors().iter() {
            //check if a given neigbor is already in closed, and if it is, if the path from the parent node is cheaper than the cost in closed
            if closed.get(i).filter(|&closed_cost| { node.cost >= *closed_cost }).is_none() {
                let (x, y) = i.get();

                //get the terrain cost of transversing the tile (and panic if its missing)   
                //right now the tiles are just integers, might come up with a more complicated solution in the future
                if let Some(tile_cost) = tile_cost.get(&map[x][y]) {
                    //get the cost of the new node by adding the terrain cost to the cost of the parent node
                    let total_cost = tile_cost + node.cost;

                    //if the cost of the new node is less than the maximum allowed cost of the search, push it to open so we can check its neigbors
                    if total_cost < max_cost {
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
    //return the nodes in closed as a vec
    let mut rtn = Vec::with_capacity(60);
    for i in closed.into_iter() {
        rtn.push(i.0)
    }
    rtn
}


#[cfg(test)]
mod test {
    use super::HashMap;
    use super::worm_search;
    use super::PointBuilder;

    #[test]
    fn add_sub_point() {
        let point_builder = PointBuilder(10, 10);
        let start = point_builder.new(4, 0);
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
            test_vec[i.0][i.1] = 9;
        }
        for x in test_vec.into_iter() {
            for y in x.into_iter() {
                print!("{} ", y);
            }
            println!("");
        }
    }
}
