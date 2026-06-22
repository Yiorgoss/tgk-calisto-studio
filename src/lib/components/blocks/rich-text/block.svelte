<script lang="ts">
	import { type IRichText } from '@payload-types';
	import Button from '@/components/common/button.svelte';
	import Image from '@/components/common/Image.svelte';
	import RichTextRender from '@/components/blocks/rich-text/render.svelte';

	const { blockData }: { blockData: IRichText } = $props();
</script>

<section id="rich-text-block" class="relative container mx-auto py-0 md:py-5">
	{#if blockData.richText}
		<RichTextRender richText={blockData.richText} />
	{/if}
	{#if blockData.richText.img}
		<Image image={blockData.richText.img} class="absolute inset-0" />
	{/if}
</section>
