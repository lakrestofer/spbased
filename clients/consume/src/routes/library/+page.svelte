<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Edit, Plus } from 'lucide-svelte';
	import ViewSelector from './ViewSelector.svelte';
	import AddSourceButton from './AddSourceButton.svelte';

	enum RightPanelContent {
		Table,
		AddSource,
		EditSource
	}
</script>

<main class="h-full">
	<div class="flex items-center justify-between gap-1 border-b p-2">
		<div class="flex gap-1">
			<ViewSelector />
			<Button size="icon" variant="outline" class="rounded-full">
				<Plus />
			</Button>
			<Button size="icon" variant="outline" class="rounded-full">
				<Edit />
			</Button>
		</div>
		<div>
			<AddSourceButton />
		</div>
	</div>

	<div></div>
</main>
