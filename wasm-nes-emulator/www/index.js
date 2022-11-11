import { CPU } from "wasm-nes-emulator";
import { memory } from "wasm-nes-emulator/wasm_nes_emulator_bg";
const CELL_SIZE = 3; // px
// Black White Red Cyan Purple Green Blue Yellow
//Orange Brown Light red Dark grey Grey Light green
//Light blueLight grey

const COLORS = ["#000000", "#FFFFFF"];
// Construct the universe, ...nd get its width and height.
const cpu = CPU.new();
cpu.load_pro([0xa9, 0x02, 0x00]);

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("canvas");
canvas.height = CELL_SIZE * 32;
canvas.width = CELL_SIZE * 32;

const ctx = canvas.getContext("2d");

const renderLoop = () => {
  drawPixel();
  cpu.next();

  animationId = requestAnimationFrame(renderLoop);
};

const drawPixel = () => {
  const memPtr = cpu.mem_ptr();
  const pixels = new Uint8Array(memory.buffer, memPtr, 0xffff);

  ctx.beginPath();
  for (let i = 0x0200; i <= 0x05ff; i++) {
    ctx.fillStyle = COLORS[pixels[i]];
    let r = i - 0x0200;
    let x = r % 32;
    let y = Math.floor(r / 32);
    ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
  }

  ctx.stroke();
};

drawPixel();
renderLoop();
