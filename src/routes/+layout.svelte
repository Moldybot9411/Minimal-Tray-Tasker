<script lang="ts">
    import favicon from "$lib/assets/favicon.svg";
    import { onMount, type ComponentProps } from "svelte";
    import "../app.css";
    import { TrackerService } from "$lib/trackerService";
    import { SettingsService } from "$lib/settingsService";
    import { listen } from "@tauri-apps/api/event";
    import Card from "$lib/Components/Card.svelte";
    import Button from "$lib/Components/Button.svelte";
    import {
        BadgePlus,
        CalendarCheck,
        CalendarCheck2,
        Pickaxe,
        Repeat,
        Settings2,
        Timer,
        TimerReset,
    } from "@lucide/svelte";
    import { type Icon as IconType } from "@lucide/svelte";
    import { goto } from "$app/navigation";

    let { children } = $props();

    let buttons: ComponentProps<typeof Button>[] & { href?: string }[] = [
        {
            variant: "secondary",
            icon: Pickaxe,
            href: "todo",
        },
        {
            variant: "secondary",
            icon: TimerReset,
            href: "daily",
        },
        {
            variant: "secondary",
            icon: CalendarCheck,
            href: "#completed",
        },
    ];

    onMount(() => {
        TrackerService.refreshDailies();
        SettingsService.refresh();

        listen("refresh_dailies", () => {
            TrackerService.refreshDailies();
        });
    });
</script>

<svelte:head>
    <link rel="icon" href={favicon} />
</svelte:head>

<Card class="mb-4 flex gap-2 w-max">
    <Button icon={BadgePlus} />
    <div class="border-l-2 border-gray-400"></div>
    <div class="flex gap-2">
        {#each buttons as button}
            <Button
                variant={button.variant}
                icon={button.icon}
                onclick={() => {
                    if (button.href) goto(button.href);
                }} />
        {/each}
    </div>
    <div class="border-l-2 border-gray-400"></div>
    <Button variant="secondary" icon={Settings2} />
</Card>

{@render children?.()}
