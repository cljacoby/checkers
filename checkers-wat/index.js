var checkers;

function notifyPieceCrowned(x, y) {
    console.log(`A piece was crowned at x=${x}, y=${y}`);
}

function notifyPieceMoved(fromX, fromY, toX, toY) {
    console.log(`A piece was moved from x=${fromX}, y=${fromY},
        to from x=${toX}, y=${toY}`);
}

// var white = 2,
//     black = 1,
//     crowned_white = 6,
//     crowned_black = 5;
// var whiteIsWhite = checkers.instance.exports.isWhite(white),
//     blackIsBlack = checkers.instance.exports.isWhite(black),
//     blackIsBlack = checkers.instance.exports.isWhite(black),
//     blackIsWhite = checkers.instance.exports.isWhite(black);
// let results = [
//     { label: "white", value: white },
//     { label: "black", value: black },
//     { label: "crowned_white", value: crowned_white },
//     { label: "crowned_black", value: crowned_black },
//     { label: "isWhite(white)", value: checkers.instance.exports.isWhite(white) },
//     { label: "isBlack(black)", value: checkers.instance.exports.isBlack(black) },
//     { label: "isWhite(black)", value: checkers.instance.exports.isWhite(black) },
//     { label: "isCrowned(crowned_white)", value: checkers.instance.exports.isCrowned(crowned_white) },
//     { label: "isCrowned(crowned_black)", value: checkers.instance.exports.isCrowned(crowned_black) },
//     { label: "withoutCrown(crowned_white)", value: checkers.instance.exports.withoutCrown(crowned_white) },
//     { label: "withoutCrown(crowned_black)", value: checkers.instance.exports.withoutCrown(crowned_black) },
// ];
// console.table(results);

(async function main() {

    let imports = {
        events: {
            notifyPieceCrowned,
            notifyPieceMoved
        }
    };
    console.table(imports);

    let resp = await fetch('./checkers.wasm');
    let bytes = await resp.arrayBuffer();
    checkers = await WebAssembly.instantiate(bytes, imports);
    console.log(checkers);

    checkers.instance.exports.initBoard();
    checkers.instance.exports.move(0, 5, 0, 4); // black's turn
    checkers.instance.exports.move(1, 0, 1, 1); // white's turn


})();