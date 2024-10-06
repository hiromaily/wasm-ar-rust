<script lang="ts">
import DetectionCamera from "./lib/components/DetectionCamera3.svelte";
import NoWebGPU from "./lib/components/NoWebGPU.svelte";
// import Video from "./lib/components/EffectVideo.svelte";
// import WasmCanvas from "./lib/components/WasmCanvas.svelte";

if (!navigator.gpu) {
  alert(
    "If you are using iPhone, WebGPU in WebKit Feature Flags must be enabled to use this application by Safari.",
  );
}
</script>

<main>
  <!-- <DetectionCamera /> -->

  {#if navigator.gpu}
    <DetectionCamera />
  {:else}
    <NoWebGPU />
  {/if}

  <!-- {#if activeComponent == "video"}
    <Video />
  {:else if activeComponent == "detection"}
    <DetectionCamera />
  {:else if activeComponent == "canvas"}
    <WasmCanvas />
  {/if} -->
</main>
