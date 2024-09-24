<script lang="ts">
import * as wasm from "canvas-wasm";
import { onMount } from "svelte";

// CanvasWasm
//

onMount(async () => {
  await wasm.start();
});
</script>
