<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import Container from './PaneContainer.svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import SourceViewDropdown, { type Layout } from './SourceViewDropdown.svelte';

	let layout: Layout;
</script>

<Container title="Sources" order={1}>
	<div class="flex justify-end gap-3">
		<SourceViewDropdown bind:layout />
		<Button>Add Source</Button>
	</div>
	<div></div>
</Container>
