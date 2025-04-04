<script lang="ts">
	import { onMount } from 'svelte';
	import type { AriaRole } from 'svelte/elements';

	interface Props {
		children: any;
		minTriggerCount: number;
		role?: AriaRole | undefined | null;
		ontrigger: (lastChild: Element) => void;
		onkeydown?: (e: KeyboardEvent) => void;
	}

	const { children, minTriggerCount, role, ontrigger, onkeydown }: Props = $props();

	let lazyContainerEl = $state<HTMLDivElement>();

	const mutuationObserver = new MutationObserver(attachIntersectionObserver);
	$effect(() => {
		if (lazyContainerEl) {
			mutuationObserver.disconnect();
			mutuationObserver.observe(lazyContainerEl, { childList: true });
			attachIntersectionObserver();
		}
	});
	const intersectionObserver = new IntersectionObserver((entries) => {
		entries.forEach((entry) => {
			if (entry.isIntersecting) {
				ontrigger(entry.target);
				intersectionObserver.unobserve(entry.target);
			}
		});
	});

	function attachIntersectionObserver() {
		// unattach all intersection observers
		intersectionObserver.disconnect();
		if (!lazyContainerEl) return;

		const containerChildren = lazyContainerEl.children;
		if (containerChildren.length < minTriggerCount) return;

		const lastChild = containerChildren[containerChildren.length - 1];
		if (!lastChild) return;

		intersectionObserver.observe(lastChild);
	}

	onMount(() => {
		return () => {
			intersectionObserver.disconnect();
			mutuationObserver.disconnect();
		};
	});

	export function hasFocus() {
		return (
			document.activeElement === lazyContainerEl ||
			lazyContainerEl?.contains(document.activeElement)
		);
	}
</script>

<div class="lazy-container" {role} bind:this={lazyContainerEl} {onkeydown}>
	{@render children()}
</div>

<style>
	.lazy-container {
		display: contents;
	}
</style>
