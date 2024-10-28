export type Tag = {
	name: string;
};

const defaultTags = [
	{
		name: 'Tag 1'
	},
	{
		name: 'Tag 2'
	},
	{
		name: 'Tag 3'
	}
];

class TagStore {
	value: Tag[] = $state(defaultTags);

	addTag(name: string) {
		this.value.push({ name });
	}

	removeTag(tag: Tag) {
		this.value = this.value.filter((t) => t.name !== tag.name);
	}
}

export const tags = new TagStore();
