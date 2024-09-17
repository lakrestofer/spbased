<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { sources, type Source } from '$lib/model/source';
	import { toast } from 'svelte-sonner';

	export let open = false;
	export let source: Source | undefined = undefined;

	let file: File | undefined;
	let name: string = '';

	$: {
		if (open && source) {
			name = source.name;
			console.log('editing source!');
		}
	}
	const onOpenChange = (opening: boolean) => {
		if (!opening) source = undefined;
	};

	const onSubmit = async () => {
		if (!source) {
			toast.error('No source to edit');
			return;
		}
		if (!file) {
			toast.error('Please select a file');
			return;
		}
		if (!name) {
			toast.error('Please enter a name');
			return;
		}

		source.name = name;

		sources.editSource(source);

		open = false;
	};
</script>

<Dialog.Root bind:open {onOpenChange}>
	<Dialog.Trigger />
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Edit source</Dialog.Title>
			<Dialog.Description>Edit the source details below</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<Label for="source_file">Source</Label>
			<Input id="source_file" type="file" bind:value={file} />
			<p class="text-muted-foreground text-sm">The following formats are supported: pdf</p>
			<Label for="name">Document Name</Label>
			<Input id="name" bind:value={name} />
		</div>
		<Dialog.Footer>
			<Button on:click={onSubmit}>Save changes</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
