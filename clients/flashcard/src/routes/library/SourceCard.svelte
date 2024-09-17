<script lang="ts">
	import { type Source } from '$lib/model/source';
	import * as Card from '$lib/components/ui/card';
	import { Skeleton } from '$lib/components/ui/skeleton/index.js';
	import { onMount } from 'svelte';
	import { faker } from '@faker-js/faker';
	import { Button } from '$lib/components/ui/button';
	import { Book } from 'lucide-svelte';

	export let source: Source;
</script>

<Card.Root class="w-[20rem]">
	<Card.Header>
		{#if source.url}
			<img src={source.url} alt={source.name} class="h-[10rem] w-full object-cover" />
		{:else}
			<Skeleton class="h-[10rem]" />
		{/if}
	</Card.Header>
	<Card.Content>
		<Card.Title>{source.name}</Card.Title>
	</Card.Content>
	<Card.Footer class="flex justify-end">
		<Button variant="ghost" size="icon" href={`/source/${source.id}`}><Book /></Button>
	</Card.Footer>
</Card.Root>
