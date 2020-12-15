const rust = import('../pkg/ddd_rust_webgl');
const canvas = document.getElementById('rust-canvas');
const gl = canvas.getContext('webgl', { antialias: true });

rust.then((m) => {
  if (!gl) {
    alert('Failed to initialize WebGL');
    return;
  }

  const FPS_THROTTLE = 1000.0 / 60.0; // ms / fps
  const app = new m.App();
  let lastDrawTime = -1; // In milliseconds

  function render(elapsed) {
    const currTime = elapsed;
    const dt = currTime - lastDrawTime + FPS_THROTTLE;

    if (dt >= FPS_THROTTLE) {
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

      app.update(elapsed, window.innerHeight, window.innerWidth);
      app.render();
      window.requestAnimationFrame(render);
    }
  }

  window.requestAnimationFrame(render);
});
