class NodeConnections{

    constructor(node_labels, connections, container){
        this.nodes = node_labels
        this.connections = connections
        this.container = container
        this.centerPositions = []
    }

    createRandomX(){
        return Math.floor(Math.random() * (window.innerWidth - 50))
    }

    createRandomY(){
        return Math.floor(Math.random() * (window.innerHeight - 50))
    }

    createNodeLabel(node_label, node_rect){
        let label = document.createElement('label')
        label.innerText = node_label


        label.classList.add('label');
        label.style.top = node_rect.bottom - 10 + 'px'
        label.style.left = node_rect.left + 'px'

        return label
    }

    createNode(){
        let node = document.createElement('div')
        node.classList.add('node')
        
        let positionX = this.createRandomX()
        let positionY = this.createRandomY()

        node.style.left = positionX + 'px'
        node.style.top = positionY + 'px'

        return node
    }

    getNodeCenters(node_rect){
        let centerX = node_rect.left + node_rect.width / 2
        let centerY = node_rect.top + node_rect.height / 2

        return [centerX, centerY]
    }

    drawNodes(){
        for(let i = 0; i < this.nodes.length; i++){
            let node = this.createNode()

            this.container.appendChild(node)

            let nodeRect = node.getBoundingClientRect()

            let label = this.createNodeLabel(this.nodes[i], nodeRect)
                    
            this.container.appendChild(label)
        
            this.centerPositions[this.nodes[i]] = this.getNodeCenters(nodeRect)
        }
    }

    getXPosition(node){
        return this.centerPositions[node][0]
    }

    getYPosition(node){
        return this.centerPositions[node][1]
    }

    drawConnections(){
        for (let i = 0 ; i < this.connections.length; i++){
            ctx.beginPath();
        
            let from_node = this.connections[i].from;
            let to_node = this.connections[i].to;
        
            ctx.moveTo(this.getXPosition(from_node), this.getYPosition(from_node));
            ctx.lineTo(this.getXPosition(to_node), this.getYPosition(to_node));
            ctx.strokeStyle = '#000';
            ctx.stroke();   
        
            // var arrowSize = 10;
            // var endY = positions[to_node][1];
            // var startY = positions[from_node][1];
            // var endX = positions[to_node][0];
            // var startX = positions[from_node][0]
        
            // var angle = Math.atan2(endY - startY, endX - startX);
            // ctx.beginPath();
            // ctx.moveTo(endX, endY);
            // ctx.lineTo(endX - arrowSize * Math.cos(angle - Math.PI / 6), endY - arrowSize * Math.sin(angle - Math.PI / 6));
            // ctx.lineTo(endX - arrowSize * Math.cos(angle + Math.PI / 6), endY - arrowSize * Math.sin(angle + Math.PI / 6));
            // ctx.closePath();
            // ctx.fill();
        }
    }
}

var canvas = document.getElementById('lineCanvas');
var ctx = canvas.getContext('2d');

nodes = ESCAPE_NODES

connections = ESCAPE_CONNECTIONS

positions = {}

container = document.getElementById('container')

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

let node_connections = new NodeConnections(nodes, connections, container)

node_connections.drawNodes()
node_connections.drawConnections()