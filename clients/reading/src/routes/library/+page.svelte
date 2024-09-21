<script lang="ts">
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import { Maximize2 } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import SourcePane from './SourcePane.svelte';
	import Source from './Source.svelte';
	import ViewPicker from './ViewPicker.svelte';
</script>

<main class="flex h-full flex-col">
	<div class="flex justify-between border-b px-3 py-2">
		<ViewPicker />
		<Button variant="ghost" size="icon">
			<Maximize2 class="size-4" />
		</Button>
	</div>
	<Resizable.PaneGroup direction="horizontal" autoSaveId="spbased_library_layout">
		<SourcePane />
		<Resizable.Handle withHandle />
		<Source />
	</Resizable.PaneGroup>
</main>
