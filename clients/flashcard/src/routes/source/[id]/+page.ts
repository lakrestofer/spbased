import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { sources } from '$lib/model/source';

export const load: PageLoad = ({ params }) => {
	const id = params?.id;
	if (!id) throw error(400, 'Missing id');

	const source = sources.getSource(id);

	if (!source) throw error(404, 'Source not found');

	return {
		source
	};
};
