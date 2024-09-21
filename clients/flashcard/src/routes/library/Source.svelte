<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import Container from './PaneContainer.svelte';
	import type { Source } from '$lib/model/source';
	import AddSourceModal from './AddSourceModal.svelte';
	import SourceList from './SourceList.svelte';

	// add modal
	let addModalOpen = false;
</script>

<Container title="Sources" order={1}>
	<!-- filter and add source button -->
	<div class="flex justify-end gap-3">
		<AddSourceModal bind:open={addModalOpen} />
	</div>
	<div class="py-3">
		<SourceList />
	</div>
</Container>
