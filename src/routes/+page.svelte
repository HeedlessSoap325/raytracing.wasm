<script lang="ts">
	import init, { render } from "$rust/lib";
    import { onMount } from "svelte";

	let canvas: HTMLCanvasElement | undefined = $state(undefined);

	async function doRender() {
		const ctx = canvas!.getContext("2d");
		const width  = 640;
		const height = 360;

		canvas!.width  = width;
		canvas!.height = height;

		const pixelData = render(BigInt(width), BigInt(height));
		console.log(pixelData)

		const imageData = new ImageData(	
			new Uint8ClampedArray<ArrayBuffer>(pixelData.buffer as ArrayBuffer, pixelData.byteOffset, pixelData.byteLength),
			width,
			height
		);

		ctx!.putImageData(imageData, 0, 0);
	}

	onMount(async () => {
		await init();
	
	});
</script>

<canvas bind:this={canvas}></canvas>

<button onclick={doRender}>Render!</button>