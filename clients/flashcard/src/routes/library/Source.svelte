<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import Container from './PaneContainer.svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import SourceViewDropdown, { type Layout } from './SourceViewDropdown.svelte';
	import type { Source } from '$lib/model/source';
	import AddSourceModal from './AddSourceModal.svelte';
	import SourceGrid from './SourceGrid.svelte';
	import SourceList from './SourceList.svelte';
	import EditSourceModal from './EditSourceModal.svelte';

	let layout: Layout; // grid or list view for sources

	// add modal
	let addModalOpen = false;
	// edit modal
	let sourceToEdit: Source | undefined = undefined;
	let editModalOpen = false;
</script>

<AddSourceModal bind:open={addModalOpen} />
<EditSourceModal bind:open={editModalOpen} bind:source={sourceToEdit} />
<Container title="Sources" order={1}>
	<!-- filter and add source button -->
	<div class="flex justify-end gap-3">
		<SourceViewDropdown bind:layout />
		<Button on:click={() => (addModalOpen = true)}>Add source</Button>
	</div>
	<div>
		{#if layout === 'grid'}
			<SourceGrid
				onClickEditSource={(source) => {
					sourceToEdit = source;
					editModalOpen = true;
				}}
			/>
		{:else}
			<SourceList />
		{/if}
	</div>
</Container>
