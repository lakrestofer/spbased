<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import { Plus, X } from 'lucide-svelte';
	import type { PaneAPI } from 'paneforge';

	let api: PaneAPI;

	// props
	export let title: string;
	export let order: number;
	export let subtitle: string | undefined = undefined;

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
			<div class="flex flex-nowrap items-center overflow-hidden px-3 py-2 align-middle">
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
		<div class="p-3">
			<slot />
		</div>
	{/if}
</Resizable.Pane>
