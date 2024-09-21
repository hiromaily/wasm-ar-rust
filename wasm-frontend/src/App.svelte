<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import * as wasm from "opencv-wasm";

  let videoElement: HTMLVideoElement;
  let canvasElement: HTMLCanvasElement;
  let isFullScreen: boolean = false;
  let imageDataUrl = "";
  let animationFrameId: number;

  async function startCamera() {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ video: true });
      videoElement.srcObject = stream;

      // カメラ映像がロードされたら定期的にキャプチャする
      videoElement.onloadedmetadata = () => {
        videoElement.play();
        captureImage();
      };
    } catch (err) {
      console.error("カメラにアクセスできませんでした:", err);
      alert("カメラにアクセスできませんでした。");
    }
  }

  async function captureImage() {
    const context = canvasElement.getContext("2d");
    if (!context) return;

    canvasElement.width = videoElement.videoWidth;
    canvasElement.height = videoElement.videoHeight;
    context.drawImage(
      videoElement,
      0,
      0,
      canvasElement.width,
      canvasElement.height
    );
    imageDataUrl = canvasElement.toDataURL("image/png");

    // Rustに送るデータを準備（例: Uint8Arrayに変換して送信）
    const imageData = context.getImageData(
      0,
      0,
      canvasElement.width,
      canvasElement.height
    );
    const rgba = new Uint8Array(imageData.data.buffer);
    //const rgba = imageData.data;
    console.log(rgba);

    // Rust/WebAssemblyへの送信処理を呼び出す
    await sendToRust(rgba);

    // 次のフレームで再度キャプチャを行う
    animationFrameId = requestAnimationFrame(captureImage);
  }

  async function sendToRust(rgba: Uint8Array) {
    try {
      // call wasm
      const result = wasm.process_image(rgba);
      console.log(result);
    } catch (e) {
      console.error(e);
    }
  }

  // async function sendToRust(rgba: Uint8ClampedArray) {
  //   try {
  //     // call wasm
  //     const result = wasm.process_image(rgba);
  //     console.log(result);
  //   } catch (e) {
  //     console.error(e);
  //   }
  // }

  function toggleFullScreen() {
    isFullScreen = !isFullScreen;

    if (isFullScreen) {
      if (videoElement.requestFullscreen) {
        videoElement.requestFullscreen();
      }
    } else {
      if (document.exitFullscreen) {
        document.exitFullscreen();
      }
    }
  }

  onMount(() => {
    startCamera();
  });

  onDestroy(() => {
    // クリーンアップ
    if (animationFrameId) {
      cancelAnimationFrame(animationFrameId);
    }
  });
</script>

<main>
  <h1>Camera Access and Image Capture Demo</h1>
  <video bind:this={videoElement} autoplay playsinline></video>
  <button on:click={toggleFullScreen}
    >{isFullScreen ? "Exit Full Screen" : "Full Screen"}</button
  >
  <canvas bind:this={canvasElement} style="display: none;"></canvas>
  {#if imageDataUrl}
    <img src={imageDataUrl} alt="Captured Image" />
  {/if}
</main>

<style>
  html,
  body,
  main {
    height: 100%;
    margin: 0;
  }
  video {
    width: 100%;
    height: 100%;
    object-fit: cover; /* アスペクト比を維持しながらビデオの内容をすべて表示 */
  }
  button {
    position: absolute; /* フルスクリーンモード時にも表示を維持 */
    top: 10px;
    right: 10px;
    z-index: 1;
    padding: 10px;
  }
</style>
