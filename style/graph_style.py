from style.node_style import NodeStyle
from style.line_style import LineStyle
from style.marker_style import MarkerStyle

class GraphStyle:
    
    def __init__(self, node_style = NodeStyle(), line_style = LineStyle(), marker_style = MarkerStyle()):
        self.node_style = node_style
        self.line_style = line_style
        self.marker_style = marker_style

    def get_line_width(self, weight, min_weight, max_weight):
        return self.line_style.get_width(int(weight), int(min_weight), max_weight)
        
    def set_line_width(self, min_width, max_width):
        self.line_style.min_width = min_width
        self.line_style.max_width = max_width
            