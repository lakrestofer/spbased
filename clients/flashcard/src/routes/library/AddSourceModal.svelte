<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { sources, type Source } from '$lib/model/source';
	import { toast } from 'svelte-sonner';

	export let open = false;

	let file: File | undefined = undefined;
	let name: string = '';

	const onSubmit = async () => {
		if (!file) {
			toast.error('Please select a file');
			return;
		}
		if (!name) {
			toast.error('Please enter a name');
			return;
		}
		sources.addSource({ name });
		open = false;
	};
</script>

<Dialog.Root bind:open>
	<Dialog.Trigger class={buttonVariants()}>Add source</Dialog.Trigger>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Add source</Dialog.Title>
			<Dialog.Description>Add a new source to the library</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<Label for="source_file">Source</Label>
			<Input id="source_file" type="file" bind:value={file} />
			<p class="text-muted-foreground text-sm">The following formats are supported: pdf</p>
			<Label for="name">Document Name</Label>
			<Input id="name" bind:value={name} />
		</div>
		<Dialog.Footer>
			<Button on:click={onSubmit}>Add source</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
