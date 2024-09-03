<script lang="ts" context="module">
	export type Layout = 'grid' | 'table';
</script>

<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { Check, LayoutGrid, Rows3 } from 'lucide-svelte';
	import * as Select from '$lib/components/ui/select/index.js';

	const items = [
		{ value: 'grid', label: 'grid' },
		{ value: 'table', label: 'table' }
	];
	let selected = items[0];
	export let layout;
	$: layout = selected.value;
</script>

<div>
	<Select.Root
		{items}
		{selected}
		onSelectedChange={(e) => (selected = { value: e?.value ?? 'grid', label: e?.label ?? 'grid' })}
	>
		<Select.Trigger>
			{#if layout == 'grid'}
				<LayoutGrid />
			{:else}
				<Rows3 />
			{/if}
		</Select.Trigger>
		<Select.Content>
			<Select.Item value={'grid'} label={'grid'}>
				<LayoutGrid />
			</Select.Item>
			<Select.Item value={'table'} label={'table'}>
				<Rows3 />
			</Select.Item>
		</Select.Content>
		<Select.Input name="favoriteFruit" />
	</Select.Root>
</div>
