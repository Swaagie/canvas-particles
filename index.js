import { Universe, Body } from './pkg';
import { v4 as uuidv4 } from 'uuid';

const fps = new class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = 1 / delta * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
  }
};

const colors = Array.from(new Set(['green', 'red', 'yellow', 'orange', 'whitesmoke']));
const dimensions = {
  x: window.innerWidth,
  y: window.innerHeight
};

const canvas = document.getElementById('drawing');
const ctx = canvas.getContext('2d');

// Set dimension on the canvas
canvas.height = dimensions.y;
canvas.width = dimensions.x;

const universe = Universe.new();
const bodies = new Map();

// Add bodies
// math.random() needs to be called multiple times for the RNG to be actually random within on cpu tick
for (let index = 0; index < 10; index++) {
  const mass = Math.random() * 1e5;
  const color = colors[Math.floor(Math.random() * colors.length)];

  bodies.set(uuidv4(), {
    mass,
    body: Body.new(
      mass,
      color,
      [0.0, dimensions.y / 2.0, 0.0],
      [Math.random() * 50, Math.random() * 100.0, 0.0]
    ),
    color
  });
}

for (const [id, { body }] of bodies) {
  universe.add_body(id, body);
}

(function renderLoop() {
  /*ctx.clearRect(0, 0, canvas.width, canvas.height);

  for (const [id, { mass, color }] of bodies) {
    const position = universe.get_position(id);

    ctx.beginPath();
    ctx.arc(position[0], -position[1] + dimensions.y, Math.round(mass / 10e3), 0, 2 * Math.PI, false);
    ctx.fillStyle = color;
    ctx.fill();
  }*/

  fps.render();
  universe.tick();
  universe.render(ctx);

  requestAnimationFrame(renderLoop);
})();