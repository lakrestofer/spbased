<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { Icon } from 'lucide-svelte';
	import type { ComponentType } from 'svelte';
	import * as Tooltip from '$lib/components/ui/tooltip';

	export var icon: ComponentType<Icon> | undefined = undefined;
	export var page: string | undefined = undefined;
	export var tooltip: string | undefined = undefined;
</script>

<Tooltip.Root>
	<Tooltip.Trigger>
		<Button variant="ghost" size="icon" class="p-2" href={page ?? '#'}>
			<svelte:component this={icon} />
		</Button>
	</Tooltip.Trigger>
	<Tooltip.Content>
		<p>{tooltip}</p>
	</Tooltip.Content>
</Tooltip.Root>
