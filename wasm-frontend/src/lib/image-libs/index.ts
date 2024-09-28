// Save image from canvas
const saveImage = (tempCanvas: HTMLCanvasElement, filename: string) => {
  const dataURL = tempCanvas.toDataURL("image/png");
  const link = document.createElement("a");
  link.href = dataURL;
  link.download = filename;
  link.click();
};

// save original camera image
export const saveOriginalImage = (
  context?: CanvasRenderingContext2D,
  video?: HTMLVideoElement,
) => {
  if (!context || !video) return;

  // Create a temporary canvas to capture the original video frame
  const tempCanvas = document.createElement("canvas");
  tempCanvas.width = video.videoWidth;
  tempCanvas.height = video.videoHeight;
  const tempContext = tempCanvas.getContext("2d");
  tempContext?.drawImage(video, 0, 0, video.videoWidth, video.videoHeight);

  saveImage(tempCanvas, "original_image.png");
};

// save output image with wasm function
export const saveOutputImage = (
  context?: CanvasRenderingContext2D,
  canvas?: HTMLCanvasElement,
) => {
  if (!context || !canvas) return;

  // Create a temporary canvas to draw the current frame
  const tempCanvas = document.createElement("canvas");
  tempCanvas.width = canvas.width;
  tempCanvas.height = canvas.height;
  const tempContext = tempCanvas.getContext("2d");

  // Draw the current processed frame onto the temporary canvas
  tempContext?.drawImage(canvas, 0, 0);

  saveImage(tempCanvas, "edge_detected_image.png");
};

// save output image with wasm function
// export const saveOutputImageWithWasm = (
//   context?: CanvasRenderingContext2D,
//   canvas?: HTMLCanvasElement,
// ) => {
//   if (!context || !canvas) return;

//   const imageData = context.getImageData(0, 0, canvas.width, canvas.height);
//   const rgbaBuffer = new Uint8Array(imageData.data.buffer);
//   // call wasm function
//   const base64String = wasm.save_image(rgbaBuffer, canvas.width, canvas.height);

//   const link = document.createElement("a");
//   link.href = `data:image/png;base64,${base64String}`;
//   link.download = "edge_detected_image.png";
//   link.click();
// };
