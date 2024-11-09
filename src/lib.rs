

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
struct Point(usize, usize, usize, usize);

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
    cost: f32
}

struct WormSearch<'a, 'b> {
    open: Vec<Node>,
    closed: HashMap<Point, f32>,
    max_cost: f32,
    tile_cost: HashMap<usize, f32>,
    point_builder: &'a PointBuilder,
    map: &'b Vec<Vec<usize>>
}

impl<'a, 'b> WormSearch<'a, 'b> {
    pub fn new(start: Point, max_cost: f32, tile_cost: HashMap<usize, f32>, point_builder: &'a PointBuilder, map: &'b Vec<Vec<usize>>) -> Self {
        WormSearch {
            open: (|start| {
                let mut op = Vec::<Node>::with_capacity(60);
                op.push(Node {
                    point: start,
                    cost: 0.0
                });
                op
            })(start),
            closed: HashMap::with_capacity(60),
            max_cost,
            tile_cost,
            point_builder,
            map
        }
    }
   
    fn run(mut self) -> Vec<Point> {
        while let Some(node) = self.open.pop() {
            let neighbors = node.point.check_neighbors();
            self.closed.insert(node.point, 0.0);
            for i in neighbors.iter() {
                if self.closed.get(i).is_none() {
                    let (x, y) = i.get();

                    if let Some(tile_cost) = self.tile_cost.get(&self.map[x][y]) {
                        let total_cost = tile_cost + node.cost;

                        if total_cost < self.max_cost {
                            self.open.push(Node {
                                point: *i,
                                cost: total_cost
                            });
                            //TODO Error here, Does not check closed to see if shorter path possible
                        } else if total_cost == self.max_cost {
                            self.closed.insert(*i, total_cost);
                        }

                    } else {
                        panic!("No tile cost exists for tile type {}", &self.map[x][y]);
                    }
                }
            }
        }
        let mut rtn = Vec::with_capacity(60);
        for i in self.closed.into_iter() {
            rtn.push(i.0)
        }
        rtn
    }
}

#[cfg(test)]
mod test {
    use super::HashMap;
    use super::WormSearch;
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
        vec![0,0,0,0,0,0,0,0,0,0],
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
        0 0 0 0 x 0 0 0 0 0
         */
       
        let bob = WormSearch::new(start, 5.0, HashMap::from([
            (0, 1.0),
            (1, 99.0),
            (2, 2.0),
        ]), &point_builder, &test_vec);
        
        //println!("{:?}", bob.run());

        for i in bob.run().into_iter() {
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
