<script lang="ts">
	import type { ITGKDoilyCard } from '@payload-types';
	import Image from '@/components/common/Image.svelte';

	const { blockData }: { blockData: ITGKDoilyCard } = $props();
</script>

<!--  <section id="TGKDoilyCard" class="">
	<div class="grid-cols-1 grid-rows-1">
		<div class="col-start-1 row-start-1">
			<Image image={blockData.bg} class="" />
		</div>
		<div class="col-start-1 row-start-1">
			<Image image={blockData.img} class="" />
		</div>
	</div>
</section>  -->
