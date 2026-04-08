<script lang="ts">
	import Image from '@/components/common/image.svelte';
	import { cn } from '@/utils';
	import { type IImage } from '@payload-types';

	const { blockData }: { blockData: IImage } = $props();
</script>

<div id="image-block" class="h-fit w-fit">
	<Image class="" image={blockData.image} />
</div>
