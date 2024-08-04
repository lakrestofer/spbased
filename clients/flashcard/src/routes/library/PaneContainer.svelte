<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import { X } from 'lucide-svelte';
	import type { PaneAPI } from 'paneforge';

	let api: PaneAPI;

	// props
	export let title: string;
	export let order: number;
	export let subtitle: string | undefined = undefined;

	// state
	let collapsed: boolean;

	// derived state

	// callbacks and event handlers
	const expand = () => api.resize(33);
	const collapse = () => api.collapse();
	const onCollapse = () => (collapsed = true);
	const onExpand = () => (collapsed = false);
</script>

<Resizable.Pane
	{onExpand}
	{onCollapse}
	collapsible
	collapsedSize={2}
	minSize={10}
	{order}
	class="flex flex-col"
	bind:pane={api}
>
	{#if collapsed}
		<Button
			variant="outline"
			size="icon"
			class="flex grow flex-col items-center justify-start rounded-none bg-accent px-2 py-3"
			on:click={expand}
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
		<div class="border-x-1 flex items-center justify-between border-y-2 px-3">
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
			<Button variant="ghost" size="icon" class="rounded-full" on:click={collapse}>
				<X class="size-4" />
			</Button>
		</div>
		<slot />
	{/if}
</Resizable.Pane>
