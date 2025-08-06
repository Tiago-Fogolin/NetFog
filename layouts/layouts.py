import random
from functools import partial
import math

MAX_WIDTH = 1500
MIN_WIDTH = 20
MAX_HEIGHT = 700
MIN_HEIGHT = 20


def denormalize_positions(normalized_positions):
    denormalized_positions = {}

    x_center_offset = (MAX_WIDTH - MIN_WIDTH) / 2
    y_center_offset = (MAX_HEIGHT - MIN_HEIGHT) / 2

    for node_label, node_position in normalized_positions.items():
        new_x = float(node_position['x']) * (MAX_WIDTH - MIN_WIDTH) + MIN_WIDTH
        new_y = float(node_position['y']) * (MAX_HEIGHT - MIN_HEIGHT) + MIN_HEIGHT
        denormalized_positions[node_label] = {'x': new_x, 'y': new_y, 'index': node_position['index']}

    return denormalized_positions

def normalize_positions(denormalized_positions):
    normalized_positions = {}

    for node_label, node_position in denormalized_positions.items():
        new_x =  (float(node_position['x']) - MIN_WIDTH) / (MAX_WIDTH - MIN_WIDTH)
        new_y = (float(node_position['y']) - MIN_HEIGHT) / (MAX_HEIGHT - MIN_HEIGHT)
        normalized_positions[node_label] = {'x': new_x, 'y': new_y, 'index': node_position['index']}
        
    return normalized_positions

class LayoutTemplate:

    def generate_positions(self, nodes): raise NotImplementedError()

class RandomLayout(LayoutTemplate):

    def __init__(self) -> None:
        super().__init__()
        self._func_x = partial(random.randint, MIN_WIDTH, MAX_WIDTH)
        self._func_y = partial(random.randint, MIN_HEIGHT, MAX_HEIGHT)
        self.positions = {}

    def generate_positions(self, nodes):
        for i, node in enumerate(nodes):
            random_x = self._func_x()
            random_y = self._func_y()
            self.positions[node.label] = {'x':random_x, 'y':random_y, 'index': i}   
        return self.positions

class CircularLayout(LayoutTemplate):

    def __init__(self) -> None:
        super().__init__()
        self.radius = 200
        self.angle = 90
        self.center_x = 800
        self.center_y = 400
        self.positions = {}

    def generate_positions(self, nodes):
        next_mirrored = False
        visited_positions = set()
        quadrants = [(1,1), (1,-1), (-1,-1), (-1,1)]
        quadrant_index = 0

        for i, node in enumerate(nodes):
            while True:
                x_sign = quadrants[quadrant_index][0]
                y_sign = quadrants[quadrant_index][1]

                next_pos = [self.radius*math.cos(math.radians(self.angle)), self.radius*math.sin(math.radians(self.angle))]
                
                if next_mirrored:
                    next_pos[0], next_pos[1] = next_pos[1], next_pos[0]

                next_x = x_sign * next_pos[0] + self.center_x
                next_y = y_sign * next_pos[1] + self.center_y

                if (next_x, next_y) not in visited_positions:
                    break

                quadrant_index += 1

                if quadrant_index == 4:
                    quadrant_index = 0

                    if not next_mirrored:
                        if (next_pos[0], next_pos[1]) != (next_pos[1], next_pos[0]):
                            next_mirrored = True
                            continue

                    if next_mirrored:
                        next_mirrored = False

                    self.angle /= 2
                

            visited_positions.add((next_x, next_y))

            self.positions[node.label] = {'x': next_x, 'y': next_y, 'index': i}

           
       
        return self.positions
    
class SpringLayout(LayoutTemplate):

    def __init__(self) -> None:
        super().__init__()
        self.positions = {}

    def generate_positions(self, nodes):
        # first display nodes in random position
        self.positions = RandomLayout().generate_positions(nodes)

        edges = set()
        for node in nodes:
            for conn in node.connections:
                a, b = node.label, conn['node'].label
                if (b, a) not in edges:
                    edges.add((a, b))

        iterations = 50
        area = MAX_WIDTH * MAX_HEIGHT
        temperature = MAX_WIDTH / 10
        k = math.sqrt(area / len(nodes))

        # attractive force
        def fa(d): return (d * d) / k

        # repulsive force
        def fr(d): return (k * k) / d

        
        for _ in range(iterations):
            disp = {node.label: {'x': 0, 'y': 0} for node in nodes}

            # apply repulsive force
            for v in nodes:
                for u in nodes:
                    if u.label != v.label:
                        dx = self.positions[v.label]['x'] - self.positions[u.label]['x']
                        dy = self.positions[v.label]['y'] - self.positions[u.label]['y']
                        dist = math.hypot(dx, dy) + 0.01
                        force = fr(dist)
                        disp[v.label]['x'] += (dx / dist) * force
                        disp[v.label]['y'] += (dy / dist) * force

            # apply attractive force on the edges
            for (v_label, u_label) in edges:
                dx = self.positions[v_label]['x'] - self.positions[u_label]['x']
                dy = self.positions[v_label]['y'] - self.positions[u_label]['y']
                dist = math.hypot(dx, dy) + 0.01
                force = fa(dist)
                disp[v_label]['x'] -= (dx / dist) * force
                disp[v_label]['y'] -= (dy / dist) * force
                disp[u_label]['x'] += (dx / dist) * force
                disp[u_label]['y'] += (dy / dist) * force

            # apply the displacement based on the temperature
            for i, node in enumerate(nodes):
                dx = disp[node.label]['x']
                dy = disp[node.label]['y']
                disp_len = math.hypot(dx, dy)
                if disp_len > 0:
                    dx = (dx / disp_len) * min(disp_len, temperature)
                    dy = (dy / disp_len) * min(disp_len, temperature)

                x = self.positions[node.label]['x'] + dx
                y = self.positions[node.label]['y'] + dy

                x = min(MAX_WIDTH, max(MIN_WIDTH, x))
                y = min(MAX_HEIGHT, max(MIN_HEIGHT, y))

                self.positions[node.label] = {'x': x, 'y': y, 'index': i}

            # cooling
            temperature *= 0.95

        return self.positions