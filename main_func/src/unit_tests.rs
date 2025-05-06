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
        let target = (b'0' + floor_num) as char;
        self.room_names
            .iter()
            .filter(|name| name.chars().nth(1) == Some(target))
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
     * (These tests do not implement API calls and several take over a minute to run)
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
        // Get all third_floor rooms
        let third_floor_rooms = LAFFERRE_ROOMS.get_nth_floor_rooms(3);

        let mut count = 0;
        // Test each basement_room against each third_floor_room consecutively
        for basement_room in &basement_rooms {
            // Get node from room hash
            let src_node = LAFFERRE_ROOMS.room_hash.get(basement_room.as_str());
            for third_floor_room in &third_floor_rooms {
                // Get node from room hash
                let dst_node = LAFFERRE_ROOMS.room_hash.get(third_floor_room.as_str());
                // Call the function to test with the room names
                let result = find_path(&LAFFERRE_ROOMS.lafferre, src_node.expect("Could not get src_node"), dst_node.expect("Could not get dst_node"));
                count = count + 1;
                assert!(result.is_some(), "No path found between {} and {}", basement_room, third_floor_room);
            }
        }
        // Run with `cargo test -- --nocapture` to see output
        println!("b_to_tf Count: {}", count);
    }
    
    
    // First Floor to Second Floor Study Rooms
    #[test]
    fn ff_to_sf() {
        // Get all first_floor rooms
        let first_floor_rooms = LAFFERRE_ROOMS.get_nth_floor_rooms(1);
        // Get all second_floor rooms
        let second_floor_rooms = LAFFERRE_ROOMS.get_nth_floor_rooms(2); // Just do all second floor

        let mut count = 0;
        // Test each first_floor_room against each second_floor_room consecutively
        for first_floor_room in &first_floor_rooms {
            // Get node from room hash
            let src_node = LAFFERRE_ROOMS.room_hash.get(first_floor_room.as_str());
            for second_floor_room in &second_floor_rooms {
                // Get node from room hash
                let dst_node = LAFFERRE_ROOMS.room_hash.get(second_floor_room.as_str());
                // Call the function to test with the room names
                let result = find_path(&LAFFERRE_ROOMS.lafferre, src_node.expect("Could not get src_node"), dst_node.expect("Could not get dst_node"));
                count = count + 1;
                assert!(result.is_some(), "No path found between {} and {}", first_floor_room, second_floor_room);
            }
        }
        // Run with `cargo test -- --nocapture` to see output
        println!("ff_to_sf Count: {}", count);
    }
    
    // First Floor to First Floor
    #[test]
    fn ff_to_ff() {
        // Get all first_floor rooms
        let first_floor_rooms = LAFFERRE_ROOMS.get_nth_floor_rooms(1);

        let mut count = 0;
        // Test each first_floor_room against each first_floor_room consecutively
        for src_room in &first_floor_rooms {
            // Get node from room hash
            let src_node = LAFFERRE_ROOMS.room_hash.get(src_room.as_str());
            for dst_room in &first_floor_rooms {
                // Skip duplicate room
                if (src_room == dst_room) {
                    continue;
                }
                // Get node from room hash
                let dst_node = LAFFERRE_ROOMS.room_hash.get(dst_room.as_str());
                // Call the function to test with the room names
                let result = find_path(&LAFFERRE_ROOMS.lafferre, src_node.expect("Could not get src_node"), dst_node.expect("Could not get dst_node"));
                count = count + 1;
                assert!(result.is_some(), "No path found between {} and {}", src_room, dst_room);
            }
        }
        // Run with `cargo test -- --nocapture` to see output
        println!("ff_to_ff Count: {}", count);        
    }
    
    // Third Floor to Basement
    #[test]
    fn tf_to_b() {
        // Get all Basement rooms
        let basement_rooms = LAFFERRE_ROOMS.get_nth_floor_rooms(0);
        // Get all third_floor rooms
        let third_floor_rooms = LAFFERRE_ROOMS.get_nth_floor_rooms(3);

        let mut count = 0;
        // Test each third_floor_room against each basement_room consecutively
        for third_floor_room in &third_floor_rooms {
            // Get node from room hash
            let src_node = LAFFERRE_ROOMS.room_hash.get(third_floor_room.as_str());
            for basement_room in &basement_rooms {
                // Get node from room hash
                let dst_node = LAFFERRE_ROOMS.room_hash.get(basement_room.as_str());
                // Call the function to test with the room names
                let result = find_path(&LAFFERRE_ROOMS.lafferre, src_node.expect("Could not get src_node"), dst_node.expect("Could not get dst_node"));
                count = count + 1;
                assert!(result.is_some(), "No path found between {} and {}", third_floor_room, basement_room);
            }
        }
        // Run with `cargo test -- --nocapture` to see output
        println!("tf_to_b Count: {}", count);        
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

