export interface RenderWorkerParams {
	wasm_module: WebAssembly.Module,
	image_width: bigint, 
	image_height: bigint,
	samples_per_pixel: bigint, 
	max_depth: bigint, 
	vfov: number,
	row_start: bigint, 
	row_end: bigint,
	seed: number,
}