class MarkerStyle:
    
    def __init__(self, svg='M0,0 L0,6 L9,3 z', svg_fill='red', width=30, height=30):
        self.svg = svg
        self.svg_fill = svg_fill
        self.width = str(width)
        self.height = str(height)