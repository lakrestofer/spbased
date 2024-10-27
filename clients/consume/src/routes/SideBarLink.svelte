<script module lang="ts">
	export interface SideBarLinkProps {
		href: string;
		title: string;
		icon: ComponentType<IconT>;
	}
</script>

<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import type { Icon as IconT } from 'lucide-svelte';
	import type { ComponentType } from 'svelte';

	let { href, title, icon: Icon }: SideBarLinkProps = $props();
</script>

<Tooltip.Provider>
	<Tooltip.Root>
		<Tooltip.Trigger>
			<Button variant="ghost" size="icon" {href}>
				<Icon />
			</Button>
		</Tooltip.Trigger>
		<Tooltip.Content>
			<p>{title}</p>
		</Tooltip.Content>
	</Tooltip.Root>
</Tooltip.Provider>
