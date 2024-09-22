<script lang="ts">
import * as wasm from "image-wasm";
import { onMount } from "svelte";
import { saveOriginalImage, saveOutputImage } from "../images";
import Help from "./Help.svelte";

// Element
let video: HTMLVideoElement;
let canvas: HTMLCanvasElement;
let context: CanvasRenderingContext2D;

// Flag
let initialized = false;
let showHelp = false;

// FPS
let fps = 0;
let lastTimestamp = 0;

// initialization
const setupVideo = async () => {
  try {
    const stream = await navigator.mediaDevices.getUserMedia({ video: true });
    video.srcObject = stream;
    await video.play();
    initialized = true;
  } catch (error) {
    console.error("Failed to setup video stream:", error);
  }
};

const setupCanvas = () => {
  canvas = document.createElement("canvas");
  document.body.appendChild(canvas);
  canvas.style.position = "absolute";
  canvas.style.top = "0";
  canvas.style.left = "0";
  canvas.width = video.videoWidth;
  canvas.height = video.videoHeight;
  context = canvas.getContext("2d")!;
};

const setupEvent = () => {
  // button event
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      toggleFullScreen();
    } else if (event.key === "s" || event.key === "S") {
      console.log("S: saveOutputImage");
      saveOutputImage(context, canvas);
    } else if (event.key === "o" || event.key === "O") {
      console.log("O: saveOriginalImage");
      saveOriginalImage(context, video);
    } else if (event.key === "h" || event.key === "H") {
      showHelp = !showHelp;
    } else if (event.key === "w" || event.key === "W") {
      toggleCanvasFullScreen();
    }
  });
};

// process each frame
const processFrame = (timestamp: number) => {
  try {
    if (!initialized || !context) return;

    // calculate and update FPS
    if (lastTimestamp) {
      const delta = (timestamp - lastTimestamp) / 1000;
      fps = Math.round(1 / delta);
    }
    lastTimestamp = timestamp;

    // video image
    context.drawImage(video, 0, 0, canvas.width, canvas.height);
    const imageData = context.getImageData(0, 0, canvas.width, canvas.height);
    // call wasm function
    const rgbaBuffer = wasm.process_image(
      new Uint8Array(imageData.data.buffer),
      canvas.width,
      canvas.height,
    );

    const outputImageData = new ImageData(
      new Uint8ClampedArray(rgbaBuffer),
      canvas.width,
      canvas.height,
    );
    // reflect images on canvas context
    context.putImageData(outputImageData, 0, 0);

    requestAnimationFrame(processFrame);
  } catch (error) {
    console.error("Error processing frame:", error);
  }
};

// full screen
const toggleFullScreen = () => {
  if (!document.fullscreenElement) {
    document.documentElement.requestFullscreen();
  } else if (document.exitFullscreen) {
    document.exitFullscreen();
  }
};

// full screen for canvas
const toggleCanvasFullScreen = () => {
  if (canvas.requestFullscreen) {
    canvas.requestFullscreen();
  }
};

onMount(async () => {
  try {
    // initialize
    await setupVideo();
    setupCanvas();
    setupEvent();

    processFrame();

    // help
    showHelp = true;
  } catch (error) {
    console.error("Error during onMount:", error);
  }
});
</script>

<video bind:this={video} autoplay playsinline></video>

<div
  id="fps"
  style="position:absolute; bottom: 10px; left: 10px; color: white; background-color: rgba(0, 0, 0, 0.7); padding: 5px;"
>
  {fps} FPS
</div>

{#if showHelp}
  <Help />
{/if}
