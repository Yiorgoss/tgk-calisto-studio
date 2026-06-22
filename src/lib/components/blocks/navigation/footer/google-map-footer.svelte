<script lang="ts">
	import type { IGoogleMapFooter } from '@payload-types';
	import Button from '@/components/common/button.svelte';
	import Image from '@/components/common/Image.svelte';

	const { blockData }: { blockData: IGoogleMapFooter } = $props();
</script>

<section id="GoogleMapFooter">
	<div class="container mx-auto py-10">
		<div class="">
			<Image image={blockData.logo} />
		</div>
		<div class="grid grid-cols-2 md:grid-cols-6 gap-5 justify-around items-center w-full">
			{#each blockData.list as { links, header, horizontal }, i}
				{@const k = 3}
				{@const lastRow = blockData.list.length % k}
				<div
					class:col-span-full={lastRow == 1 && blockData.list.length - 1 == i}
					class:col-span-3={lastRow == 2 && blockData.list.length - 2 <= i}
					class="col-span-2"
				>
					<div class="text-2xl text-center py-5">{header}</div>
					<div class:flex-row={horizontal} class="flex flex-col justify-center items-center">
						{#each links as { link }}
							{console.log({ link })}
							<Button {link} class="h-6" />
						{/each}
					</div>
				</div>
			{/each}
		</div>
	</div>
	<div class="w-full pt-10">
		{@html blockData.iframe}
	</div>
</section>
