import init, { render_band } from "$rust/lib";
import type { RenderWorkerParams } from "$lib/ts/types";

self.onmessage = async ({ data }: { data: RenderWorkerParams }) => {
	const t0 = performance.now();
	console.info(`[renderParallel]: Worker started (${data.seed})`);
    await init({ module_or_path : data.wasm_module });
	console.info(`[renderParallel]: WASM initialized (${data.seed})`);

    const pixels = render_band(
        data.image_width, data.image_height,
        data.samples_per_pixel, data.max_depth, data.vfov,
        data.row_start, data.row_end,
        data.seed
    );
	                    	
	/* 
		Typescript will tell you that this is wrong, but it's actually right	
		The Documentation seems to be off or something, but this is how you do it. 
	*/
    self.postMessage(pixels, [pixels.buffer]);
	const t1 = performance.now();
	console.info(`[renderParallel]: Worker finished in ${t1 - t0}ms (${data.seed})`);
};