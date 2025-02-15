var circle;
var isDragging = false;
var isPanning = false;
var previousX, previousY;
var svg = document.querySelector("svg");

var viewBox = { x: 0, y: 0, width: 300, height: 100 };
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

document.addEventListener('mousemove', onMouseMove);
document.addEventListener('mousedown', onMouseDown);
document.addEventListener('mouseup', onMouseUp);
document.addEventListener('wheel', onWheel, { passive: false });