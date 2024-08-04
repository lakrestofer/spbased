<script>
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import SideBar from './SideBar.svelte';
	import BottomBar from './BottomBar.svelte';
</script>

<ModeWatcher defaultMode="dark" />
<div class="flex h-screen max-h-screen flex-col bg-background">
	<div class="flex grow">
		<SideBar />
		<div class="flex-grow">
			<slot />
		</div>
	</div>
	<BottomBar />
</div>
