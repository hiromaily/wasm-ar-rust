<script lang="ts">
  import * as wasm from "image-wasm";
  import { onMount } from "svelte";

  let video: HTMLVideoElement;
  let canvas: HTMLCanvasElement;
  let context: CanvasRenderingContext2D;
  let initialized = false;

  // initialization
  async function setup() {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ video: true });
      video.srcObject = stream;
      await video.play();
      initialized = true;
    } catch (error) {
      console.error("Failed to setup video stream:", error);
    }
  }

  // process each frame
  function processFrame() {
    try {
      if (!initialized || !context) return;

      context.drawImage(video, 0, 0, canvas.width, canvas.height);
      const imageData = context.getImageData(0, 0, canvas.width, canvas.height);
      // call wasm function
      const rgbaBuffer = wasm.process_image(
        new Uint8Array(imageData.data.buffer),
        canvas.width,
        canvas.height
      );

      const outputImageData = new ImageData(
        new Uint8ClampedArray(rgbaBuffer),
        canvas.width,
        canvas.height
      );

      context.putImageData(outputImageData, 0, 0);

      requestAnimationFrame(processFrame);
    } catch (error) {
      console.error("Error processing frame:", error);
    }
  }

  // full screen
  function toggleFullScreen() {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen();
    } else if (document.exitFullscreen) {
      document.exitFullscreen();
    }
  }

  // save original camera image
  function saveOriginalImage() {
    if (!context || !video) return;

    // Create a temporary canvas to capture the original video frame
    const tempCanvas = document.createElement("canvas");
    tempCanvas.width = video.videoWidth;
    tempCanvas.height = video.videoHeight;
    const tempContext = tempCanvas.getContext("2d");
    tempContext?.drawImage(video, 0, 0, video.videoWidth, video.videoHeight);

    // Get the data URL of the canvas
    const dataURL = tempCanvas.toDataURL("image/png");

    const link = document.createElement("a");
    link.href = dataURL;
    link.download = "original_image.png";
    link.click();
  }

  // save output image with wasm function
  function saveOutputImage() {
    if (!context) return;

    const imageData = context.getImageData(0, 0, canvas.width, canvas.height);
    const rgbaBuffer = new Uint8Array(imageData.data.buffer);
    // call wasm function
    const base64String = wasm.save_image(
      rgbaBuffer,
      canvas.width,
      canvas.height
    );

    const link = document.createElement("a");
    link.href = `data:image/png;base64,${base64String}`;
    link.download = "edge_detected_image.png";
    link.click();
  }

  onMount(async () => {
    try {
      await setup();

      canvas = document.createElement("canvas");
      document.body.appendChild(canvas);
      canvas.style.position = "absolute";
      canvas.style.top = "0";
      canvas.style.left = "0";
      canvas.width = video.videoWidth;
      canvas.height = video.videoHeight;
      context = canvas.getContext("2d")!;

      // button event
      document.addEventListener("keydown", (event) => {
        if (event.key === "Escape") {
          toggleFullScreen();
        } else if (event.key === "s" || event.key === "S") {
          saveOutputImage();
        } else if (event.key === "o" || event.key === "O") {
          saveOriginalImage();
        }
      });

      processFrame();
    } catch (error) {
      console.error("Error during onMount:", error);
    }
  });
</script>

<video bind:this={video} autoplay playsinline></video>
