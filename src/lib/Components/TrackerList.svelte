<script lang="ts">
    import { TrackerService } from "$lib/trackerService";
    import {
        Check,
        Minus,
        Plus,
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

<div class="flex flex-col gap-2">
    {#each trackers as tracker}
        <div
            class="flex justify-between p-2 dark:bg-gray-700 bg-gray-300 rounded-md shadow-sm dark:shadow-gray-800 shadow-gray-400">
            <div class="flex wrap-anywhere items-center gap-2">
                {#if variant === "completed"}
                    <Check color="lime" class="shrink-0" />
                {/if}
                {#if variant === "todo" && tracker.amount > 1}
                    <div
                        class="flex self-start items-center text-nowrap bg-gray-300 dark:bg-gray-500 p-0.5 px-1 rounded-md">
                        {tracker.progress}/{tracker.amount}
                        {#if tracker.isDaily}
                            <TimerReset class="opacity-40 ml-1" />
                        {/if}
                    </div>
                {/if}
                {tracker.name}
            </div>
            <div class="flex gap-1 ml-2">
                {#if variant === "todo"}
                    {#if tracker.amount - tracker.progress === 1}
                        <Button
                            variant="accept"
                            onclick={() => TrackerService.increment(tracker)}
                            class="!p-1 w-max h-max"
                            icon={Check}
                            title="Finish Task" />
                    {:else}
                        <Button
                            variant="accept"
                            onclick={() => TrackerService.increment(tracker)}
                            class="!p-1 w-max h-max"
                            icon={Plus}
                            title="Increment Task" />
                    {/if}
                {:else}
                    <Button
                        variant="accept"
                        onclick={() => TrackerService.reopen(tracker)}
                        class="!p-1 w-max h-max"
                        icon={RotateCcw}
                        title="Reopen Task" />
                {/if}
                <Button
                    variant="danger"
                    onclick={() => {
                        ondelete(tracker.id);
                    }}
                    class="!p-1 w-max h-max"
                    icon={Trash2}
                    title="Delete Tracker" />
            </div>
        </div>
    {/each}
</div>
