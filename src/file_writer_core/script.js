const { init, createRandomGraph, loadGraph, exportJson, exportPajek } = window.NetFogViewer;

function toggleMenu() {
    const menu = document.getElementById("sideMenu");
    menu.classList.toggle("open");
}

function writeJson() {
    exportJson();
}

function writePajek() {
    exportPajek();
}

(async function () {
    const container = document.getElementById('graph');
    await init(container);
    await loadGraph(GRAPH_DATA);
})();
