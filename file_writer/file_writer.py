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


