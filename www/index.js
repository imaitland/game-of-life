// Import the WebAssembly memory at the top of the file.
import { memory } from "wasm-game-of-life/game_of_life_bg";
import { Universe, Cell } from "wasm-game-of-life";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#000000";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const WIDTH = window.innerWidth / CELL_SIZE;
const HEIGHT = window.innerHeight / CELL_SIZE;

// Construct the universe.
const universe = Universe.new(WIDTH, HEIGHT);

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * HEIGHT + 1;
canvas.width = (CELL_SIZE + 1) * WIDTH + 1;

const ctx = canvas.getContext("2d");

const translateCanvasClickToCell = (event) => {
  const rect = canvas.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;

  // since each cell is 10px by 10px with 1px border.
  let cellX = Math.floor(x / (CELL_SIZE + 1));
  let cellY = Math.floor(y / (CELL_SIZE + 1));

  universe.toggle_cell(cellY, cellX);
  console.log("CLICK");
  drawGrid();
  drawCells();
};

canvas.addEventListener("click", (event) => {
  console.log("CANVAS");
  translateCanvasClickToCell(event);
});

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= WIDTH; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * HEIGHT + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= HEIGHT; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * WIDTH + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * WIDTH + column;
};

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << n % 8;
  return (arr[byte] & mask) === mask;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, (WIDTH * HEIGHT) / 8); // cells is a bitset, single bits...
  ctx.beginPath();

  for (let row = 0; row < HEIGHT; row++) {
    for (let col = 0; col < WIDTH; col++) {
      const idx = getIndex(row, col);
      ctx.fillStyle = bitIsSet(idx, cells) ? ALIVE_COLOR : DEAD_COLOR;
      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
  ctx.stroke();
};

let animationId = null;

const renderLoop = () => {
  universe.tick();

  drawGrid();
  drawCells();

  animationId = requestAnimationFrame(renderLoop);
};

const playPauseButton = document.getElementById("play-pause-button");
const clearButton = document.getElementById("clear-button");
const gliderButton = document.getElementById("gospers-glider-button");

const isPaused = () => {
  return animationId === null;
};

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "🧬";
  cancelAnimationFrame(animationId);
  animationId = null;
};

const clear = () => {
  universe.reset_universe();
};

playPauseButton.addEventListener("click", (event) => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

clearButton.addEventListener("click", (event) => {
  console.log("CLEAR");
  clear();
  drawGrid();
  drawCells();
});

gliderButton.addEventListener("click", (event) => {});

// start the first iteration of the rendering loop.
drawGrid();
drawCells();
play();
