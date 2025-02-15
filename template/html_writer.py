class HtmlWriter:

    def __init__(self, svg):
        self.svg = svg

    def __read_file(self, file_path):
        with open(file_path, 'r', encoding='utf8') as file:
            return file.read()
        
    def output(self, file_name):
        html_string = self.__read_file('template/template.html')
        script_string = self.__read_file('template/script.js')

        svg_string = html_string.replace('ESCAPE_SVG', self.svg)

        final_string  = svg_string.replace('ESCAPE_SCRIPT', script_string)

        with open(file_name, 'w', encoding='utf8') as file:
            file.write(final_string)

