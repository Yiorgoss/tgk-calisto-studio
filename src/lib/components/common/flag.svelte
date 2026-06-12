<script lang="ts">
	import { defaultLocale, type SupportedLocalesKeys } from '@/config';

	const { flag, alt }: { flag?: string | number; alt?: string } = $props();
	const dynamicFlagsImport = async ({ name }: { name: SupportedLocalesKeys }) => {
		let a11y;
		let i;
		switch (name) {
			case 'gr':
			case 'greek':
			case 'el':
				a11y = Promise.resolve('Εικόνα της σημαίας Ελλάδα');
				i = import('../../assets/flags/gr.svg');
				break;
			case 'gb':
			case 'english':
			case 'en':
				a11y = Promise.resolve('Image of flag Great Britain');
				i = import('../../assets/flags/gb.svg');
				break;
		}
		if (!a11y || !i) throw Error('Flag not Found');
		return Promise.all([a11y, i]);
	};

	let name = $derived(flag ?? defaultLocale);
	let { a11y, svg } = $derived(
		// @ts-ignore
		await dynamicFlagsImport({ name }).then((x) => {
			return { a11y: x[0], svg: x[1].default };
		})
	);
</script>

{#if svg}
	<img class="h-4 aspect-4/3" src={svg} alt={a11y} />
{/if}
