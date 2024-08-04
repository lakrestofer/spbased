<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { Icon } from 'lucide-svelte';
	import type { ComponentType } from 'svelte';

	export var icon: ComponentType<Icon> | undefined = undefined;
	export var page: string | undefined = undefined;
</script>

<Button variant="ghost" size="icon" class="p-2" href={page ?? '#'}>
	<svelte:component this={icon} />
</Button>
