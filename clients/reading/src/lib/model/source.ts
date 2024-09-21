import type { Readable } from 'svelte/motion';
import { get, writable } from 'svelte/store';

import { faker } from '@faker-js/faker';

/// source object
export type Source = {
	id: string;
	name: string;
	n_fragments: number;
	n_pages: number;
	completion: number;
	tags: string[];
	created_at: string;
	updated_at: string;
};
/// new source object
export type NewSource = {
	name: string;
};

const createRandomSources = (count: number): Source[] => {
	return Array.from({ length: count }, () => {
		const n_fragments = faker.number.int({ min: 0, max: 100 });
		const n_pages = faker.number.int({ min: n_fragments, max: 323 });
		const completion = n_fragments / n_pages;

		const tags = Array.from({ length: faker.number.int({ min: 0, max: 5 }) }, () =>
			faker.hacker.noun()
		);

		return {
			id: faker.string.uuid(),
			name: faker.company.name(),
			n_fragments,
			n_pages,
			completion,
			tags,
			created_at: faker.date.past({ years: 3 }).toISOString(),
			updated_at: faker.date.recent().toISOString()
		};
	});
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
		const now = new Date().toISOString();
		sources.update((s) => [
			...s,
			{
				id: faker.string.uuid(),
				name: source.name,
				n_fragments: 0,
				n_pages: faker.number.int({ min: 10, max: 323 }),
				completion: 0,
				tags: [],
				created_at: now,
				updated_at: now
			}
		]);
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
