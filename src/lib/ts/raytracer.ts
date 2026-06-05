import wasmUrl from "$rust/lib_bg.wasm?url";
import RenderWorker from "$lib/ts/render_worker?worker";

const NUM_WORKERS = navigator.hardwareConcurrency || 4;

// Compile once, reuse across all workers
let compiledModule: WebAssembly.Module | null = null;

async function getCompiledModule(): Promise<WebAssembly.Module> {
    if (!compiledModule) {
        const response = await fetch(wasmUrl);
        const buffer = await response.arrayBuffer();
        compiledModule = await WebAssembly.compile(buffer);
    }
    return compiledModule;
}

export async function renderParallel(width: number, height: number, samples_per_pixel: number, max_depth: number, vfov: number): Promise<Uint8Array> {
    const wasm_module = await getCompiledModule();
    const rowsPerWorker = Math.ceil(height / NUM_WORKERS);

    const tasks = Array.from({ length: NUM_WORKERS }, (_, i) => {
        const row_start = i * rowsPerWorker;
        const row_end = Math.min(row_start + rowsPerWorker, height);

        return new Promise<{ band: Uint8Array; row_start: number }>((resolve) => {
            const worker = new RenderWorker();
            worker.onmessage = ({ data }) => {
                worker.terminate();
                resolve({ band: data, row_start });
            };
            worker.onerror = (e) => console.error(e);

            worker.postMessage({
                wasm_module,
                image_width: BigInt(width),
                image_height: BigInt(height),
                samples_per_pixel: BigInt(samples_per_pixel),
                max_depth: BigInt(max_depth),
                vfov,
                row_start: BigInt(row_start),
                row_end: BigInt(row_end),
                seed: Math.floor(Math.random() * 0xFFFFFFFF),
            });
        });
    });

    const bands = (await Promise.all(tasks)) as { band: Uint8Array; row_start: number }[];
    
	const result = new Uint8Array(width * height * 4);
    for (const { band, row_start } of bands) {
        result.set(band, row_start * width * 4);
    }
    return result;
}