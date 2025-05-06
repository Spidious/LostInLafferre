use crate::*;
use std::fs;
use serde::Deserialize;


#[derive(Deserialize)]
struct GraphData {
    nodes: Vec<RoomEntry>,
}


#[derive(Deserialize)]
struct RoomEntry {
    room_names: Vec<String>,
    #[serde(skip)]
    lafferre: Graph<Coords, f64, Undirected>,
    #[serde(skip)]
    room_hash: HashMap<String, NodeIndex>,
}

impl RoomEntry {
    fn new() -> Self {
        let all_names = Self::fetch_room_names();

        let mut deps = Graph::<Coords, f64, Undirected>::new_undirected();
        let mut room_gid: HashMap<String, NodeIndex> = HashMap::new();
        let _ = create_graph_from_json(&mut deps, &mut room_gid, "graph_data.json");  

        Self {
            room_names: all_names,
            lafferre: deps,
            room_hash: room_gid,
        }
    }

    /// Fetch room names from a JSON file and return them as a vector of strings.
    fn fetch_room_names() -> Vec<String> {
        let data = fs::read_to_string("../graph_data.json").expect("Failed to read file");
        let graph: GraphData = serde_json::from_str(&data).expect("Invalid JSON");
    
        graph.nodes.into_iter()
            .flat_map(|node| node.room_names)
            .filter(|name| !name.trim().is_empty())
            .collect()
    }
    

    /// Get all Nth Floor room names from the JSON file.
    fn get_nth_floor_rooms(&self, floor_num: u8) -> Vec<String> {
        self.room_names
            .iter()
            .filter(|name| name.chars().nth(1) == Some(floor_num as char))
            .cloned()
            .collect()
    }

    

}

# [cfg(test)]
mod tests {
    use super::*;

    // Add room handling for the json file
    static LAFFERRE_ROOMS: std::sync::LazyLock<RoomEntry> = std::sync::LazyLock::new(|| RoomEntry::new());
   

    /**************************************************************************
     * Unit Tests Section 1: Floor to Floor
     * 1. Basement to Third Floor
     * 2. First Floor to Second Floor Study Rooms
     * 3. First Floor to First Floor
     * 4. Third Floor to Basement
     **************************************************************************/

     // Basement to Third Floor
    #[test]
    fn b_to_tf() {
        // Get all Basement rooms
        let basement_rooms = LAFFERRE_ROOMS.get_nth_floor_rooms(0);
        let third_floor_rooms = LAFFERRE_ROOMS.get_nth_floor_rooms(3);

        // Test each basement_room against each third_floor_room consecutively
        for basement_room in &basement_rooms {
            // Get node from room hash
            let src_node = LAFFERRE_ROOMS.room_hash.get(basement_room.as_str());
            for third_floor_room in &third_floor_rooms {
                // Get node from room hash
                let dst_node = LAFFERRE_ROOMS.room_hash.get(third_floor_room.as_str());
                // Call the function to test with the room names
                let result = find_path(&LAFFERRE_ROOMS.lafferre, src_node.expect("Could not get src_node"), dst_node.expect("Could not get dst_node"));
                assert!(result.is_some(), "No path found between {} and {}", basement_room, third_floor_room);
            }
        }
    }
    
    
    // First Floor to Second Floor Study Rooms
    #[test]
    fn ff_to_sf() {
        
    }
    
    // First Floor to First Floor
    #[test]
    fn ff_to_ff() {
        
    }
    
    // Third Floor to Basement
    #[test]
    fn tf_to_b() {
        
    }

    /**************************************************************************
     * Unit Tests Section 1: Floor to Floor
     * 1. API Response
     * 2. Stair/Elevator Selection
     * 3. Bad Input from Frontend
     * 4. Multiple Request Handling (threading)
     * 5. Create Graph from Json
     * 6. 25+ rooms (Limit Testing)
     **************************************************************************/
    
    // API Response
    #[test]
    fn api_resp() {

    }

    // Optimal Stair/Elevator Selection
    #[test]
    fn stair_elev() {
        
    }
    
    // Bad input from frontend
    #[test]
    fn bad_input() {
        
    }
    
    // Multiple Request Handling
    #[test]
    fn mult_req() {
        
    }
    
    // Create Graph from Json
    #[test]
    fn graph_create() {
        
    }
    
    // 25+ rooms
    #[test]
    fn crash_test() {
        
    }
    
    // Same Room as Source and Destination
    #[test]
    fn same_room() {
        
    }
}

