<script>
	import { Button } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import { Sun, Moon } from 'lucide-svelte';
	import { toggleMode, mode } from 'mode-watcher';
</script>

<Button
	on:click={toggleMode}
	size="icon"
	variant="ghost"
	class={cn('p-2', $mode === 'dark' ? 'text-blue-400' : 'text-yellow-400')}
>
	{#if $mode === 'dark'}
		<Moon />
	{:else}
		<Sun />
	{/if}
</Button>
