<script lang="ts">
	import {renderParallel} from "$lib/ts/raytracer";

	let canvas: HTMLCanvasElement | undefined = $state(undefined);
	let ms = $state(0);

	async function doRender() {
		const ctx = canvas!.getContext("2d");
		const width  = 640;
		const height = 360;
		const samples_per_pixel = 50;
		const max_depth = 50;
		const vfov = 90;

		canvas!.width  = width;
		canvas!.height = height;

		const t0 = performance.now();
		const pixelData = await renderParallel(width, height, samples_per_pixel, max_depth, vfov);
		const t1 = performance.now();

		const imageData = new ImageData(	
			new Uint8ClampedArray<ArrayBuffer>(pixelData.buffer as ArrayBuffer, pixelData.byteOffset, pixelData.byteLength),
			width,
			height
		);

		ctx!.putImageData(imageData, 0, 0);
		ms = t1 - t0;
	}
</script>

<canvas bind:this={canvas}></canvas>

<button onclick={doRender}>Render! prev: {ms}</button>