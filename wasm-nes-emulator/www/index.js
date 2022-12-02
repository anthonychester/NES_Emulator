import { CPU } from "wasm-nes-emulator";
import { memory } from "wasm-nes-emulator/wasm_nes_emulator_bg";
const CELL_SIZE = 8; // px
// Black White Red Cyan Purple Green Blue Yellow
//Orange Brown Light red Dark grey Grey Light green
//Light blueLight grey

//Button press for next and display a and pc

const COLORS = ["#000000", "#FFFFFF"];
// Construct the universe, ...nd get its width and height.
const cpu = CPU.new();

/*
0x85, STA Zero Page
0xa9, LDA Zero Page, X
0x85, STA Zero Page
0xa9, LDA Zero Page, X
0xa2, LDX Immediate
0x91, STA Indirect, Y
0xc8, INY
0xc0, CPY Immediate
0xd0  BNE -
0x91, STA Indirect, Y
0x8a, TXA
0x65, ADC Zero Page
0xaa, TAX
0xa9, LDA Immediate
0x86, STX Zero Page
0xa0, LDY Immediate
0xe0, CPX Immediate
0xd0, BNE -
0x00, BRK
 */

const all_screen = [
0xa9, 0x01, 0x85, 0x10, 0xa9, 0x00, 0x85, 0x01, 0xa9, 0x02, 0x85, 0x02, 0xa5, 0x10, 0xa2, 0x02,
0xa0, 0x00, 0x91, 0x01, 0xc8, 0xc0, 0xff, 0xd0, 0xf9, 0x91, 0x01, 0x8a, 0x65, 0x00, 0xaa, 0xa5,
0x10, 0x86, 0x02, 0xa0, 0x00, 0xe0, 0x06, 0xd0, 0xe9, 0x20, 0x2f, 0x08, 0x4c, 0x04, 0x08, 0xa6,
0x10, 0xe8, 0x86, 0x10, 0x60, 0x00
];

cpu.load_pro(all_screen);

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("canvas");
canvas.height = CELL_SIZE * 32;
canvas.width = CELL_SIZE * 32;

const ctx = canvas.getContext("2d");

document.getElementById("next").addEventListener("click", (event) => {
  //cpu.next();
});

let not_one = false;
let over_2 = false;
const renderLoop = () => {
  while (cpu.update == false) {
    cpu.next();
    if (cpu.register_x > 2) {
      over_2 = true;
    }
    document.getElementById("check").innerText = over_2;
  }
  //window.alert("end");
  drawPixel();
  cpu.reset_update();
  //cpu.next();
  document.getElementById("pc").innerText =
    "pc: 0x" + cpu.program_counter.toString(16);
  document.getElementById("reg").innerText =
    "o2: " +
    over_2 +
    ", a: " +
    cpu.register_a +
    ", x: " +
    cpu.register_x +
    ", y: " +
    cpu.register_y;
  ///all_white[cpu.program_counter - 0x8000].toString(16);
  requestAnimationFrame(renderLoop);
};

const drawPixel = () => {
  const memPtr = cpu.mem_ptr();
  const pixels = new Uint8Array(memory.buffer, memPtr, 0xffff);

  document.getElementById("mem").innerText = pixels[0x0002].toString(16);

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

/*
0x85, STA Zero Page
0xa9, LDA Zero Page, X
0xa2, LDX Immediate
0x91, STA Indirect, Y
0xc8, INY
0xc0, CPY Immediate
0xd0  BNE -

0x8a, TXA
0x65, ADC Zero Page
0xaa, TAX
0x86, STX Zero Page
0xa0, LDY Immediate
0xe0, CPX Immediate
0x00, BRK
*/
