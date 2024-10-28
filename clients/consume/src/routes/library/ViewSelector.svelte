<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Popover from '$lib/components/ui/popover';
	import * as Command from '$lib/components/ui/command';
	import { Check, ChevronsUpDown } from 'lucide-svelte';
	import { tick } from 'svelte';
	import { cn } from '$lib/utils';

	const views = [
		{
			value: 'default',
			label: 'Default'
		}
	];

	let open = $state(false);
	let value = $state('default');
	let triggerRef = $state<HTMLButtonElement>(null!);
	const selectedValue = $derived(views.find((f) => f.value === value)?.label);

	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef.focus();
		});
	}
</script>

<Popover.Root bind:open>
	<Popover.Trigger bind:ref={triggerRef}>
		{#snippet child({ props })}
			<Button
				variant="outline"
				class="w-[200px] justify-between"
				{...props}
				role="combobox"
				aria-expanded={open}
			>
				{selectedValue || 'Select a view...'}
				<ChevronsUpDown class="ml-2 size-4 shrink-0 opacity-50" />
			</Button>
		{/snippet}
	</Popover.Trigger>
	<Popover.Content>
		<Command.Root>
			<Command.Input placeholder="Search views..." />
			<Command.List>
				<Command.Empty>No Views found</Command.Empty>
				<Command.Group>
					{#each views as view}
						<Command.Item
							value={view.value}
							onSelect={() => {
								value = view.value;
								closeAndFocusTrigger();
							}}
						>
							<Check class={cn('mr-2 size-4', value !== view.value && 'text-transparent')} />
							{view.label}
						</Command.Item>
					{/each}
				</Command.Group>
			</Command.List>
		</Command.Root>
	</Popover.Content>
</Popover.Root>
