<script lang="ts">
	import type { ITGKGridIcons } from '@payload-types';
	import Icon from '@/components/common/icon.svelte';
	import { RichTextRender } from '@/components/blocks/rich-text';

	const { blockData }: { blockData: ITGKGridIcons } = $props();
</script>

<section id="TGKGridIcons" style:background={blockData.style?.background} class="">
	<div
		style:padding={blockData.style?.padding}
		style:align-items={blockData.style?.alignY}
		style:justify-content={blockData.style?.alignX}
		style:gap={blockData.style?.gap}
		style:width={blockData.style?.width}
		class:container={blockData.style?.container}
		class="flex flex-wrap justify-evenly items-start gap-4"
	>
		{#each blockData.arr ?? [] as { icon, richText }}
			<div
				class="flex duration-200 transition-transform hover:scale-110 flex-col justify-center items-center"
			>
				<div class="">
					<Icon {icon} />
				</div>
				<div class="">
					<RichTextRender {richText} />
				</div>
			</div>
		{/each}
	</div>
</section>
