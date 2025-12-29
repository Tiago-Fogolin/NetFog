var circle;
var isDragging = false;
var isPanning = false;
var previousX, previousY;
var svg = document.querySelector("svg");

var viewBox = { x: 0, y: 0, width: 1500, height: 700 };
svg.setAttribute("viewBox", `${viewBox.x} ${viewBox.y} ${viewBox.width} ${viewBox.height}`);


function moveCircle(dx, dy) {
    let nodeIndex = getNodeIndex();
    let label = getLabel(nodeIndex);
    let lines = getLines(nodeIndex);

    dx *= viewBox.width / svg.clientWidth;
    dy *= viewBox.height / svg.clientHeight;

    var cx = parseFloat(circle.getAttribute('cx'));
    var cy = parseFloat(circle.getAttribute('cy'));

    moveLabel(label, dx, dy);
    moveLines(lines[0], lines[1], dx, dy);

    circle.setAttribute('cx', cx + dx);
    circle.setAttribute('cy', cy + dy);

}


function getNodeIndex(){
    let className = circle.classList[0];
    let index = className.split('node')[1];
    return index;
}

function getLabel(index){
    let label = document.querySelector(`[class="label${index}"]`);
    return label
}

function getLines(nodeIndex){
    let pos1Lines = document.querySelectorAll(`[class^="${nodeIndex}line"]`);
    let pos2Lines = document.querySelectorAll(`[class$="line${nodeIndex}"]`);

    return [pos1Lines, pos2Lines]
}

function moveLabel(label, dx, dy){
    let x = parseFloat(label.getAttribute('x'));
    let y = parseFloat(label.getAttribute('y'));

    label.setAttribute('x', x + dx);
    label.setAttribute('y', y + dy);
}

function moveLines(pos1lines, pos2lines, dx, dy){
    for(let i = 0 ; i < pos1lines.length; i++){
        let line = pos1lines[i];
        let x1 = parseFloat(line.getAttribute('x1'));
        let y1 = parseFloat(line.getAttribute('y1'));

        line.setAttribute('x1', x1 + dx);
        line.setAttribute('y1', y1 + dy);
    }

    for(let i = 0 ; i < pos2lines.length; i++){
        let line = pos2lines[i];
        let x2 = parseFloat(line.getAttribute('x2'));
        let y2 = parseFloat(line.getAttribute('y2'));

        line.setAttribute('x2', x2 + dx);
        line.setAttribute('y2', y2 + dy);
    }

}

function onMouseMove(event) {
    if (isDragging) {
        var mouseX = event.clientX;
        var mouseY = event.clientY;
        var dx = mouseX - previousX;
        var dy = mouseY - previousY;
        moveCircle(dx, dy);
        previousX = mouseX;
        previousY = mouseY;
    } else if (isPanning) {
        var mouseX = event.clientX;
        var mouseY = event.clientY;
        var dx = mouseX - previousX;
        var dy = mouseY - previousY;
        panSVG(dx, dy);
        previousX = mouseX;
        previousY = mouseY;
    }
}

function panSVG(dx, dy) {
    viewBox.x -= dx * (viewBox.width / window.innerWidth);
    viewBox.y -= dy * (viewBox.height / window.innerHeight);
    svg.setAttribute("viewBox", `${viewBox.x} ${viewBox.y} ${viewBox.width} ${viewBox.height}`);
}

function onMouseDown(event) {
    if (event.target instanceof SVGCircleElement) {
        isDragging = true;
        previousX = event.clientX;
        previousY = event.clientY;
        circle = event.target;
    } else {
        isPanning = true;
        previousX = event.clientX;
        previousY = event.clientY;
    }
}


function onMouseUp() {
    isDragging = false;
    isPanning = false;
}

function onWheel(event) {
    event.preventDefault();
    const zoomFactor = 0.9;

    const scale = event.deltaY < 0 ? zoomFactor : (1 / zoomFactor);

    const mouseXRatio = (event.clientX - svg.getBoundingClientRect().left) / svg.clientWidth;
    const mouseYRatio = (event.clientY - svg.getBoundingClientRect().top) / svg.clientHeight;

    const newWidth = viewBox.width * scale;
    const newHeight = viewBox.height * scale;

    viewBox.x -= (newWidth - viewBox.width) * mouseXRatio;
    viewBox.y -= (newHeight - viewBox.height) * mouseYRatio;
    viewBox.width = newWidth;
    viewBox.height = newHeight;

    // Atualizar o viewBox do SVG
    svg.setAttribute("viewBox", `${viewBox.x} ${viewBox.y} ${viewBox.width} ${viewBox.height}`);
}

function toggleMenu() {
    const menu = document.getElementById("sideMenu");
    menu.classList.toggle("open");
}

function parseSVG() {
    const svg = document.querySelector("svg");

    const nodes = [];
    const edges = [];
    const arcs = [];
    const labels = {};


    svg.querySelectorAll("text").forEach(label => {
        const className = label.getAttribute("class") || "";
        const match = className.match(/label(\d+)/);
        if (match) {
            labels[match[1]] = label.textContent.trim();
        }
    });

    svg.querySelectorAll("circle").forEach(circle => {
        const className = circle.getAttribute("class") || "";
        const match = className.match(/node(\d+)/);
        if (match) {
            const index = match[1];
            nodes.push({
                x: parseFloat(circle.getAttribute("cx")),
                y: parseFloat(circle.getAttribute("cy")),
                radius: parseFloat(circle.getAttribute("r")),
                className,
                label: labels[index] || "",
                index: index
            });
        }
    });


    svg.querySelectorAll("line").forEach(line => {
        const className = line.getAttribute("class") || "";
        const match = className.match(/(\d+)line(\d+)/);
        if (match) {
            const sourceIndex = match[1];
            const targetIndex = match[2];

            const connection = {
                x1: parseFloat(line.getAttribute("x1")),
                y1: parseFloat(line.getAttribute("y1")),
                x2: parseFloat(line.getAttribute("x2")),
                y2: parseFloat(line.getAttribute("y2")),
                className,
                source: sourceIndex,
                target: targetIndex
            };

            if (line.hasAttribute("marker-end") && line.getAttribute("marker-end") !== "") {
                arcs.push(connection);
            } else {
                edges.push(connection);
            }
        }
    });

    return { nodes, edges, arcs , labels };
}

function writePajek() {
    var {nodes, edges, arcs, labels} = parseSVG();
    const [initialWidth, initialHeight] = [1500,700];

    let content = `*Vertices ${Object.keys(nodes).length}\n`;

    for (const [idx, values] of Object.entries(nodes)) {
        const index = values.index;
        const x = values.x;
        const y = values.y;
        const label = values.label;

        const normX = (x / initialWidth).toFixed(4);
        const normY = (y / initialHeight).toFixed(4);

        content += `${index} \"${label}\" ${normX} ${normY}\n`;
    }

    if (Array.isArray(edges) && edges.length > 0) {
        content += `*Edges\n`;
        for (const edge of edges) {
            const { source, target} = edge;
            content += `${source} ${target} 1\n`;
        }
    }

    if (Array.isArray(arcs) && arcs.length > 0) {
        content += `*Arcs\n`;
        for (const arc of arcs) {
            const { source, target} = arc;
            content += `${source} ${target} 1\n`;
        }
    }

    const blob = new Blob([content], { type: "text/plain" });
    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = "data.net";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
}

function writeJson() {
    var {nodes, edges, arcs} = parseSVG();
    var final_dict = {
        'nodes': [],
        'edges': [],
        'arcs': []
    };

    nodes.forEach((e) => {
        let obj = {
            'label': e.label
        };
        final_dict['nodes'].push(obj);
    });


    edges.forEach((e) => {
        let edge_obj = {
            "source": e.source,
            "target": e.target
        };
        final_dict['edges'].push(edge_obj);
    });

    arcs.forEach((e) => {
        let arc_obj = {
            "source": e.source,
            "target": e.target
        };
        final_dict['arcs'].push(arc_obj);
    });

    var conteudoJson = JSON.stringify(final_dict);
    var blob = new Blob([conteudoJson], { type: "application/json" });

    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = "data.json";

    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
}


document.addEventListener('mousemove', onMouseMove);
document.addEventListener('mousedown', onMouseDown);
document.addEventListener('mouseup', onMouseUp);
document.addEventListener('wheel', onWheel, { passive: false });
