<script lang="ts">
    import { goto } from "$app/navigation";
    import favicon from "$lib/assets/favicon.svg";
    import Button from "$lib/Components/Button.svelte";
    import Card from "$lib/Components/Card.svelte";
    import Dialog from "$lib/Components/Dialog.svelte";
    import Switch from "$lib/Components/Switch.svelte";
    import {
        settings,
        SettingsService,
        Settings as SettingsType,
    } from "$lib/settingsService";
    import { deleteDialog } from "$lib/stores/deleteStore";
    import { TrackerService } from "$lib/trackerService";
    import {
        BadgePlus,
        CalendarCheck,
        Moon,
        Pickaxe,
        Settings2,
        TimerReset,
        Trash2,
    } from "@lucide/svelte";
    import { listen } from "@tauri-apps/api/event";
    import { onMount, type ComponentProps } from "svelte";
    import "../app.css";
    import { page } from "$app/state";

    let { children } = $props();

    let activeDialog: "new" | "settings" | undefined = $state();
    let newTrackerDialogVisible = $derived(activeDialog === "new");
    let settingsDialogVisible = $derived(activeDialog === "settings");

    let new_tracker_name: string = $state("");
    let new_amount: number | undefined = $state();
    let new_is_daily: boolean = $state(false);
    $effect(() => {
        if (new_amount) {
            new_amount = Math.abs(new_amount);
        }
    });
    $inspect(activeDialog);

    let buttons: ComponentProps<typeof Button>[] & { href?: string }[] = [
        {
            variant: "secondary",
            icon: Pickaxe,
            href: "/",
            title: "To do",
        },
        {
            variant: "secondary",
            icon: TimerReset,
            href: "/daily",
            title: "Finished daily tasks",
        },
        {
            variant: "secondary",
            icon: CalendarCheck,
            href: "/completed",
            title: "Finished tasks",
        },
    ];

    onMount(async () => {
        let lightmode: boolean = localStorage?.getItem("theme") === "light";
        if (lightmode == false) {
            document.documentElement.classList.add("dark");
        }

        SettingsService.refresh();
        await TrackerService.refresh();
        await TrackerService.refreshDailies();

        listen("refresh_dailies", () => {
            TrackerService.refreshDailies();
        });
    });

    async function createTracker(
        name: string,
        amount?: number,
        daily?: boolean,
    ) {
        if (!name.trim()) return;

        TrackerService.add(name, amount, daily);

        new_tracker_name = "";
        new_amount = undefined;
    }

    function toggleTheme() {
        let lightmode: boolean = localStorage?.getItem("theme") === "light";

        if (lightmode) {
            localStorage?.setItem("theme", "dark");
            document.documentElement.classList.add("dark");
            return;
        }

        localStorage?.setItem("theme", "light");
        document.documentElement.classList.remove("dark");
    }
</script>

<svelte:head>
    <link rel="icon" href={favicon} />
</svelte:head>

<Card class="mb-2 flex flex-col justify-center gap-4">
    <div class="flex gap-2">
        <Button
            icon={BadgePlus}
            label="Add Tracker"
            class="w-full"
            onclick={() => (activeDialog = "new")}
            title="Add Tracker" />
        <Button
            variant="secondary"
            icon={Settings2}
            onclick={() => (activeDialog = "settings")}
            title="Open Settings" />
        <Button
            variant="secondary"
            icon={Moon}
            onclick={() => toggleTheme()}
            title="Toggle Dark/Light Mode" />
    </div>
    <!-- <div class="border-b-2 border-gray-500"></div> -->
    <div class="flex gap-2">
        {#each buttons as button}
            {@const isActive = page.url.pathname === button.href}

            <div class="relative flex w-full flex-col items-center">
                <Button
                    variant={button.variant}
                    icon={button.icon}
                    title={button.title}
                    class="w-full"
                    onclick={() => {
                        if (button.href) goto(button.href);
                    }} />

                {#if isActive}
                    <div
                        class="absolute -bottom-3 h-1.5 w-4.5 rounded-full bg-indigo-400 dark:bg-blue-500">
                    </div>
                {/if}
            </div>
        {/each}
    </div>
</Card>

{@render children?.()}

<Dialog label="Delete Tracker" bind:visible={$deleteDialog.open}>
    <div class="flex gap-2">
        <Button
            variant="secondary"
            class="w-full"
            label="Cancel"
            onclick={() => deleteDialog.close()} />
        <Button
            class="w-full"
            label="Delete"
            icon={Trash2}
            onclick={() => deleteDialog.confirm()} />
    </div>
</Dialog>

<Dialog
    label="New Tracker"
    bind:visible={newTrackerDialogVisible}
    onclose={() => (activeDialog = undefined)}>
    <div class="p-2 flex flex-col gap-2">
        <div class="flex flex-col">
            <label for="newtracker" class="font-bold">Tracker Name</label>
            <input
                type="text"
                name="newtracker"
                id="newtracker"
                class="p-2 rounded-md bg-white dark:bg-gray-700"
                bind:value={new_tracker_name}
                placeholder="Drink water" />
        </div>

        <div class="flex flex-col">
            <label for="newamount" class="font-bold">Amount</label>
            <input
                type="number"
                name="newamount"
                id="newamount"
                min="1"
                class="p-2 rounded-md bg-white dark:bg-gray-700"
                bind:value={new_amount}
                placeholder="1" />
        </div>

        <Switch
            label="Make Tracker repeat daily"
            name="dailycheckbox"
            bind:checked={new_is_daily} />

        <Button
            onclick={() => {
                createTracker(new_tracker_name, new_amount, new_is_daily);
                activeDialog = undefined;
            }}
            disabled={new_tracker_name || new_amount ? false : true}
            class="w-full"
            icon={BadgePlus}
            label="Add Tracker" />
    </div>
</Dialog>

<Dialog
    label="Settings"
    bind:visible={settingsDialogVisible}
    onclose={() => (activeDialog = undefined)}>
    <Switch
        class="font-bold"
        label="Autostart"
        name="autostart"
        checked={$settings[SettingsType.autostart]}
        onclick={(e) =>
            SettingsService.setAutostart(e.currentTarget.checked)} />
    <Switch
        class="font-bold"
        label="Enable Reminder"
        name="reminder"
        checked={$settings[SettingsType.reminders]}
        onclick={(e) =>
            SettingsService.setReminders(e.currentTarget.checked)} />
    <Switch
        class="font-bold"
        label="Enable Ram Saver"
        name="ramsaver"
        checked={$settings[SettingsType.ramsaver]}
        onclick={(e) => SettingsService.setRamSaver(e.currentTarget.checked)} />
</Dialog>

<style>
    :root {
        padding: 0.5rem;
    }

    :root.dark {
        background-color: var(--color-gray-700);
        color: white;
    }
</style>
