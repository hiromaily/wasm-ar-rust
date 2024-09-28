<script lang="ts">
import DetectionCamera from "./lib/components/DetectionCamera3.svelte";
// import Video from "./lib/components/EffectVideo.svelte";
// import WasmCanvas from "./lib/components/WasmCanvas.svelte";

const activeComponent = "detection"; // video, detection, canvas
</script>

<main>
  <DetectionCamera />
  <!-- {#if activeComponent == "video"}
    <Video />
  {:else if activeComponent == "detection"}
    <DetectionCamera />
  {:else if activeComponent == "canvas"}
    <WasmCanvas />
  {/if} -->
</main>
