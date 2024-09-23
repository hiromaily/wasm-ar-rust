<script lang="ts">
import DetectionCamera from "./lib/components/DetectionCamera2.svelte";
import Video from "./lib/components/Video.svelte";
const isDetect = true;
</script>

<main>
  {#if isDetect}
    <DetectionCamera />
  {:else}
    <Video />
  {/if}
</main>
