const rust = import('../pkg/ddd_rust_webgl');
const canvas = document.getElementById('rust-canvas');
const gl = canvas.getContext('webgl2', { antialias: true });

rust.then((m) => {
  if (!gl) {
    alert('Failed to initialize WebGL');
    return;
  }

  const FPS_THROTTLE = 8;
  const app = new m.App();
  let lastDrawTime = -1; // In milliseconds

  function render(elapsed) {
    const dt = elapsed - lastDrawTime;

    if (dt < FPS_THROTTLE) {
      return;
    }

    if (dt >= FPS_THROTTLE) {
      lastDrawTime = elapsed;

      if (
        window.innerHeight !== canvas.height ||
        window.innerWidth !== canvas.width
      ) {
        canvas.height = window.innerHeight;
        canvas.width = window.innerWidth;

        gl.viewport(0, 0, window.innerWidth, window.innerHeight);
      }

      app.update(elapsed, dt, window.innerHeight, window.innerWidth);
      app.render();
      window.requestAnimationFrame(render);
    }
  }

  window.requestAnimationFrame(render);
});
