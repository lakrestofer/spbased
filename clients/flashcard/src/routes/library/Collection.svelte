<script lang="ts">
	import { Search } from 'lucide-svelte';
	import Container from './PaneContainer.svelte';
	import CollectionTable from './CollectionTable.svelte';
	import { Input } from '$lib/components/ui/input';
</script>

<Container lowerButtonText="Add collection" title="Collections" subtitle="hello there" order={1}>
	<div class="flex h-full flex-col gap-3" slot="upper">
		<!-- Actions -->
		<div class="flex items-center gap-3">
			<Search />
			<Input />
		</div>
		<!-- Table -->
		<CollectionTable />
	</div>
	<div slot="lower">hello there</div>
</Container>
