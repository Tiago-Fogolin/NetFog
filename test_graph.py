from unittest import TestCase
from graph.node import Node
from graph.graph import Graph

class NodeTest(TestCase):
    
    def test_add_connection(self):
        node1 = Node('node1')
        node2 = Node('node2')

        node1.add_connection('node2', weight=1, directed=False)
        connection = node1.connections[0]
        self.assertEqual(1,len(node1.connections))
        self.assertEqual({'node':'node2', 'weight': 1, 'directed': False}, connection)

class GraphTest(TestCase):

    def setUp(self):
        self.grafo = Graph()
        self.grafo.add_node('node1')
        self.grafo.add_node('node2')
        self.grafo.add_node('node3')
        self.grafo.add_node('node4')

       
        self.grafo.create_connection('node1', 'node2', weight=2, directed=False)
        self.grafo.create_connection('node3', 'node4', weight=4, directed=True)
        self.grafo.create_connection('node4', 'node1', weight=5.5, directed=False)
        self.grafo.create_connection('node3', 'node2', weight=1.2, directed=True)
        self.grafo.create_connection('node2', 'node3', weight=1.6, directed=False)

    def test_create_graph(self):
        graph = Graph()
        self.assertIsInstance(graph, Graph)

    def test_add_node(self):
        graph = Graph()
        graph.add_node('new_node')
        self.assertEqual(1, len(graph.nodes))
        self.assertEqual('new_node', graph.nodes[0].label)

        graph.add_node('new_new_node')
        self.assertEqual(2, len(graph.nodes))
        self.assertEqual('new_new_node', graph.nodes[1].label)


    def test_repeat_node(self):
        graph = Graph()
        graph.add_node('new_node')
        graph.add_node('new_node')
        self.assertEqual(1, len(graph.nodes))
        self.assertEqual('new_node', graph.nodes[0].label)

    def test_create_connection(self):
        graph = Graph()
        graph.add_node('node1')
        graph.add_node('node2')
        graph.add_node('node3')

        graph.create_connection('node1', 'node2')
        test_conn = [{'from': 'node1', 'to': 'node2', 'weight': 1, 'directed': False}]
        self.assertEqual(test_conn, graph.get_connections())
        graph.create_connection('node2', 'node1')
        test_conn.append({
            'from': 'node2', 'to': 'node1', 'weight': 1, 'directed': False
        })  
        self.assertEqual(test_conn, graph.get_connections())
        graph.create_connection('node3', 'node1', weight=3)
        test_conn.append({
            'from': 'node3', 'to': 'node1', 'weight': 3, 'directed': False
        }) 
        self.assertEqual(test_conn, graph.get_connections())

    def test_graph_from_adjacency_matrix(self):
        adj_matrix = [
            [0, 1],
            [1, 0]
        ]

        graph = Graph.from_adjacency_matrix(adj_matrix, custom_labels=['one', 'two'])
    
        connections = [
            {'from': 'one', 'to': 'two', 'weight': 1, 'directed': False},
            {'from': 'two', 'to': 'one', 'weight': 1, 'directed': False}
        ]

        self.assertEqual(connections, graph.get_connections())

        adj_matrix2 = [
            [0, 2, 1],
            [1, 0, 3],
            [1, 2, 0],
        ]

        graph2 = Graph.from_adjacency_matrix(adj_matrix2, custom_labels=['one', 'two', 'three'])
        connections = [
            {'from': 'one', 'to': 'two', 'weight': 2, 'directed': False},
            {'from': 'one', 'to': 'three', 'weight': 1, 'directed': False},
            {'from': 'two', 'to': 'one', 'weight': 1, 'directed': False},
            {'from': 'two', 'to': 'three', 'weight': 3, 'directed': False},
            {'from': 'three', 'to': 'one', 'weight': 1, 'directed': False},
            {'from': 'three', 'to': 'two', 'weight': 2, 'directed': False}
        ]

        self.assertEqual(connections, graph2.get_connections())

    def test_total_weight(self):
        grafo = Graph()
        grafo.add_node('node1')
        grafo.add_node('node2')
        grafo.add_node('node3')
        grafo.add_node('node4')

        grafo.create_connection('node1', 'node2', weight=2)
        grafo.create_connection('node3', 'node4', weight=4)
        grafo.create_connection('node4', 'node1', weight=5.5)
        grafo.create_connection('node3', 'node2', weight=1.2)
        grafo.create_connection('node2', 'node3', weight=1.6)

        total_weight = 2 + 4 + 5.5 + 1.2 + 1.6
        self.assertEqual(total_weight, grafo.get_total_weight())

    def test_mean_weight(self):
        grafo = Graph()
        grafo.add_node('node1')
        grafo.add_node('node2')
        grafo.add_node('node3')
        grafo.add_node('node4')

        grafo.create_connection('node1', 'node2', weight=2)
        grafo.create_connection('node3', 'node4', weight=4)
        grafo.create_connection('node4', 'node1', weight=5.5)
        grafo.create_connection('node3', 'node2', weight=1.2)
        grafo.create_connection('node2', 'node3', weight=1.6)

        mean = (2 + 4 + 5.5 + 1.2 + 1.6)/5

        self.assertEqual(mean, grafo.get_mean_weight())


    def test_get_node_count(self):
        self.assertEqual(self.grafo.get_node_count(), 4)

    def test_get_edge_count(self):
        self.assertEqual(self.grafo.get_edge_count(), 5)

    def test_get_density(self):
        expected_density = (1 * 5) / (4 * (4 - 1))
        self.assertEqual(self.grafo.get_density(directed=False), expected_density)

        expected_density_directed = (2 * 5) / (4 * (4 - 1))
        self.assertEqual(self.grafo.get_density(directed=True), expected_density_directed)

    def test_compute_degrees(self):
        degrees_node1 = self.grafo.compute_degrees('node1')
        self.assertEqual(degrees_node1['in_degree'], 0)
        self.assertEqual(degrees_node1['out_degree'], 0)
        self.assertEqual(degrees_node1['undirected_degree'], 2)
        self.assertEqual(degrees_node1['total_degree'], 0)

        degrees_node2 = self.grafo.compute_degrees('node2')
        self.assertEqual(degrees_node2['in_degree'], 1)
        self.assertEqual(degrees_node2['out_degree'], 0)
        self.assertEqual(degrees_node2['undirected_degree'], 2)
        self.assertEqual(degrees_node2['total_degree'], 1)

        degrees_node3 = self.grafo.compute_degrees('node3')
        self.assertEqual(degrees_node3['in_degree'], 0)
        self.assertEqual(degrees_node3['out_degree'], 2)
        self.assertEqual(degrees_node3['undirected_degree'], 1)
        self.assertEqual(degrees_node3['total_degree'], 2)

        degrees_node4 = self.grafo.compute_degrees('node4')
        self.assertEqual(degrees_node4['in_degree'], 1)
        self.assertEqual(degrees_node4['out_degree'], 0)
        self.assertEqual(degrees_node4['undirected_degree'], 1)
        self.assertEqual(degrees_node4['total_degree'], 1)

    def test_get_mean_degree(self):
        expected_mean_degree = (2 * 5) / 4
        self.assertEqual(self.grafo.get_mean_degree(directed=False), expected_mean_degree)

        expected_mean_degree_directed = (1 * 5) / 4
        self.assertEqual(self.grafo.get_mean_degree(directed=True), expected_mean_degree_directed)

    def test_get_mean_weight(self):
        expected_mean_weight = (2 + 4 + 5.5 + 1.2 + 1.6) / 5
        self.assertEqual(self.grafo.get_mean_weight(), expected_mean_weight)

    def test_centrality_degree(self):
        centralities = self.grafo.get_centrality_degree('node1')
        self.assertEqual(centralities['in_centrality'], 0)
        self.assertEqual(centralities['out_centrality'], 0)
        self.assertEqual(centralities['total_centrality'], 0)
        self.assertAlmostEqual(centralities['undirected_centrality'], 2 / 3)

        centralities = self.grafo.get_centrality_degree('node2')
        self.assertAlmostEqual(centralities['in_centrality'], 1 / 3)
        self.assertAlmostEqual(centralities['out_centrality'], 0)
        self.assertAlmostEqual(centralities['total_centrality'], 1 / 3)
        self.assertAlmostEqual(centralities['undirected_centrality'], 2 / 3)

        centralities = self.grafo.get_centrality_degree('node3')
        self.assertAlmostEqual(centralities['in_centrality'], 0)
        self.assertAlmostEqual(centralities['out_centrality'], 2 / 3)
        self.assertAlmostEqual(centralities['total_centrality'], 2 / 3)
        self.assertAlmostEqual(centralities['undirected_centrality'], 1 / 3)

        centralities = self.grafo.get_centrality_degree('node4')
        self.assertAlmostEqual(centralities['in_centrality'], 1 / 3)
        self.assertAlmostEqual(centralities['out_centrality'], 0)
        self.assertAlmostEqual(centralities['total_centrality'], 1 / 3)
        self.assertAlmostEqual(centralities['undirected_centrality'], 1 / 3)

        
       



        