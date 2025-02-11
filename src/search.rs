use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use petgraph::{Graph, Undirected};


// Borrowed from Anders for consistency in model
struct Coords(f64, f64, f64);

impl Coords{
    //3D euclidean distance function
    fn euc_dist(&self,other: &Coords) -> f64{
        f64::sqrt((self.0 - other.0)*(self.0 - other.0) +  
                (self.1 - other.1)*(self.1 - other.1) + 
                (self.2 - other.2)*(self.2 - other.2))
    }
}


//place holder
struct Node{
    id: int,

}

fn findPath(start:Coords,end:Coords,graph:Graph){


    
    // open list
    let mut open = BinaryHeap::new();




}
