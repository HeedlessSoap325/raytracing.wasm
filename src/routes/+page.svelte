<script lang="ts">
	import {renderParallel} from "$lib/ts/raytracer";
	import Icon from "@iconify/svelte";

	let canvas: HTMLCanvasElement | undefined = $state(undefined);
	
	let ms 						  = $state(0);
	let rendering: boolean 		  = $state(false);
	let width: number 			  = $state(640);
	let height: number 			  = $state(360);
	let samples_per_pixel: number = $state(50);
	let max_depth: number 		  = $state(50);
	let vfov: number 			  = $state(90);

	async function doRender() {
		if (!canvas) return;
		const ctx = canvas.getContext("2d");
		if (!ctx) return;

		rendering = true;

		const t0 = performance.now();
		const pixelData = await renderParallel(width, height, samples_per_pixel, max_depth, vfov);
		const t1 = performance.now();

		const imageData = new ImageData(	
			new Uint8ClampedArray<ArrayBuffer>(pixelData.buffer as ArrayBuffer, pixelData.byteOffset, pixelData.byteLength),
			width,
			height
		);

		ctx.putImageData(imageData, 0, 0);
		ms = t1 - t0;
		rendering = false;
	}

	$effect(() => {
		if (!canvas) return;

		canvas.width  = width;
		canvas.height = height;
	})
</script>

<div id="content">
	<canvas id="canvas" bind:this={canvas}></canvas>

	<div id="controlls">
		<div class="controll">
			<label for="width">Width</label>
			<input id="width" bind:value={width} type="number" step="1"/>
		</div>

		<div class="controll">
			<label for="height">Height</label>
			<input id="height" bind:value={height} type="number" step="1"/>
		</div>

		<div class="controll">
			<label for="spp">samples Per Pixel</label>
			<input id="spp" bind:value={samples_per_pixel} type="number" step="1"/>
		</div>

		<div class="controll">
			<label for="max_depth">Max depth</label>
			<input id="max_depth" bind:value={max_depth} type="number" step="1"/>
		</div>

		<div class="controll">
			<label for="vfov">Vertical FOV</label>
			<input id="vfov" bind:value={vfov} type="number" step="0.01"/>
		</div>

		<button onclick={doRender} disabled={rendering} class="btn">
			{#if rendering}
				<Icon icon="tabler:loader-2" class="spin" aria-hidden="true" />
				Rendering
			{:else}
				Render!
			{/if}
		</button>
	</div>

	{#if ms > 0}
		<span>The previous render took {ms}Ms (1000Ms = 1s)</span>
	{/if}
</div>

<style>
	#canvas {
		border: 1px solid black;
	}

	#controlls {
		margin-top: 2rem;
		display: grid;
		grid-template-columns: 1fr 1fr;
		width: fit-content;
		gap: 2rem;
	}

	.controll {
		display: flex;
		flex-direction: column;
	}

	.btn {
		display: flex;
		align-items: center;
		gap: 5px;
	}

	:global {
		.spin { 
			animation: spin 0.8s linear infinite; 
		}

		@keyframes spin { 
			to { 
				transform: rotate(360deg); 
			} 
		}
	}
</style>