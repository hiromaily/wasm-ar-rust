<script lang="ts">
import DetectionCamera from "./lib/components/DetectionCamera2.svelte";
import Video from "./lib/components/EffectVideo.svelte";
import WasmCanvas from "./lib/components/WasmCanvas.svelte";

const activeComponent = "detection"; // video, detection, canvas
</script>

<main>
  {#if activeComponent == "video"}
    <Video />
  {:else if activeComponent == "detection"}
    <DetectionCamera />
  {:else if activeComponent == "canvas"}
    <WasmCanvas />
  {/if}
</main>
