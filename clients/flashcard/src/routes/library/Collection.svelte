<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Plus } from 'lucide-svelte';
	import Container from './PaneContainer.svelte';
	import CollectionTable from './CollectionTable.svelte';
</script>

<Container title="Collections" subtitle="hello there" order={1}>
	<div class="flex flex-col gap-3">
		<!-- Actions -->
		<div class="flex justify-end">
			<Button>
				Add collection <Plus />
			</Button>
		</div>
		<!-- Table -->
		<CollectionTable />
		<!-- Table -->
		<div></div>
	</div>
</Container>
