<script lang="ts">
	import type { IIcon } from '@payload-types';
	import 'iconify-icon';

	const {
		icon,
		name,
		class: className,
		style
	}: { icon?: IIcon; name?: string; class?: string; style?: string } = $props();
</script>

<iconify-icon
	class={className}
	height={icon?.size ?? 'none'}
	width={icon?.size ?? 'none'}
	icon={name ?? icon?.name ?? ''}
	style:padding={icon?.style?.padding}
	style:height={icon?.style?.height ?? 'inherit'}
	style:width={icon?.style?.width ?? 'inherit'}
	style:color={icon?.style?.color}
	style:background={icon?.style?.background}
	style:border={icon?.style?.border}
	style:border-radius={icon?.style?.borderRadius}
	style={style ?? icon?.style?.string}
>
</iconify-icon>
