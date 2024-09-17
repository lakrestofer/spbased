<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { sources, type Source } from '$lib/model/source';
	import { toast } from 'svelte-sonner';
	import type { PageData } from './$types';

	let source: Source | undefined = undefined;

	let file: File | undefined;
	let name: string = '';

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

		sources.editSource(source);
	};
</script>

what
<div class="grid gap-4 py-4">
	<Label for="source_file">Source</Label>
	<Input id="source_file" type="file" bind:value={file} />
	<p class="text-muted-foreground text-sm">The following formats are supported: pdf</p>
	<Label for="name">Document Name</Label>
	<Input id="name" bind:value={name} />
</div>
