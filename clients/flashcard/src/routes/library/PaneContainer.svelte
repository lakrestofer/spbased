<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import { Plus, X } from 'lucide-svelte';
	import type { PaneAPI } from 'paneforge';

	let api: PaneAPI;
	let innerApi: PaneAPI;

	// props
	export let title: string;
	export let order: number;
	export let subtitle: string;
	export let lowerButtonText: string;

	// state
	let expanded: boolean = true;
	let innerExpanded: boolean = false;

	// derived state

	// callbacks and event handlers
</script>

<Resizable.Pane
	onExpand={() => {
		expanded = true;
	}}
	onCollapse={() => {
		expanded = false;
	}}
	collapsible
	collapsedSize={2}
	minSize={10}
	{order}
	class="flex flex-col"
	bind:pane={api}
>
	{#if !expanded}
		<Button
			variant="outline"
			size="icon"
			class="flex grow flex-col items-center justify-start rounded-none bg-accent px-2 py-3"
			on:click={api.expand}
		>
			<div class="flex flex-nowrap items-center overflow-hidden [writing-mode:vertical-lr]">
				<h1 class="mb-2 font-mono text-xl font-bold">
					{title}
				</h1>
				{#if subtitle}
					<h2 class="text-nowrap font-mono text-lg font-bold">
						{` - ${subtitle}`}
					</h2>
				{/if}
			</div>
		</Button>
	{:else}
		<div class="flex items-center justify-between">
			<div class="flex flex-nowrap items-center overflow-hidden align-middle">
				<h1 class="mr-2 font-mono text-xl font-bold">
					{title}
				</h1>
				{#if subtitle}
					<h2 class="text-nowrap font-mono">
						{` - ${subtitle}`}
					</h2>
				{/if}
			</div>
			<Button variant="ghost" size="icon" class="rounded-full" on:click={api.collapse}>
				<X class="size-4" />
			</Button>
		</div>

		<Resizable.PaneGroup direction="vertical" autoSaveId={`spbased_library_${title}_inner_layout`}>
			<Resizable.Pane defaultSize={90} minSize={10}>
				<slot name="upper" />
			</Resizable.Pane>
			<Resizable.Handle />
			<Resizable.Pane
				collapsible
				collapsedSize={4}
				defaultSize={0}
				minSize={10}
				onExpand={() => (innerExpanded = true)}
				onCollapse={() => (innerExpanded = false)}
				bind:pane={innerApi}
			>
				{#if innerExpanded}
					<div class="flex items-center justify-end">
						<Button variant="ghost" size="icon" class="rounded-full" on:click={innerApi.collapse}>
							<X class="size-4" />
						</Button>
					</div>
					<slot name="lower" />
				{:else}
					<Button class="w-full" on:click={() => innerApi.resize(50)}>
						{lowerButtonText}
						<Plus />
					</Button>
				{/if}
			</Resizable.Pane>
		</Resizable.PaneGroup>
	{/if}
</Resizable.Pane>
