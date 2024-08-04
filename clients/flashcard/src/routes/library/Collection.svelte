<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Plus, Search } from 'lucide-svelte';
	import Container from './PaneContainer.svelte';
	import CollectionTable from './CollectionTable.svelte';
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import { Input } from '$lib/components/ui/input';
	import type { PaneAPI } from 'paneforge';

	let pane: PaneAPI;

	let addCollOpen = false;
	const onExpand = () => (addCollOpen = true);
	const onCollapse = () => (addCollOpen = false);
	const expand = () => pane.resize(50);
</script>

<Container title="Collections" subtitle="hello there" order={1}>
	<Resizable.PaneGroup direction="vertical" autoSaveId="spbased_library_collection_layout">
		<Resizable.Pane defaultSize={90} minSize={10}>
			<div class="flex h-full flex-col gap-3 p-3">
				<!-- Actions -->
				<div class="flex items-center gap-3">
					<Search />
					<Input />
				</div>
				<!-- Table -->
				<CollectionTable />
			</div>
		</Resizable.Pane>
		<Resizable.Handle />
		<Resizable.Pane
			collapsible
			collapsedSize={4}
			defaultSize={0}
			minSize={10}
			{onExpand}
			{onCollapse}
			bind:pane
		>
			{#if addCollOpen}
				hello there
			{:else}
				<Button class="w-full" on:click={expand}>
					Add collection <Plus />
				</Button>
			{/if}
		</Resizable.Pane>
	</Resizable.PaneGroup>
</Container>
