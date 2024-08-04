<script>
	import { Button } from '$lib/components/ui/button';
	import { BookOpen, BrainCog, Home, Library, Settings } from 'lucide-svelte';
	import NavButton from './NavButton.svelte';
	import ThemeToggleButton from './ThemeToggleButton.svelte';
</script>

<div class="flex h-full flex-col items-center justify-between border-r p-2">
	<!-- header -->
	<div>
		<Button variant="ghost" size="icon">
			<BrainCog class="size-12" />
		</Button>
	</div>
	<!-- navigation -->
	<div class="flex flex-col">
		<NavButton icon={Home} page={'/'} tooltip="Home" />
		<NavButton icon={Library} page={'/library'} tooltip="Library" />
		<NavButton icon={BookOpen} page={'study'} tooltip="Study" />
	</div>
	<!-- more navigation and settings -->
	<div class="flex flex-col">
		<ThemeToggleButton />
		<NavButton icon={Settings} page="/settings" tooltip="Settings" />
	</div>
</div>
