<script lang="ts">
	import { type IFlexboxLayout } from '@payload-types';
	import RenderBlocks from '../render-blocks.svelte';

	const { blockData }: { blockData: IFlexboxLayout } = $props();
</script>

<section id="flexbox-layout" class="">
	<div
		style:justify-center={blockData.style?.alignX}
		style:align-items={blockData.style?.alignY}
		style:gap={blockData.style?.gap ?? '30px'}
		style:flex-direction={blockData.style?.flexDirection}
		style:overflow={blockData.style?.overflow}
		style:padding={blockData.style?.padding ?? '0px 20px'}
		style:background={blockData.style?.background}
		class="w-full h-full container flex flex-wrap justify-center mx-auto"
	>
		{#each blockData.list ?? [] as { block }}
			<div class="flex grow justify-center xl:shrink basis-full sm:basis-1/3 xl:basis-1/5">
				<RenderBlocks blockData={block?.[0]} />
			</div>
		{/each}
	</div>
</section>
