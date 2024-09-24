<script lang="ts">
import * as wasm from "image-detection-wasm";
import { onMount } from "svelte";
import { saveOriginalImage, saveOutputImage } from "../images";
import Help from "./Help.svelte";

// DetectionCamera
//
// 1. 2 layer from the bottom
//  - video
//  - canvas for input and output but output doesn't work
//
// 2. call detect_draw_image() as wasm function for detecting image location and drawing
//
// 3. draw output from wasm function but drawing doesn't work
//    even if drawing by canvas API to canvas, it doesn't work

// wasm response
interface WasmResponse {
  raw_data: number[];
  min_value: number;
  min_value_location: [number, number];
}

// Element
let video: HTMLVideoElement;
let canvas: HTMLCanvasElement; // reference of HTML <canvas> element
let context: CanvasRenderingContext2D; // 2d rendering context of canvas

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
  canvas.width = video.videoWidth; // 640
  canvas.height = video.videoHeight; // 480
  context = canvas.getContext("2d")!;

  // invisible or full screen
  //canvas.style.display = "none";
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
const processFrame = async (timestamp: number) => {
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
    console.log("call wasm.detect_draw_image()");
    const response = await wasm.detect_draw_image(
      new Uint8Array(imageData.data.buffer),
      canvas.width,
      canvas.height,
    );
    // check response if error
    if (response instanceof Error) throw response;
    const wasmResp = response as unknown as WasmResponse;
    if (wasmResp.min_value < 3500) {
      // detected!!
      console.log("response.min_value:", wasmResp.min_value);
    }

    // Note: For now, it's useless to draw output image
    // const rgbaBuffer = wasmResp.raw_data;
    // const outputImageData = new ImageData(
    //   new Uint8ClampedArray(rgbaBuffer),
    //   canvas.width,
    //   canvas.height,
    // );
    // // reflect output images on canvas context
    // context.putImageData(outputImageData, 0, 0);

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

    const unixTimestamp = Math.floor(new Date().getTime() / 1000);
    processFrame(unixTimestamp);

    // help
    showHelp = true;
  } catch (error) {
    console.error("Error during onMount:", error);
  }
});
</script>

<video id="video" bind:this={video} autoplay playsinline></video>

<div
  id="fps"
  style="position:absolute; bottom: 10px; left: 10px; color: white; background-color: rgba(0, 0, 0, 0.7); padding: 5px;"
>
  {fps} FPS
</div>

{#if showHelp}
  <Help />
{/if}
