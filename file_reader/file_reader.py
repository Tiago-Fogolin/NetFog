class FileReaderTemplate:
    def read_file(self, path_to_file: str): raise NotImplementedError()

class NetFileReader(FileReaderTemplate):
    
    def read_file(self, path_to_file):
        graph_dict = {
            'nodes': [],
            'edges': [],
            'arcs': []
        }
        node_dict = {}
        with open(path_to_file, 'r', encoding='utf8') as file:
            reading_nodes = False
            reading_edges = False
            reading_arcs  = False

            for line in file.readlines():
                line = line.replace("\n", "")

                if line == '':
                    continue

                if '*vertices' in line or '*Vertices' in line:
                    reading_edges = False
                    reading_arcs  = False
                    reading_nodes = True
                    continue
                
                if '*edges' in line or '*Edges' in line:
                    reading_edges = True
                    reading_arcs  = False
                    reading_nodes = False
                    continue

                if '*arcs' in line:
                    reading_edges = False
                    reading_arcs  = True
                    reading_nodes = False
                    continue

                if reading_nodes:
                    node_index, node_label, x_pos, y_pos = (None, None, None, None)

                    start_node = line.find('"')
                    end_node = line.find('"', start_node + 1)

                    node_label = line[start_node+1:end_node]

                    line_elements = line.split()
                    
                    
                    if len(line_elements) == 2:
                        node_index = line_elements[0]
                    elif len(line_elements) > 2:
                        node_index, x_pos, y_pos = line_elements[0], line_elements[-2], line_elements[-1]

                    node = {
                        'node': node_label,
                        'x': x_pos,
                        'y': y_pos
                    }
                    
                    graph_dict['nodes'].append(node)
                    node_dict[int(node_index)] = node_label

                elif reading_edges:
                    from_index, to_index, weight = line.split()

                    conn = {
                        'from': node_dict[int(from_index)],
                        'to': node_dict[int(to_index)],
                        'weight': weight
                    }
                    
                    graph_dict['edges'].append(conn)
                
                elif reading_arcs:
                    from_index, to_index, weight = line.split()

                    conn = {
                        'from': node_dict[int(from_index)],
                        'to': node_dict[int(to_index)],
                        'weight': weight
                    }
                    
                    graph_dict['arcs'].append(conn)

        return graph_dict
                    
