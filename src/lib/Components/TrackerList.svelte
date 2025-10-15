<script lang="ts">
    import { TrackerService } from "$lib/trackerService";
    import {
        Check,
        Minus,
        RotateCcw,
        TimerReset,
        Trash2,
    } from "@lucide/svelte";
    import Button from "./Button.svelte";
    import type { SelectTracker } from "$lib/db/schema";

    interface Props {
        trackers: SelectTracker[];
        variant?: "todo" | "completed";
        ondelete: (id: number) => void;
    }

    let {
        trackers = [],
        variant = "todo",
        ondelete = () => {},
    }: Props = $props();
</script>

{#each trackers as tracker}
    <div class="flex items-center justify-between">
        <div class="flex items-center wrap-anywhere gap-2">
            {#if variant === "completed"}
                <Check color="green" />
            {/if}
            {tracker.name}
            {tracker.progress}/{tracker.amount}
            {#if tracker.isDaily}
                <TimerReset class="opacity-40" />
            {/if}
        </div>
        <div class="flex gap-1">
            {#if variant === "todo"}
                <Button
                    variant="accept"
                    onclick={() => TrackerService.increment(tracker)}
                    class="!p-1"
                    icon={Check} />
                <Button
                    variant="warning"
                    onclick={() => TrackerService.decrement(tracker)}
                    class="!p-1"
                    icon={Minus} />
            {:else}
                <Button
                    class="!p-1"
                    onclick={() => TrackerService.reopen(tracker)}
                    variant="accept"
                    icon={RotateCcw} />
            {/if}
            <Button
                variant="danger"
                onclick={() => {
                    ondelete(tracker.id);
                }}
                class="!p-1"
                icon={Trash2} />
        </div>
    </div>
{/each}
