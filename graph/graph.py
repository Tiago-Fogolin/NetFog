from graph.node import Node
from template.html_writer import HtmlWriter
from template.svg_writer import SVGWriter
from layouts.layouts import RandomLayout, normalize_positions
from file_reader.file_reader import FileReaderTemplate, NetFileReader, JsonFileReader
from file_writer.file_writer import NetFileWriter, JsonFileWriter
from style.graph_style import GraphStyle
from collections import deque
import math

def create_nodes_from_labels(size, labels):
        str_list = labels if labels else list(map(str, range(size)))
        
        return list(map(Node, str_list))
    
def create_node_dict(nodes, start_index=0):
    node_dict = dict(
        [(i, str(node)) for i, node in enumerate(nodes, start_index)]
    )

    return node_dict

def create_positions_node_dicts(nodes, start_index=0, positions=None):
    node_dict = {}

    for index, node in enumerate(nodes, start_index):
        x = positions[node.label]['x']
        y = positions[node.label]['y']
        node_dict[node.label] = {'index': index, 'x': x, 'y': y}
        

    return node_dict

def invert_node_dict(node_dict):
    return dict( (node_label, i) for i, node_label in node_dict.items() )

def create_node_tuple_list(nodes, connections, work_with_labels=False):
    node_dict = invert_node_dict(create_node_dict(nodes, start_index=1))
    edges_tuple_list = []
    arcs_tuple_list = []

    for conn in connections:
        from_node = node_dict[conn['from']]
        to_node = node_dict[conn['to']]
        weight = conn['weight']

        if work_with_labels:
            from_node = conn['from']
            to_node = conn['to']
            weight = conn['weight']
        
        if conn['directed']:
            arcs_tuple_list.append((from_node, to_node, weight))
            continue

        edges_tuple_list.append((from_node, to_node, weight))

    return edges_tuple_list, arcs_tuple_list

class Graph:
    """
    A class that represents a graph with nodes and its connections.
    """
    
    def __init__(self):
        self.nodes = []
        self.normalized_positions = {}

    def add_node(self, label: str) -> None:
        """
        Args:
            label (str): Indicates the label of a new node in the graph.
        """
        new_node = Node(label)
        if str(new_node) not in list(map(str, self.nodes)):
            self.nodes.append(new_node)

        return new_node
    
    def node_by_label(self, label):

        for node in self.nodes:
            if node.label == label:
                return node

        return None
    
    def create_connection(self, from_: str, to_: str, weight: int = 1, directed: bool = False):
        """
        Args:
            from_ (str): Indicates the label of the node it is creating the connection.
            to_ (str): Indicates the label of the node that is being connected.
            weight (int, optional): Indicates the weigth of the connection.
            directed (boolean, optional): Indicates wether the connection is directed or not.
        """
        from_node = None
        to_node = None
        for node in self.nodes:
            if node.label == from_:
                from_node = node
            if node.label == to_:
                to_node = node
            
        if not from_node or not to_node:
            raise Exception("Node not found.")
    
        from_node.add_connection(to_node, weight, directed)

    def get_connections(self):
        all_connections = []
        
        for node in self.nodes:
            
            for conn in node.connections:
                formatted_conn = {}

                formatted_conn['from'] = str(node)
                formatted_conn['to'] = str(conn['node'])
                formatted_conn['weight'] = conn['weight']
                formatted_conn['directed'] = conn['directed']
                
                all_connections.append(formatted_conn)

        return all_connections
    
    def from_adjacency_matrix(adj_matrix: list , directed: bool = False, custom_labels: list = None) -> 'Graph':
        """
        Args:
            adj_matrix (list): A list that represents the connections of the graph.
            directed (bool, optional): Indicates wether the connections are directed or not.
            custom_labels (list, optional): A list containing the labels of the nodes.
        """
        ajd_matrix_graph = Graph()
        ajd_matrix_graph.nodes = create_nodes_from_labels(
            len(adj_matrix), 
            custom_labels
        )

        ajd_matrix_graph.node_dict = create_node_dict(ajd_matrix_graph.nodes)

        for i in range(len(adj_matrix)):
            for j in range(len(adj_matrix)):
                weight = adj_matrix[i][j]
                if weight != 0:
                    
                    ajd_matrix_graph.create_connection(
                        ajd_matrix_graph.node_dict[i],
                        ajd_matrix_graph.node_dict[j],
                        weight,
                        directed,
                    )
        return ajd_matrix_graph
    
    def _from_dict(dictionary: dict):
        new_Graph = Graph()

        for i, node in enumerate(dictionary['nodes']):
            new_Graph.add_node(node['node'])
            if node['x'] and node['y']:
                new_Graph.normalized_positions[node['node']] = {'x': node['x'], 'y': node['y'], 'index': i}
        
        for edge in dictionary['edges']:
            _from, _to, _weight = edge['from'], edge['to'], edge['weight']

            new_Graph.create_connection(_from, _to, _weight, directed=False)
        
        for arc in dictionary['arcs']:
            _from, _to, _weight = arc['from'], arc['to'], arc['weight']

            new_Graph.create_connection(_from, _to, _weight, directed=True)

        return new_Graph
    
    def _from_file_reader(file_path: str, file_reader: FileReaderTemplate):
        reader = file_reader()

        graph = Graph._from_dict(reader.read_file(file_path))

        return graph

    def _get_dict():
        ...

    def from_net_file(file_path: str):
        return Graph._from_file_reader(file_path, NetFileReader)
    
    def from_json_file(file_path: str):
        return Graph._from_file_reader(file_path, JsonFileReader)


    def generate_adjacency_matrix(self):
        matrix_size = len(self.nodes)
        adj_matrix = [[0 for i in range(matrix_size)] for i in range(matrix_size)]
        node_dict = invert_node_dict(create_node_dict(self.nodes))
        connections = self.get_connections()

        for conn in connections:
            i = node_dict[conn['from']]
            j = node_dict[conn['to']]
            adj_matrix[i][j] = int(conn['weight'])

            if not conn['directed']:
                adj_matrix[j][i] = int(conn['weight'])


        return adj_matrix
    
    def get_total_weight(self):
        return sum(list(map(lambda x: x['weight'], self.get_connections())))

    def get_mean_weight(self):
        return self.get_total_weight()/len(self.get_connections())
    
    def get_node_count(self):
        return len(self.nodes)

    def get_edge_count(self):
        return len(self.get_connections())
    
    def get_density(self, directed=False):
        edge_count = self.get_edge_count()
        node_count = self.get_node_count()
        multiply = 2 if directed else 1

        density = (multiply * edge_count) / (node_count * (node_count - 1))

        return density

    def compute_degrees(self, node_label: str):
        """
            Returns the in_degree (only for arcs), 
                        out_degree (only for arcs),
                        total_degree (sum of in_degree and out_degree),
                        undirected_degree (number of edges connected to the node)
        """
        connections = self.get_connections()

        degrees = {
            "in_degree": 0,
            "out_degree": 0,
            "total_degree": 0,
            "undirected_degree": 0
        }

        for conn in connections:
            directed = conn['directed']

            if directed:
                if conn['from'] == node_label:
                    degrees['out_degree'] += 1
                    degrees['total_degree'] += 1

                if conn['to'] == node_label:
                    degrees['in_degree'] += 1
                    degrees['total_degree'] += 1
                    
                continue

            if conn['to'] == node_label or conn['from'] == node_label:
                degrees['undirected_degree'] += 1

        return degrees

    def compute_entropy(self):
        result = {
            'in_entropy': 0,
            'out_entropy': 0,
            'undirected_entropy': 0
        }

        dist = self.get_degree_distribution()

        for degree, dist_value in dist['in_distribution'].items():
            result['in_entropy'] += dist_value * math.log(dist_value, math.e)

        for degree, dist_value in dist['out_distribution'].items():
            result['out_entropy'] += dist_value * math.log(dist_value, math.e)

        for degree, dist_value in dist['undirected_distribution'].items():
            result['undirected_entropy'] += dist_value * math.log(dist_value, math.e)

        result['in_entropy'] = -result['in_entropy']
        result['out_entropy'] = -result['out_entropy']
        result['undirected_entropy'] = -result['undirected_entropy']

        return result
    
    def get_max_possible_entropy(self):
        return math.log(self.get_node_count() - 1, math.e)

    

    def get_skewness(self):


        def _rank_degree_for_skewness(degree_collection):
            sorted_degrees = sorted(degree_collection.items(), key=lambda x: x[1], reverse=True)
            ranked = []

            for i, (node_label, degree) in enumerate(sorted_degrees, start=1):
                rank = (i, degree)
                ranked.append(rank)

            return ranked
        
        def _get_ranked_degrees(all_nodes_degrees):
            ranked_degrees = []

            for degree_type in ['in_degree', 'out_degree', 'undirected_degree']:
                ranked = _rank_degree_for_skewness({x: all_nodes_degrees[x][degree_type] for x in all_nodes_degrees})
                ranked_degrees.append(ranked)

            return ranked_degrees
        
        def _compute_sknums(ranked_degrees):
            sknums = []
            for ranked_degree in ranked_degrees:

                result = 0
                for rank, degree in ranked_degree:
                    result += rank * degree

                sknums.append(result)

            return sknums
        
        def _compute_sku(ranked_degrees, node_count):
            mean_degrees = []
            for ranked in ranked_degrees:
                if ranked:
                    degrees = [degree for rank, degree in ranked]
                    mean_degrees.append(sum(degrees) / len(degrees))
                else:
                    mean_degrees.append(0)

            skus = []

            for degree in mean_degrees:
                skus.append(degree * (node_count * (node_count + 1) / 2))

            return skus

        result = {
            'in_skewness': 0,
            'out_skewness': 0,
            'undirected_skewness': 0
        }

        degrees = self.get_all_nodes_degrees()

        ranked_degrees = _get_ranked_degrees(degrees)

        sknums = _compute_sknums(ranked_degrees)

        skus = _compute_sku(ranked_degrees, self.get_node_count())

        result['in_skewness'] = sknums[0] / skus[0]
        result['out_skewness'] = sknums[1] / skus[1]
        result['undirected_skewness'] = sknums[2] / skus[2]

        return result



    def get_all_nodes_degrees(self):
        degree_dict = {}

        for node in self.nodes:
            degree = self.compute_degrees(node.label)

            degree_dict[node.label] = degree

        return degree_dict

    def get_average_degree(self, directed=False):
        multiply = 1 if directed else 2

        edge_count = self.get_edge_count()
        node_count = self.get_node_count()

        mean = (multiply * edge_count) / (node_count)

        return mean
    
    def get_node_strength(self, node_label: str):
        """
            Returns the out_strength, in_strength, and total_strength of a node
        """

        connections = self.get_connections()

        strengths = {
            "out_strength": 0,
            "in_strength": 0,
            "total_strength": 0
        }

        for conn in connections:
            if conn['from'] == node_label:
                strengths['out_strength'] += conn['weight']

                if not conn['directed']:
                    strengths['in_strength'] += conn['weight']

            if conn['to'] == node_label:
                strengths['in_strength'] += conn['weight']

                if not conn['directed']:
                    strengths['out_strength'] += conn['weight']

        strengths['total_strength'] = strengths['out_strength'] + strengths['in_strength']

        return strengths

    def get_centrality_degree(self, node_label: str):
        """
            Returns the out_centrality (only for arcs),
                        in_centrality (only for arcs),
                        total_centrality (only for arcs),
                        undirected_centrality (only for edges)
        """
        centralities = {
            "out_centrality" : 0,
            "in_centrality" : 0,
            "total_centrality" : 0,
            "undirected_centrality" : 0,
        }

        degrees = self.compute_degrees(node_label)
        node_count = self.get_node_count()
        
        if node_count <= 1:
            return centralities
        
        
        centralities['out_centrality'] = degrees["out_degree"] / (node_count - 1)
        centralities['in_centrality'] = degrees["in_degree"] / (node_count - 1)
        centralities['total_centrality'] = degrees["total_degree"] / (node_count - 1)
        centralities['undirected_centrality'] = degrees["undirected_degree"] / (node_count - 1)

        return centralities
    
    def get_degree_distribution(self):
        computed_degrees = [self.compute_degrees(node.label) for node in self.nodes]
        node_count = self.get_node_count()
        count_in_degree = {}
        count_out_degree = {}
        count_undirected_degree = {}

        distribution = {
            'undirected_distribution': 0,
            'in_distribution': 0,
            'out_distribution': 0
        }

        for computed_degree in computed_degrees:
            in_degree = computed_degree.get('in_degree', 0)
            out_degree = computed_degree.get('out_degree', 0)
            undirected_degree = computed_degree.get('undirected_degree', 0)

           
            count_in_degree[in_degree] = count_in_degree.get(in_degree, 0) + 1

            count_out_degree[out_degree] = count_out_degree.get(out_degree, 0) + 1
        
            count_undirected_degree[undirected_degree] = count_undirected_degree.get(undirected_degree, 0) + 1

        distribution = {
            'undirected_distribution': {k: v / node_count for k, v in count_undirected_degree.items()},
            'in_distribution': {k: v / node_count for k, v in count_in_degree.items()},
            'out_distribution': {k: v / node_count for k, v in count_out_degree.items()}
        }

        return distribution

    def dfs(self, start_node_label=''):
        """
            Returns the nodes in order, when transversed by a dfs algorithm
        """
        final_order = []
        
        visited = set()

        def _dfs(node):
            final_order.append(node)
            visited.add(node)

            for conn in node.connections:
                if conn['node'] not in visited:
                    _dfs(conn['node'])

        start_node = self.nodes[0] if not start_node_label else self.node_by_label(start_node_label)

        _dfs(start_node)

        return final_order

    def bfs(self, start_node_label=''):
        """
            Returns the nodes in order, when transversed by a bfs algorithm
        """
        final_order = []

        start_node = self.nodes[0] if not start_node_label else self.node_by_label(start_node_label)
        q = deque([start_node])
        visited = set([start_node])

        while q:
            node = q.popleft()
            final_order.append(node)
            
            for conn in node.connections:
                if conn['node'] not in visited:

                    q.append(conn['node'])
                    visited.add(conn['node'])

        return final_order

    def dijkstra(self, start_node_label=''):
        size = len(self.nodes)
        start_vertex = self.nodes.index(self.node_by_label(start_node_label)) if start_node_label else 0
        distances = {}
        for i in range(size):
            distances[self.nodes[i].label] = float('inf')
        distances[self.nodes[start_vertex].label] = 0
        visited = [False] * size
        adj_matrix = self.generate_adjacency_matrix()
 
        for _ in range(size):
            min_distance = float('inf')
            u = None
            for i in range(size):
                if not visited[i] and distances[self.nodes[i].label] < min_distance:
                    min_distance = distances[self.nodes[i].label]
                    u = i

            if u is None:
                break

            visited[u] = True

            for v in range(size):
                if adj_matrix[u][v] != 0 and not visited[v]:
                    alt = distances[self.nodes[u].label] + adj_matrix[u][v]
                    if alt < distances[self.nodes[v].label]:
                        distances[self.nodes[v].label] = alt

        return distances

    def output_html(self, file_name, layout=RandomLayout, style=GraphStyle(), override_positions=False):
        svg_writer = SVGWriter(graph_style=style)

        svg_writer.draw_graph(self.nodes, self.get_connections(), layout, self.normalized_positions, override_positions)

        html_writer = HtmlWriter(str(svg_writer.get_svg()))

        html_writer.output(file_name)

    def output_net_file(self, path, layout=RandomLayout):
        net_file_writer = NetFileWriter()

        positions = self.normalized_positions if self.normalized_positions else normalize_positions(layout().generate_positions(self.nodes))

        node_dict_positions = create_positions_node_dicts(self.nodes, 1, positions)
        
        edges, arcs = create_node_tuple_list(self.nodes, self.get_connections())

        net_file_writer.write_file(path, node_dict_positions, edges, arcs)

    def output_json_file(self, path):
        json_file_writer = JsonFileWriter()

        edges, arcs = create_node_tuple_list(self.nodes, self.get_connections(), work_with_labels=True)

        json_file_writer.write_file(path, self.nodes, edges, arcs)