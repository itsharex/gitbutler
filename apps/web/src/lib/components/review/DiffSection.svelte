<script lang="ts">
	import { isLockfile } from '$lib/diff/lockfiles';
	import { splitDiffIntoHunks } from '$lib/diffParsing';
	import Button from '@gitbutler/ui/Button.svelte';
	import HunkDiff, { type LineClickParams } from '@gitbutler/ui/HunkDiff.svelte';
	import FileIcon from '@gitbutler/ui/file/FileIcon.svelte';
	import type { DiffSection } from '@gitbutler/shared/branches/types';
	import type { ContentSection, LineSelector } from '@gitbutler/ui/utils/diffParsing';

	interface Props {
		isLoggedIn: boolean;
		section: DiffSection;
		selectedSha: string | undefined;
		selectedLines: LineSelector[];
		clearLineSelection: (fileName: string) => void;
		toggleDiffLine: (fileName: string, diffSha: string, params: LineClickParams) => void;
		onCopySelection: (contentSections: ContentSection[]) => void;
		onQuoteSelection: () => void;
	}
	const {
		isLoggedIn,
		section,
		toggleDiffLine,
		selectedSha,
		selectedLines: lines,
		onCopySelection,
		onQuoteSelection,
		clearLineSelection
	}: Props = $props();

	const lockFile = $derived.by(() => {
		if (!section.newPath) return false;
		return isLockfile(section.newPath);
	});

	let displayLockHunks = $state<boolean>(false);

	const hunks = $derived.by(() => {
		if (!section.diffPatch) return [];
		return splitDiffIntoHunks(section.diffPatch);
	});
	const filePath = $derived(section.newPath || 'unknown');

	function handleLineClick(params: LineClickParams) {
		toggleDiffLine(section.newPath || 'unknown', section.diffSha, params);
	}

	const selectedLines = $derived(selectedSha === section.diffSha ? lines : []);
</script>

<div class="diff-section">
	<div class="diff-section__header">
		<FileIcon fileName={filePath} size={16} />
		<p title={filePath} class="text-12 text-body file-name">{filePath}</p>
	</div>
	{#if lockFile && !displayLockHunks}
		<div class="lock-files-hidden-by-default">
			<p class="text-12 hidden-lock-file-message">Lock files are hidden by default</p>
			<Button kind="outline" icon="eye-shown" onclick={() => (displayLockHunks = true)}
				>Show diff</Button
			>
		</div>
	{:else}
		{#each hunks as hunkStr}
			<HunkDiff
				filePath={section.newPath || 'unknown'}
				{hunkStr}
				diffLigatures={false}
				{selectedLines}
				onLineClick={handleLineClick}
				{onCopySelection}
				onQuoteSelection={isLoggedIn ? onQuoteSelection : undefined}
				{clearLineSelection}
			/>
		{/each}
	{/if}
</div>

<style lang="postcss">
	.diff-section {
		display: flex;
		padding: 14px;
		flex-direction: column;
		align-items: flex-start;
		gap: 14px;
		align-self: stretch;

		&:not(:last-child) {
			border-bottom: 1px solid var(--clr-border-2);
		}
	}

	.diff-section__header {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.file-name {
		color: var(--clr-text-1);
	}

	.lock-files-hidden-by-default {
		width: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 40px 24px;
		border: 1px solid var(--clr-border-2);
		border-radius: var(--radius-ml);
		background: var(--clr-bg-1-muted);
	}

	.hidden-lock-file-message {
		color: var(--clr-text-2);
		text-align: center;
	}
</style>
