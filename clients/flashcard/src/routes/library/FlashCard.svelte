<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Plus } from 'lucide-svelte';
	import Container from './PaneContainer.svelte';
	import FlashCardTable from './FlashCardTable.svelte';
</script>

<Container lowerButtonText="Add flashcard" title="Flashcards" subtitle="hello there!" order={3}>
	<div class="flex flex-col gap-3">
		<!-- Actions -->
		<div class="flex justify-end">
			<Button>
				Create FlashCard <Plus />
			</Button>
		</div>
		<!-- Table -->
		<FlashCardTable />
		<!-- Table -->
		<div></div>
	</div>
</Container>
