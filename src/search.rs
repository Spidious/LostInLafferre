use std::collections::BinaryHeap;
use std::cmp::Ordering;



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

fn findPath(start:Coords,end:Coords,graph:Node){

    let mut open = BinaryHeap::new();




}
