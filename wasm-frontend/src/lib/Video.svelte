<script lang="ts">
import * as wasm from "image-wasm";
import { onMount } from "svelte";

let video: HTMLVideoElement;
let canvas: HTMLCanvasElement;
let context: CanvasRenderingContext2D;
let initialized = false;

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

function processFrame() {
	try {
		if (!initialized || !context) return;

		context.drawImage(video, 0, 0, canvas.width, canvas.height);
		const imageData = context.getImageData(0, 0, canvas.width, canvas.height);
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

		context.putImageData(outputImageData, 0, 0);

		requestAnimationFrame(processFrame);
	} catch (error) {
		console.error("Error processing frame:", error);
	}
}

function toggleFullScreen() {
	if (!document.fullscreenElement) {
		document.documentElement.requestFullscreen();
	} else if (document.exitFullscreen) {
		document.exitFullscreen();
	}
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

		document.addEventListener("keydown", (event) => {
			if (event.key === "Escape") {
				toggleFullScreen();
			}
		});

		processFrame();
	} catch (error) {
		console.error("Error during onMount:", error);
	}
});
</script>

<video bind:this={video} autoplay playsinline></video>
