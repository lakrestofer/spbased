<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import Container from './PaneContainer.svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import SourceViewDropdown, { type Layout } from './SourceViewDropdown.svelte';
	import type { Source } from '$lib/model/source';
	import AddSourceModal from './AddSourceModal.svelte';
	import SourceGrid from './SourceGrid.svelte';
	import SourceList from './SourceList.svelte';

	let layout: Layout;

	let open = false;
</script>

<AddSourceModal bind:open />
<Container title="Sources" order={1}>
	<!-- filter and add source button -->
	<div class="flex justify-end gap-3">
		<SourceViewDropdown bind:layout />
		<Button on:click={() => (open = true)}>Add source</Button>
	</div>
	<div>
		{#if layout === 'grid'}
			<SourceGrid />
		{:else}
			<SourceList />
		{/if}
	</div>
</Container>
