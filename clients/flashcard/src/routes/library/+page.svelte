<script lang="ts">
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import { Maximize2 } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import Collection from './Collection.svelte';
	import Source from './Source.svelte';
	import FlashCard from './FlashCard.svelte';
</script>

<main class="flex h-full flex-col">
	<div class="flex justify-end border-b">
		<Button variant="ghost" size="icon">
			<Maximize2 class="size-4" />
		</Button>
	</div>
	<Resizable.PaneGroup direction="horizontal" autoSaveId="spbased_library_layout">
		<Collection />
		<Resizable.Handle withHandle />
		<Source />
		<Resizable.Handle withHandle />
		<FlashCard />
	</Resizable.PaneGroup>
</main>
