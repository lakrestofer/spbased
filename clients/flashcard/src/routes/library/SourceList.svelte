<script lang="ts">
	import { createTable, Render, Subscribe } from 'svelte-headless-table';
	import { sources } from '$lib/model/source';
	import * as Table from '$lib/components/ui/table';

	const table = createTable(sources);

	const columns = table.createColumns([
		table.column({
			accessor: 'name',
			header: 'Name'
		}),
		table.column({
			accessor: 'n_fragments',
			header: 'Fragments'
		}),
		table.column({
			accessor: 'n_pages',
			header: 'Pages'
		}),
		table.column({
			accessor: 'completion',
			header: 'Completion',
			cell: ({ value }) => (value * 100).toFixed(2) + '%'
		}),
		table.column({
			accessor: 'tags',
			header: 'Tags'
		}),
		table.column({
			accessor: 'created_at',
			header: 'Created At',
			cell: ({ value }) => value.substring(0, 10)
		}),
		table.column({
			accessor: 'updated_at',
			header: 'Updated At',
			cell: ({ value }) => value.substring(0, 10)
		})
	]);
	const { headerRows, pageRows, tableAttrs, tableBodyAttrs } = table.createViewModel(columns);
</script>

<div class="rounded-md border">
	<Table.Root {...$tableAttrs}>
		<Table.Header>
			{#each $headerRows as headerRow}
				<Subscribe rowAttrs={headerRow.attrs()}>
					<Table.Row>
						{#each headerRow.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs props={cell.props()}>
								<Table.Head {...attrs}>
									<Render of={cell.render()} />
								</Table.Head>
							</Subscribe>
						{/each}
					</Table.Row>
				</Subscribe>
			{/each}
		</Table.Header>
		<Table.Body {...$tableBodyAttrs}>
			{#each $pageRows as row (row.id)}
				<Subscribe rowAttrs={row.attrs()} let:rowAttrs>
					<Table.Row {...rowAttrs}>
						{#each row.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs>
								<Table.Cell {...attrs}>
									<Render of={cell.render()} />
								</Table.Cell>
							</Subscribe>
						{/each}
					</Table.Row>
				</Subscribe>
			{/each}
		</Table.Body>
	</Table.Root>
</div>
