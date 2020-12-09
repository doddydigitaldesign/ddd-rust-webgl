import { CONSTANTS, elementIds } from './consts';

const asyncWasm = import('../pkg/index.js').catch(console.error);

const el = document.getElementById(elementIds.rustCanvas);
async function main() {
  const wasm = await asyncWasm;
  if (wasm) {
    const rust = await (wasm as any).default;

    var lastDrawTime = -1;

    const canvas = el as HTMLCanvasElement;

    const gl = canvas?.getContext('webgl', { antialias: true });

    if (gl && rust) {
      gl.enable(gl.BLEND);

      gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

      const client = new rust.Client();

      const initialTime = Date.now();

      const render = (time?: number) => {
        window.requestAnimationFrame(render);

        const currentTime = Date.now();

        if (currentTime >= lastDrawTime + CONSTANTS.FPS_THROTTLE) {
          lastDrawTime = currentTime;

          if (
            window.innerHeight !== canvas.height ||
            window.innerWidth !== canvas.width
          ) {
            canvas.height = window.innerHeight;
            canvas.style.height = '' + window.innerHeight;

            canvas.width = window.innerWidth;
            canvas.style.width = '' + window.innerWidth;

            gl?.viewport(0, 0, window.innerWidth, window.innerHeight);
          }

          let elapsedTime = currentTime - initialTime;

          // Rust update call
          client.update(elapsedTime, window.innerHeight, window.innerWidth);

          // Rust render call
          client.render();
        }
      };

      // Go
      render();
    }
  } else {
    console.error('Failed to initialize WebGL');
  }
}

main();
