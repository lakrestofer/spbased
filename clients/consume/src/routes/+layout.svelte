<script lang="ts">
	import { ModeWatcher } from 'mode-watcher';
	import '../app.css';
	import { Button } from '$lib/components/ui/button';
	import { Bell, Brain, Home, Library, Settings } from 'lucide-svelte/icons';
	import ToggleThemeButton from './ToggleThemeButton.svelte';
	import SideBarLink from './SideBarLink.svelte';
	let { children } = $props();
</script>

<ModeWatcher />

<div class="flex h-screen w-screen flex-col">
	<!-- sidebar and main area -->
	<div class="flex flex-1">
		<!-- sidebar -->
		<div class="flex flex-col justify-between border-r p-2">
			<!-- upper section with logo -->
			<div class="flex flex-col items-center gap-2">
				<Brain class="size-10" />
			</div>
			<!-- middle section with links -->
			<div class="flex flex-col items-center gap-2">
				<SideBarLink href="/" title="Home" icon={Home} />
				<SideBarLink href="/library" title="Library" icon={Library} />
			</div>
			<!-- bottom section with settings icon and toggle theme icon -->
			<div class="flex flex-col items-center gap-2">
				<SideBarLink href="/settings" title="Settings" icon={Settings} />
				<ToggleThemeButton />
			</div>
		</div>
		<!-- main area -->
		<div class="flex-1 overflow-auto">
			{@render children?.()}
		</div>
	</div>
	<!-- bottom bar -->
	<div class="flex justify-between border-t p-1">
		<!-- left section -->
		<div></div>
		<!-- right section -->
		<div>
			<Button variant="outline" size="icon">
				<Bell class="size-5" />
			</Button>
		</div>
	</div>
</div>
