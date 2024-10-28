<script module lang="ts">
	interface Props {
		class?: string | undefined | null;
		selectedTags: string[];
	}
</script>

<script lang="ts">
	import Check from 'lucide-svelte/icons/check';
	import ChevronsUpDown from 'lucide-svelte/icons/chevrons-up-down';
	import * as Command from '$lib/components/ui/command/index.js';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { cn } from '$lib/utils.js';
	import { tags } from '$lib/model/tag.svelte.js';

	let { class: className, selectedTags = $bindable([]) }: Props = $props();
	let selectedTagsString = $derived.by(() => {
		return selectedTags.slice(0, 3).join(', ') + (selectedTags.length > 3 ? '...' : '');
	});

	let open = $state(false);
	let commandValue = $state('');

	$inspect(commandValue);
	let inputText = $state('');
	$inspect(inputText);
	function onkeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && commandValue == '') {
			if (inputText) {
				selectedTags = [...selectedTags, inputText];
				tags.addTag(inputText);
				inputText = '';
			}
		}
	}
</script>

<Popover.Root bind:open>
	<Popover.Trigger>
		{#snippet child({ props })}
			<Button
				variant="outline"
				class={cn('justify-between', className)}
				{...props}
				role="combobox"
				aria-expanded={open}
			>
				{selectedTags.length ? selectedTagsString : 'Select tags...'}
				<ChevronsUpDown class="ml-2 size-4 shrink-0 opacity-50" />
			</Button>
		{/snippet}
	</Popover.Trigger>
	<Popover.Content class="p-0">
		<Command.Root loop={true} bind:value={commandValue}>
			<Command.Input placeholder="Search tags..." bind:value={inputText} {onkeydown} />
			<Command.List>
				<Command.Empty>No tag with that name found. Press enter to create it!</Command.Empty>
				<Command.Group>
					{#each tags.value as tag}
						<Command.Item
							value={tag.name}
							onSelect={() => {
								if (selectedTags.includes(tag.name)) {
									selectedTags = selectedTags.filter((t) => t !== tag.name);
								} else {
									selectedTags = [...selectedTags, tag.name];
								}
							}}
						>
							<Check
								class={cn('mr-2 size-4', !selectedTags.includes(tag.name) && 'text-transparent')}
							/>
							{tag.name}
						</Command.Item>
					{/each}
				</Command.Group>
			</Command.List>
		</Command.Root>
	</Popover.Content>
</Popover.Root>
