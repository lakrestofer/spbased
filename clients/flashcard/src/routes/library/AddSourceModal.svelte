<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import type { Source } from '$lib/model/source';

	export let open = false;
	export let sourceToEdit: Source | undefined = undefined;

	let file: File | undefined;
	let name: string = '';

	$: edit = sourceToEdit !== undefined;
	$: title = edit ? 'Edit source' : 'Add source';
	$: description = edit ? 'Edit the source details below' : 'Add a new source to the library';
	$: buttonText = edit ? 'Save changes' : 'Add source';

	const onOpenChange = (opening: boolean) => {
		if (opening) return;

		sourceToEdit = undefined;
	};
</script>

<Dialog.Root bind:open {onOpenChange}>
	<Dialog.Trigger />
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>{title}</Dialog.Title>
			<Dialog.Description>{description}</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<Label for="source_file">Source</Label>
			<Input id="source_file" type="file" bind:value={file} />
			<p class="text-muted-foreground text-sm">The following formats are supported: pdf</p>
			<Label for="name">Document Name</Label>
			<Input id="name" bind:value={name} />
		</div>
		<Dialog.Footer>
			<Button>{buttonText}</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
