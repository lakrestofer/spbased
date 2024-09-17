<script lang="ts">
	import { sources, type Source } from '$lib/model/source';
	import SourceCard from './SourceCard.svelte';
</script>

<div class="flex flex-wrap justify-center gap-3 p-3">
	{#if $sources.length}
		{#each $sources as source}
			<SourceCard {source} />
		{/each}
	{:else}
		<p>No sources found. Add a new source by clicking "Add source" above</p>
	{/if}
</div>
