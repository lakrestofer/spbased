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
		<NavButton icon={Home} />
		<NavButton icon={Library} />
		<NavButton icon={BookOpen} />
	</div>
	<!-- more navigation and settings -->
	<div class="flex flex-col">
		<ThemeToggleButton />
		<NavButton icon={Settings} />
	</div>
</div>
