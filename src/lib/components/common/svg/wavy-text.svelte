<script lang="ts">
	import { animate, scroll } from 'motion';
	import { onMount } from 'svelte';

	const { data }: { data: any } = $props();

	let elem: HTMLElement;
	onMount(() =>
		scroll(
			animate(elem.querySelectorAll('svg text textPath'), {
				//@ts-ignore
				startOffset: '100%',
				repeat: Infinity,
				repeatType: 'mirror'
			}),
			{
				offset: ['end 0.7', 'end start'],
				target: elem
			}
		)
	);
</script>

<div bind:this={elem}>
	<svg
		class="absolute inset-0 h-full w-full"
		viewBox={data.viewbox ?? '0 0 100 100'}
		preserveAspectRatio="xMinYMin slice"
	>
		<path
			stroke={data.strokeColor ?? 'transparent'}
			stroke-width={data.strokeWidth}
			stroke-linecap="round"
			id="wavy-text-path"
			d={data.path}
			fill="none"
		/>
		<text font-size={data.fontSize} word-spacing={data.wordSpacing}>
			<textPath
				href="#wavy-text-path"
				alignment-baseline="middle"
				fill={data.color}
				startOffset="0"
				text-anchor="middle"
			>
				{data.text}
			</textPath>
		</text>
	</svg>
</div>
