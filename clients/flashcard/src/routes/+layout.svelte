<script>
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import SideBar from './SideBar.svelte';
	import BottomBar from './BottomBar.svelte';
	import { Toaster } from '$lib/components/ui/sonner';
</script>

<ModeWatcher defaultMode="dark" />
<Toaster />
<div class="bg-background flex h-screen max-h-screen flex-col">
	<div class="flex grow">
		<SideBar />
		<div class="flex-grow">
			<slot />
		</div>
	</div>
	<BottomBar />
</div>
