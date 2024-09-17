import type { Readable } from 'svelte/motion';
import { get, writable } from 'svelte/store';

import { faker } from '@faker-js/faker';

/// source object
export type Source = {
	id: string;
	name: string;
	url?: string;
};
/// new source object
export type NewSource = {
	name: string;
};

const createRandomSources = (count: number): Source[] => {
	return Array.from({ length: count }, () => ({
		id: faker.string.uuid(),
		name: faker.company.name(),
		url: faker.image.url()
	}));
};

const initialSources = createRandomSources(5);

export type SourceStore = Readable<Source[]> & {
	addSource: (source: NewSource) => void;
	editSource: (source: Source) => void;
	getSource: (id: string) => Source | undefined;
};

const createSourcesStore = () => {
	const sources = writable<Source[]>(initialSources);

	const addSource = (source: NewSource) => {
		sources.update((s) => [...s, { id: faker.string.uuid(), name: source.name }]);
	};

	const editSource = (source: Source) => {
		sources.update((s) => {
			const index = s.findIndex((x) => x.id === source.id);
			if (index === -1) return s;
			const updated = [...s];
			updated[index] = source;
			return updated;
		});
	};

	const getSource = (id: string) => {
		const _sources = get(sources);
		return _sources.find((x) => x.id === id);
	};

	return {
		subscribe: sources.subscribe,
		addSource,
		editSource,
		getSource
	};
};

export const sources: SourceStore = createSourcesStore();
