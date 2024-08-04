<script lang="ts">
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import { Maximize2 } from 'lucide-svelte';
	import CollectionTable from './CollectionTable.svelte';
	import FlashCardTable from './FlashCardTable.svelte';
	import SourceTable from './SourceTable.svelte';
	import { Button } from '$lib/components/ui/button';
</script>

<main class="flex h-full flex-col">
	<div class="flex justify-end border bg-primary-foreground">
		<Button variant="ghost" size="icon">
			<Maximize2 class="size-4" />
		</Button>
	</div>
	<Resizable.PaneGroup direction="horizontal" autoSaveId="spbased_library_layout">
		<CollectionTable />
		<Resizable.Handle withHandle />
		<SourceTable />
		<Resizable.Handle withHandle />
		<FlashCardTable />
	</Resizable.PaneGroup>
</main>
