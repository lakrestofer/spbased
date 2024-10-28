<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import TagSelector from './TagSelector.svelte';

	let files: FileList | undefined = $state();
	let file = $derived.by(() => {
		if (!files) return;
		return files[0];
	});
	let file_name = $derived(!file ? '' : file.name);
	let title = $state('');
	$effect(() => {
		title = file_name;
	});
	let author = $state('');
	let selectedTags = $state<string[]>([]);
</script>

<Dialog.Root>
	<Dialog.Trigger class={buttonVariants({ variant: 'outline' })}>Add Source</Dialog.Trigger>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Add Source</Dialog.Title>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="file" class="text-right">File</Label>
				<!-- HACK: can only bind files on plain input-->
				<input
					id="file"
					type="file"
					bind:files
					class="col-span-3 flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
				/>
			</div>
			<!-- title -->
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="title" class="text-right">Title</Label>
				<Input id="title" class="col-span-3" bind:value={title} />
			</div>
			<!-- author -->
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="author" class="text-right">Author</Label>
				<Input id="author" class="col-span-3" bind:value={author} />
			</div>
			<!-- tags -->
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="tags" class="text-right">Tags</Label>
				<TagSelector class="col-span-3" bind:selectedTags />
			</div>
			<Dialog.Footer>
				<Button type="submit">Save changes</Button>
			</Dialog.Footer>
		</div></Dialog.Content
	>
</Dialog.Root>
