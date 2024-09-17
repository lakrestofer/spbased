import type { Readable } from 'svelte/motion';
import { writable } from 'svelte/store';

export type Source = {
	name: string;
};

export type SourceStore = Readable<Source[]>;

const createSourcesStore = () => {
	const sources = writable<Source[]>([]);

	return {
		subscribe: sources.subscribe
	};
};

export const sources: SourceStore = createSourcesStore();
