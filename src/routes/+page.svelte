<script lang="ts">
	import init, { hello } from "$rust/lib";
    import { onMount } from "svelte";

	onMount(async () => {
		await init();
		console.log(hello());
	});
</script>