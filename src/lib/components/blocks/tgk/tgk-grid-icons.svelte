<script lang="ts">
	import type { ITGKGridIcons } from '@payload-types';
	import Icon from '@/components/common/icon.svelte';
	import { RichTextRender } from '@/components/blocks/rich-text';

	const { blockData }: { blockData: ITGKGridIcons } = $props();
</script>

<section id="TGKGridIcons" class="">
	<div
		style:padding={blockData.style?.padding}
		style:align-items={blockData.style?.alignY}
		style:justify-content={blockData.style?.alignX}
		style:gap={blockData.style?.gap}
		style:width={blockData.style?.width}
		class:container={blockData.style?.container}
		class="flex flex-wrap justify-center items-center gap-4"
	>
		{#each blockData.arr ?? [] as { icon, richText }}
			<div class="flex flex-col">
				<div class="">
					<Icon {icon} />
				</div>
				<div class="">
					<RichTextRender {richText} />
					<RichTextRender {richText} />
					<RichTextRender {richText} />
					<RichTextRender {richText} />
				</div>
			</div>
		{/each}
	</div>
</section>
