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


// State will be pushed to the open list
struct State{
    node : nodeIndex,
    f : f64,
    g : f64
}

fn findPath(start:Coords,end:Coords,graph:Graph){

    //closed list
    let mut closed = Vec::new();
    
    // open list
    let mut open = BinaryHeap::new();


    // Path List
    let mut path = Vec::new();


    // add the start node to the open list

    // best known distance from start to finish
    let mut g_score = vec![f64::INFINITY; n];



    // add all adjacent nodes to the open list

    // iterate through the open list calculate
        //expand eacg node in order of priorty

    
    




}


