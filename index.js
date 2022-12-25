import('./pkg')
    .then(wasm => {
        let content = document.getElementById("content");
        let input = `Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi`;
        let grid = new wasm.Grid(input);
        content.innerHTML = grid.to_svg();
    })
    .catch(console.error);