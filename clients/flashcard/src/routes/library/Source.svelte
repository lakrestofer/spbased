<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Plus } from 'lucide-svelte';
	import Container from './PaneContainer.svelte';
	import SourceTable from './SourceTable.svelte';
</script>

<Container title="Sources" subtitle="hello there!" order={2}>
	<div class="flex flex-col gap-3">
		<!-- Actions -->
		<div class="flex justify-end">
			<Button>
				Add Source <Plus />
			</Button>
		</div>
		<!-- Table -->
		<SourceTable />
		<!-- Table -->
		<div></div>
	</div>
</Container>
