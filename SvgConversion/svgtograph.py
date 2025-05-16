import networkx as nx
from svgpathtools import svg2paths
import xml.etree.ElementTree as ET
import matplotlib.pyplot as plt
import json

HEIGHT_INCREMENT = 50
# Replace with location of temporary midline svg
MIDLINE_TEMP_OUTPUT_PATH = "../maps/midlines.svg"


def graph_to_json(graph, output_file):
        # Assign unique IDs to each node
        node_id_map = {node: idx for idx, node in enumerate(graph.nodes())}
        
        graph_data = {"nodes": []}

        # Process nodes and their connections
        for node, data in graph.nodes(data=True):
            node_data = {
                "id": node_id_map[node],
                "coordinates": {"x": node[0], "y": node[1], "z": node[2]},
                "room_names": data.get("room_names", []),
                "connections": []
            }

            # Add connections to other nodes
            for neighbor in graph.neighbors(node):
                edge_data = graph.get_edge_data(node, neighbor)
                node_data["connections"].append({
                    "target": node_id_map[neighbor],
                    "cost": edge_data.get("cost", 1)  # Default cost if missing
                })

            graph_data["nodes"].append(node_data)

        # Save to JSON file
        with open(output_file, "w") as f:
            json.dump(graph_data, f, indent=4)

        print(f"Graph saved to {output_file}")





# Removes artifacts from generating midline svg
def strip_namespace(element):
    # Recursively remove namespace prefixes from an element and its children.
    for elem in element.iter():
        if '}' in elem.tag:
            elem.tag = elem.tag.split('}', 1)[1] 

# Takes the original svg and generates a new svg of only the midlines   
def remove_non_midlines(input_svg_path):

    tree = ET.parse(input_svg_path)
    root = tree.getroot()

    # Extract the namespace (if exists)
    namespace = root.tag.split('}')[0].strip('{') if '}' in root.tag else None
    ns = f'{{{namespace}}}' if namespace else ''

    # Find the <g> element with id="midlines"
    midlines_group = root.find(f".//{ns}g[@id='MidlinePath']")

    if midlines_group is not None:
        # Create a new SVG root element with the same attributes
        new_svg = ET.Element("svg", root.attrib)
        
        # Ensure the namespace is preserved
        if namespace:
            new_svg.set("xmlns", namespace)

        strip_namespace(new_svg)
        strip_namespace(midlines_group)
        
        # Append the extracted <g> element inside the new SVG
        new_svg.append(midlines_group)

        # Write the new SVG file
        new_tree = ET.ElementTree(new_svg)

        new_tree.write(MIDLINE_TEMP_OUTPUT_PATH, encoding="utf-8", xml_declaration=True)
    

# Takes the svg file and returns a dictionary of stairs and elevators, where each pair is {id:(x,y)}   
def get_floor_changers(svg_filename):
    
    tree = ET.parse(svg_filename)
    root = tree.getroot()

    elev = None
    stairs = None
    
    elevators_dict = {}
    stairs_dict = {}

    # Extract the desired group
    for g in root.findall('.//{http://www.w3.org/2000/svg}g[@id="Elevator"]'):
        elev = g
        break 

    if elev is not None:
        for elevator in elev:
            elevators_dict[elevator.attrib['adjacency']] = (float(elevator.attrib['cx']),float(elevator.attrib['cy']))
            
        
    
    for g in root.findall('.//{http://www.w3.org/2000/svg}g[@id="Stairs"]'):
        stairs = g
        break
    
    if stairs is not None:
        for stair_case in stairs:
            stairs_dict[stair_case.attrib['adjacency']] = (float(stair_case.attrib['cx']),float(stair_case.attrib['cy']))
                   
    return stairs_dict, elevators_dict

# Takes the svg file and returns a dictionary of rooms, where each pair is {room-name:(x,y)}   
def get_rooms(svg_filename):
    
    entrance_dict = {}
    tree = ET.parse(svg_filename)
    root = tree.getroot()
    entrances = None
    for g in root.findall('.//{http://www.w3.org/2000/svg}g[@id="Entrance"]'):
        entrances = g
        break
    
    if entrances is not None:
        for entrance in entrances:
            entrance_dict[entrance.attrib['data-name']] = (float(entrance.attrib['cx']),float(entrance.attrib['cy']))
    
    return entrance_dict
    
# Processes the midlines of the svg and converts it into a graph, the nodes temporarily contain no data 
def svg_to_graph(midlines, floor_num):

    # Retreive all polylines from the SVG
    returned_paths = svg2paths(midlines,
        convert_circles_to_paths = False,
        convert_ellipses_to_paths = False,
        convert_lines_to_paths = False,
        convert_polylines_to_paths = True,
        convert_polygons_to_paths = False,
        convert_rectangles_to_paths = False)
    
    if len(returned_paths) == 2:
        paths, attributes = returned_paths
    else:
        paths, attributes, svg_attributes = returned_paths

    G = nx.Graph()
        
    # Loop over each path in the SVG
    for path in paths:
        # Each 'path' is a svgpathtools Path object (a list of segments).
        for segment in path:

            start_complex = segment.start  # start of path segment
            end_complex   = segment.end    # end of path segment
            
            
            cost = abs(start_complex - end_complex) 
            
            # Convert to floats
            start_node = (start_complex.real, start_complex.imag, floor_num * HEIGHT_INCREMENT)
            end_node   = (end_complex.real, end_complex.imag, floor_num * HEIGHT_INCREMENT)
            

            # Add the start and end as nodes in the graph
            G.add_node(start_node, room_names = [], elevator_number = 0, stair_number = 0)
            G.add_node(end_node , room_names = [], elevator_number = 0, stair_number = 0)

            G.add_edge(start_node, end_node, cost = cost, segment_type=type(segment).__name__)

    return G


# This function is meant to examine each node and determine if the node is a room, staircase, or elevator
def populate_data(svg_filename, nodes):
    stairs_dict, elevator_dict = get_floor_changers(svg_filename)
    entrance_dict = get_rooms(svg_filename)
    
    for node in nodes:
        # Check the point to see if it's a stair or elevator
        for stair in stairs_dict:
            cx , cy = stairs_dict[stair]
            if cx == node[0] and cy == node[1]:
                nodes[node]['stair_number'] = stair
                    
                break

                    
        for elevator in elevator_dict:
            cx,cy = elevator_dict[elevator]
            if cx == node[0] and cy == node[1]:
                nodes[node]['elevator_number'] = elevator
                break
                
        for entrance in entrance_dict:
            cx,cy = entrance_dict[entrance]
            if cx == node[0] and cy == node[1]:
                nodes[node]['room_names'].append(entrance)
                break
        
# Driver function for graphing a floor, graphs out the passed svg and displays it, returns the graph        
def graph_floor(svg_path , floor_num):   
    
    remove_non_midlines(svg_path)
    graph = svg_to_graph(MIDLINE_TEMP_OUTPUT_PATH, floor_num)
    populate_data(svg_path,graph.nodes())

    # Print out basic info
    print("Number of nodes:", graph.number_of_nodes())
    print("Number of edges:", graph.number_of_edges())
     
   
    node_colors = []
    for node in graph.nodes():
        if graph.nodes[node]['elevator_number'] != 0:
            node_colors.append('red')  # Color elevators red
            
        elif graph.nodes[node]['stair_number'] != 0:
            node_colors.append('brown')
        elif len(graph.nodes[node]['room_names']) > 0:
            node_colors.append('green')
        else:
            node_colors.append('blue')
    
    pos = {node: (node[0], node[1]) for node in graph.nodes()}


    nx.draw(graph, pos=pos, with_labels=False, node_size=6, font_size=200,node_color=node_colors)
    
    for node, data in graph.nodes(data=True):
        if data['room_names']:  # Check if the node has room names
            room_label = ', '.join(data['room_names'])  # Join multiple names
            plt.text(node[0], node[1], room_label, fontsize=8, ha='right', color='black', bbox=dict(facecolor='white', alpha=0.5, edgecolor='none'))

    #plt.show()

    return graph


if __name__ == "__main__":


    # Graph each of the floors
    svg_path1 = "../maps/test_one_backend.svg"
    svg_path2 = "../maps/test_two_backend.svg"
    floor1 = graph_floor(svg_path1 , 0)
    floor2 = graph_floor(svg_path2 , 1)
    
    master_graph = nx.Graph()
    
    floor_graphs = [floor1, floor2]
    
    for floor_graph in floor_graphs:
        master_graph = nx.compose(master_graph, floor_graph)
    
    for i in range(len(floor_graphs) - 1):  # Connect adjacent floors
        current_floor = floor_graphs[i]
        next_floor = floor_graphs[i + 1]

        for node in current_floor.nodes:
            if 'elevator_number' in current_floor.nodes[node] and current_floor.nodes[node]['elevator_number'] != 0:
                elevator_id = current_floor.nodes[node]['elevator_number']
                
                # Find matching node on the next floor
                for next_node in next_floor.nodes:
                    if ('elevator_number' in next_floor.nodes[next_node] and 
                        next_floor.nodes[next_node]['elevator_number'] == elevator_id):
                        
                        master_graph.add_edge(node, next_node, cost=1)  # Small cost for moving between floors
                        
            if 'stair_number' in current_floor.nodes[node] and current_floor.nodes[node]['stair_number'] != 0:
                stair_id = current_floor.nodes[node]['stair_number']
                
                # Find matching node on the next floor
                for next_node in next_floor.nodes:
                    if ('stair_number' in next_floor.nodes[next_node] and 
                        next_floor.nodes[next_node]['stair_number'] == stair_id):
                        
                        master_graph.add_edge(node, next_node, cost=HEIGHT_INCREMENT) 
                        
                        
    print(len(master_graph.nodes()))
    print(len(master_graph.edges()))


    # Convert and save the final master graph to JSON
    output_json_path = "../maps/graph_data.json"
    graph_to_json(master_graph, output_json_path)