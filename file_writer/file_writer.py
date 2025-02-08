import json

class FileWriterTemplate:
    def write_file(self, path: str): raise NotImplementedError()

class NetFileWriter(FileWriterTemplate):
    
    def write_file(self, path: str, nodes, edges, arcs):
        with open(path, 'w', encoding='utf-8') as file:
            node_count = len(nodes)

            file.write(f"*vertices {node_count}\n")
            for label, values in nodes.items():
                index = values['index']
                x = values['x']
                y = values['y']
                if not x and not y:
                    text = f'{index} "{label}"\n'
                else:
                    text = f'{index} "{label}" {x} {y}\n'

                file.write(text)

            if edges:
                file.write(f"*edges\n")
                for from_, to_, weight_ in edges:
                    file.write(f"{from_} {to_} {weight_}\n")

            if arcs:
                file.write(f"*arcs\n")
                for from_, to_, weight_ in arcs:
                    file.write(f"{from_} {to_} {weight_}\n")

class JsonFileWriter(FileWriterTemplate):
    
    def write_file(self, path: str, nodes, edges, arcs):
        final_dict = {
            'nodes': []
        }

        for node in nodes:
            
            node_obj = {
                'label': node.label
            }

            final_dict['nodes'].append(node_obj)

        for from_, to_, weight_ in edges:
            if not final_dict.get('edges', None):
                final_dict['edges'] = []
            
            edge_obj = {
                "source": from_,
                "target": to_,
                "weight": weight_
            }

            final_dict['edges'].append(edge_obj)

        for from_, to_, weight_ in arcs:
            if not final_dict.get('arcs', None):
                final_dict['arcs'] = []
            
            edge_obj = {
                "source": from_,
                "target": to_,
                "weight": weight_
            }

            final_dict['arcs'].append(edge_obj)


        with open(path, 'w', encoding='utf-8') as file:
           json.dump(final_dict, file, indent=4)



