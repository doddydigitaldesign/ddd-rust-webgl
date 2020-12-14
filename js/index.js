// import { CONSTANTS, elementIds } from './consts';

// const asyncWasm = import('../pkg/ddd_rust_webgl');

// const el = document.getElementById(elementIds.rustCanvas);
// async function main() {
//   const wasm = await asyncWasm;
//   if (wasm) {
//     const rust = await (wasm as any).default;

//     var lastDrawTime = -1;

//     const canvas = el as HTMLCanvasElement;

//     const gl = canvas?.getContext('webgl', { antialias: true });

//     if (gl && rust) {
//       const app = new rust.App();

//       const initialTime = Date.now();

//       const render = (timeStamp: number) => {
//         window.requestAnimationFrame(render);

//         const currentTime = Date.now();
//         // console.log('currentTime:', currentTime);

//         if (currentTime >= lastDrawTime + CONSTANTS.FPS_THROTTLE) {
//           lastDrawTime = currentTime;

//           if (
//             window.innerHeight !== canvas.height ||
//             window.innerWidth !== canvas.width
//           ) {
//             canvas.height = window.innerHeight;
//             // canvas.clientHeight = window.innerHeight;
//             canvas.style.height = '' + window.innerHeight + 'px';

//             canvas.width = window.innerWidth;
//             // canvas.clientWidth = window.innerWidth;
//             canvas.style.width = '' + window.innerWidth + 'px';

//             gl.viewport(0, 0, window.innerWidth, window.innerHeight);

//             gl?.viewport(0, 0, window.innerWidth, window.innerHeight);
//           }

//           let elapsedTime = currentTime - initialTime;

//           // Rust update call
//           app.update(elapsedTime, window.innerHeight, window.innerWidth);

//           // Rust render call
//           app.render();
//         }
//       };

//       // Go
//       render(Date.now());
//     }
//   } else {
//     console.error('Failed to initialize WebGL');
//   }
// }

// main();

const rust = import('../pkg/ddd_rust_webgl');
const canvas = document.getElementById('rust-canvas');
const gl = canvas.getContext('webgl', { antialias: true });

rust.then((m) => {
  if (!gl) {
    alert('Failed to initialize WebGL');
    return;
  }

  const FPS_THROTTLE = 1000.0 / 30.0; // milliseconds / frames
  const dougsClient = new m.App();
  const initialTime = Date.now();
  let lastDrawTime = -1; // In milliseconds

  function render() {
    window.requestAnimationFrame(render);
    const currTime = Date.now();

    if (currTime >= lastDrawTime + FPS_THROTTLE) {
      lastDrawTime = currTime;

      if (
        window.innerHeight !== canvas.height ||
        window.innerWidth !== canvas.width
      ) {
        canvas.height = window.innerHeight;
        canvas.clientHeight = window.innerHeight;
        canvas.style.height = window.innerHeight;

        canvas.width = window.innerWidth;
        canvas.clientWidth = window.innerWidth;
        canvas.style.width = window.innerWidth;

        gl.viewport(0, 0, window.innerWidth, window.innerHeight);
      }

      let elapsedTime = currTime - initialTime;
      dougsClient.update(elapsedTime, window.innerHeight, window.innerWidth);
      dougsClient.render();
    }
  }

  render();
});
