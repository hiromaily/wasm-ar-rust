<script lang="ts">
import * as wasm from "image-detection-wasm";
import { onMount } from "svelte";
import Help from "./Help.svelte";

interface WasmResponse {
  min_value: number;
  min_value_location: [number, number];
}

// Element
let video: HTMLVideoElement;
let canvas: HTMLCanvasElement;
let context: CanvasRenderingContext2D;

// Flag
let initialized = false;
let showHelp = false;
const isDetected = false;

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

const setupEvent = () => {
  // button event
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      toggleFullScreen();
    } else if (event.key === "h" || event.key === "H") {
      showHelp = !showHelp;
    }
  });
};

// process each frame
const processFrame = async (timestamp: number) => {
  try {
    if (!initialized) return;

    const w = video.videoWidth;
    const h = video.videoHeight;

    // calculate and update FPS
    if (lastTimestamp) {
      const delta = (timestamp - lastTimestamp) / 1000;
      fps = Math.round(1 / delta);
    }
    lastTimestamp = timestamp;

    // video image
    context.drawImage(video, 0, 0, w, h);
    const imageData = context.getImageData(0, 0, w, h);
    // call wasm function
    console.log("call wasm.detect_image()");
    const response = await wasm.detect_image(
      new Uint8Array(imageData.data.buffer),
      w,
      h,
    );
    // check response if error
    if (response instanceof Error) throw response;
    const wasmResp = response as unknown as WasmResponse;

    console.log(wasmResp);
    if (wasmResp.min_value < 3500) {
      console.log("response.min_value:", wasmResp.min_value);
    }

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

onMount(async () => {
  try {
    // initialize
    await setupVideo();
    setupEvent();

    // canvas
    canvas = document.createElement("canvas");
    context = canvas.getContext("2d")!;

    const unixTimestamp = Math.floor(new Date().getTime() / 1000);
    processFrame(unixTimestamp);

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
