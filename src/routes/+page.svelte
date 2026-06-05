<script lang="ts">
	import init, { render } from "$rust/lib";
    import { onMount } from "svelte";

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
		const pixelData = render(BigInt(width), BigInt(height), BigInt(samples_per_pixel), BigInt(max_depth), vfov);
		const t1 = performance.now();

		const imageData = new ImageData(	
			new Uint8ClampedArray<ArrayBuffer>(pixelData.buffer as ArrayBuffer, pixelData.byteOffset, pixelData.byteLength),
			width,
			height
		);

		ctx!.putImageData(imageData, 0, 0);
		ms = t1 - t0;
	}

	onMount(async () => {
		await init();
	
	});
</script>

<canvas bind:this={canvas}></canvas>

<button onclick={doRender}>Render! prev: {ms}</button>