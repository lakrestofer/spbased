import type { Readable } from 'svelte/motion';
import { get, writable } from 'svelte/store';

import { faker } from '@faker-js/faker';

/// source object
export type Tag = {
	id: string;
	name: string;
	color: string;
	created_at: string;
	updated_at: string;
};
/// new source object
export type NewTag = {
	name: string;
};

const createRandomTags = (count: number): Tag[] => {
	return Array.from({ length: count }, () => {
		return {
			id: faker.string.uuid(),
			name: faker.company.name(),
			color: faker.color.rgb(),

			created_at: faker.date.past({ years: 3 }).toISOString(),
			updated_at: faker.date.recent().toISOString()
		};
	});
};

const initialSources = createRandomTags(5);

export type TagStore = Readable<Tag[]> & {
	addSource: (source: NewTag) => void;
	editSource: (source: Tag) => void;
	getSource: (id: string) => Tag | undefined;
};

const createTagStore = () => {
	const sources = writable(initialSources);

	const addSource = (tag: NewTag) => {
		const now = new Date().toISOString();
		sources.update((s) => [
			...s,
			{
				id: faker.string.uuid(),
				name: tag.name,
				color: faker.color.rgb(),
				created_at: now,
				updated_at: now
			}
		]);
	};

	const editSource = (tag: Tag) => {
		sources.update((s) => {
			const index = s.findIndex((x) => x.id === tag.id);
			if (index === -1) return s;
			const updated = [...s];
			updated[index] = tag;
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

export const tags: TagStore = createTagStore();
