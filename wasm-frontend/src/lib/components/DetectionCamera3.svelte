<script lang="ts">
import * as wasm from "image-detect-mozaic-wasm";
import { onMount } from "svelte";
import { saveOriginalImage, saveOutputImage } from "../image-libs";
import { checkMobileDevice } from "../ua";
import Help from "./Help.svelte";

// DetectionCamera
//  Changed to hide the video element and make the drawing canvas full screen.
//
// 1. 3 layer from the bottom
//  - video
//  - videoCanvas for input
//  - drawingCanvas for drawing from wasm function
//
// 2. call detect_draw_image() as wasm function for detecting image location
//    and apply mozaic effect
//  - detect_draw_image works! with drawingCanvas
// 3. reflect output image into drawingCanvas

// wasm response
interface WasmResponse {
  raw_data: number[];
  min_value: number;
  min_value_location: [number, number];
}

// video
let video: HTMLVideoElement;

// canvas for input data for wasm function
let videoCanvas: HTMLCanvasElement; // reference of HTML <canvas> element
let videoContext: CanvasRenderingContext2D; // 2d rendering context of canvas

// canvas for output data by drawing
let drawingCanvas: HTMLCanvasElement; // reference of HTML <canvas> element
let drawingContext: CanvasRenderingContext2D; // 2d rendering context of canvas

// Flag
let initialized = false;
let showHelp = false;

// FPS
let fps = 0;
let lastTimestamp = 0;

// Wasm instance
//const effectMode = 1; // effect mode: 1: mozaic, 2: canny edge
const effectMode = Math.random() < 0.5 ? 1 : 2;

const maxCount = 50;
const threshold = 4500.0;
const isRectangle = true;
const imageDetector = new wasm.ImageDetector(
  effectMode,
  maxCount,
  threshold,
  isRectangle,
);

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

// for only generating image for wasm
const setupVideoCanvas = () => {
  videoCanvas = document.createElement("canvas");

  // Note:
  //  Don't change size, it would break input data of wasm function
  videoCanvas.width = video.videoWidth; // 640
  videoCanvas.height = video.videoHeight; // 480
  videoContext = videoCanvas.getContext("2d")!;
};

const setupDrawingCanvas = () => {
  drawingCanvas = document.createElement("canvas");
  const app = document.getElementById("app");
  app!.appendChild(drawingCanvas);

  drawingCanvas.width = video.videoWidth; // 640
  drawingCanvas.height = video.videoHeight; // 480

  drawingCanvas.style.display = "block";
  drawingCanvas.style.width = "100%";
  drawingCanvas.style.height = "100%";
  drawingCanvas.style.objectFit = "contain";
  drawingCanvas.style.objectPosition = "center";

  drawingContext = drawingCanvas.getContext("2d")!;

  // invisible
  //drawingCanvas.style.display = "none";
};

const setupEvent = (
  context: CanvasRenderingContext2D,
  canvas: HTMLCanvasElement,
) => {
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
      toggleCanvasFullScreen(canvas);
    }
  });
};

// process each frame
const processFrame = async (timestamp: number) => {
  try {
    if (!initialized || !videoContext) return;

    // calculate and update FPS
    if (lastTimestamp) {
      const delta = (timestamp - lastTimestamp) / 1000;
      fps = Math.round(1 / delta);
    }
    lastTimestamp = timestamp;

    // video image
    videoContext.drawImage(video, 0, 0, videoCanvas.width, videoCanvas.height);
    const imageData = videoContext.getImageData(
      0,
      0,
      videoCanvas.width,
      videoCanvas.height,
    );

    // call wasm function
    console.log("call imageDetector.detect_image_and_mozaic()");
    const response = await imageDetector.detect_image_and_mozaic(
      new Uint8Array(imageData.data.buffer),
      videoCanvas.width,
      videoCanvas.height,
    );
    // check response if error
    if (response instanceof Error) throw response;
    const wasmResp = response as unknown as WasmResponse;

    // drawing
    const rgbaBuffer = wasmResp.raw_data;
    const outputImageData = new ImageData(
      new Uint8ClampedArray(rgbaBuffer),
      drawingCanvas.width,
      drawingCanvas.height,
    );
    // reflect output images on canvas context
    drawingContext.putImageData(outputImageData, 0, 0);

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
const toggleCanvasFullScreen = (canvas: HTMLCanvasElement) => {
  if (canvas.requestFullscreen) {
    canvas.requestFullscreen();
  }
};

onMount(async () => {
  try {
    // initialize
    await setupVideo();
    setupVideoCanvas();
    setupDrawingCanvas();
    setupEvent(videoContext, videoCanvas);

    // start processFrame
    const unixTimestamp = Math.floor(new Date().getTime() / 1000);
    processFrame(unixTimestamp);

    // help
    const deviceType: string = checkMobileDevice();
    if (deviceType === "Others") showHelp = true;
  } catch (error) {
    console.error("Error during onMount:", error);
  }
});
</script>

<!-- svelte-ignore a11y-media-has-caption -->
<video
  id="video-off"
  bind:this={video}
  autoplay
  playsinline
  aria-live="polite"
  aria-label="Live webcam feed"
></video>

<div
  id="fps"
  style="position:absolute; bottom: 10px; left: 10px; color: white; background-color: rgba(0, 0, 0, 0.7); padding: 5px;"
>
  {fps} FPS
</div>

{#if showHelp}
  <Help />
{/if}
