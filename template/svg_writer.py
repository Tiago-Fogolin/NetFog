import svg
from layouts import layouts
from style.graph_style import GraphStyle

class SVGWriter:

    def __init__(self, graph_style) -> None:
        self.elements = []
        self.markers = []
        self.centers  = {}
        self.graph_style = graph_style

    def add_node(self, x, y, label, node_index):
        self.elements.append(
            svg.Circle(
                cx=x,
                cy=y,
                stroke=self.graph_style.node_style.border_color,
                fill=self.graph_style.node_style.fill_color,
                r=20,
                class_=f'node{node_index}'
            )
        )
        self.add_label(x, y+35, label, node_index)

    def add_label(self, x, y, label, node_index):
        self.elements.append(
            svg.Text(
                text=label,
                text_anchor='middle',
                x=x,
                y=y,
                class_=f'label{node_index}'
            )
        )

    def add_line(
            self, 
            x1, 
            y1, 
            x2, 
            y2, 
            from_index, 
            to_index, 
            directed,
            weight,
            max_weight
        ):
        
        stroke_width = self.graph_style.get_line_width(weight, 1, max_weight)
        class_ = f'{from_index}line{to_index}'

        self.elements.append(
            svg.Line(
                x1=x1,
                y1=y1,
                x2=x2,
                y2=y2,
                stroke=self.graph_style.line_style.color,
                class_=class_,
                stroke_width=stroke_width
            )
        )

        if directed:
            self.add_marker_only_line(x1, y1, x2, y2, stroke_width, class_)


    def add_marker_only_line(self, x1, y1, x2, y2, stroke_width, class_):
        
        self.markers.append(
            svg.Line(
                x1=x1,
                y1=y1,
                x2=x2,
                y2=y2,
                stroke="none", 
                marker_end='url(#marker)',
                stroke_width=stroke_width,
                class_ = class_
            )
        )

    def add_arrow_ref(self):
        arrow_head = svg.Path(
            d=self.graph_style.marker_style.svg,
            fill=self.graph_style.marker_style.svg_fill
        )

        arrow_marker = svg.Marker(
            id='marker',
            markerWidth=self.graph_style.marker_style.width,
            markerHeight=self.graph_style.marker_style.height,
            refX='29',
            refY='3',
            orient='auto',
            markerUnits='strokeWidth',
            elements=[arrow_head]
        )

        defs = svg.Defs(
            elements=[arrow_marker]
        )

        self.elements.append(defs)

    def draw_graph(self, nodes, connections, layout, normalized_positions, override_positions):
        if not normalized_positions or override_positions:
            self.generate_node_positions(nodes, layout)
        else:
            self.centers = layouts.denormalize_positions(normalized_positions)

        self.add_arrow_ref()
        self.draw_lines(connections)
        self.draw_nodes(nodes)

    def generate_node_positions(self, nodes, layout):
        position_layout = layout()
        self.centers = position_layout.generate_positions(nodes)

    def draw_nodes(self, nodes):
        for node in nodes:
            self.add_node(
                x=self.centers[node.label]['x'],
                y=self.centers[node.label]['y'],
                label=node.label,
                node_index=self.centers[node.label]['index']
            )

    def draw_lines(self, connections):
        max_weight = max(map(lambda x: int(x['weight']), connections))

        for conn in connections:
            from_node = conn['from']
            to_node = conn['to']
            self.add_line(
                x1=self.centers[from_node]['x'],
                y1=self.centers[from_node]['y'],
                x2=self.centers[to_node]['x'],
                y2=self.centers[to_node]['y'],
                from_index=self.centers[from_node]['index'],
                to_index=self.centers[to_node]['index'],
                directed=conn['directed'],
                weight=conn['weight'],
                max_weight=max_weight
            )

        if self.markers:
            self.elements.append(self.markers)

    def get_svg(self):
        return svg.SVG(
            width="100%",
            height="100%",
            viewBox="20 20 1480 680",
            elements=self.elements
        )
