<script lang="ts">
const images = [
  "./webgpu-featureflag.png",
  "./webgpu-featureflag2.png",
  "./webgpu-featureflag3.png",
];
</script>

<div class="image-container">
  {#each images as imageUrl}
    <img src={imageUrl} alt="webgpu with webkit" />
  {/each}
</div>

<style>
  .image-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    overflow-y: auto;
    height: 100vh;
  }
  img {
    max-width: 100%;
    height: auto;
    margin-bottom: 10px;
  }
</style>
