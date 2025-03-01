class LineStyle:

    
    def __init__(self, color='black', min_width=1, max_width=5, dynamic_weight_size=True):
        self.color = color
        self.min_width = min_width
        self.max_width = max_width
        self.dynamic_weight_size = dynamic_weight_size

    def get_width(self, weight, min_weight, max_weight):
        if not self.dynamic_weight_size or min_weight == max_weight:
            return str(self.min_width)
        
        normalized_weight = (weight - min_weight) / (max_weight - min_weight)
        return str(self.min_width + normalized_weight * (self.max_width - self.min_width))