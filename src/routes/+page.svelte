<script lang="ts">
    import Check from "@lucide/svelte/icons/check";
    import Minus from "@lucide/svelte/icons/minus";
    import {
        BadgePlus,
        CalendarCheck,
        Pickaxe,
        RotateCcw,
        Save,
        Settings,
        TimerReset,
        Trash2,
    } from "@lucide/svelte/icons/index";
    import Card from "$lib/Components/Card.svelte";
    import Dialog from "$lib/Components/Dialog.svelte";
    import { onMount } from "svelte";
    import { TrackerService, trackers } from "$lib/trackerService";
    import type { SelectTracker } from "$lib/db/schema";
    import Button from "$lib/Components/Button.svelte";
    import Switch from "$lib/Components/Switch.svelte";
    import {
        settings,
        SettingsService,
        Settings as SettingsType,
    } from "$lib/settingsService";
    import { Settings2Icon } from "@lucide/svelte";
    import TrackerList from "$lib/Components/TrackerList.svelte";

    let new_tracker_name: string = $state("");
    let new_amount: number | undefined = $state();
    let new_is_daily: boolean = $state(false);
    $effect(() => {
        if (new_amount != undefined) {
            new_amount = Math.abs(new_amount);
        }
    });

    let loading: boolean = $state(false);

    let activeDialog: "new" | "settings" | "discard" | null = $state(null);

    let itemToDelete: number | undefined = $state(undefined);

    onMount(async () => {
        refreshTrackers();
    });

    async function refreshTrackers() {
        loading = true;
        await TrackerService.refresh();
        loading = false;
    }

    async function createTracker(
        name: string,
        amount?: number,
        daily?: boolean,
    ) {
        if (!name.trim()) return;

        TrackerService.add(name, amount, daily);

        await refreshTrackers();

        new_tracker_name = "";
        new_amount = undefined;
    }
</script>

<Card class="flex p-4 gap-2 rounded-t-none">
    <Button
        class="w-full"
        icon={BadgePlus}
        label="Add Tracker"
        onclick={() => (activeDialog = "new")} />
    <Button
        variant="secondary"
        icon={Settings2Icon}
        onclick={() => (activeDialog = "settings")} />
</Card>

<!-- TODO -->
<Card
    class="flex flex-col mt-4 gap-2 p-4 text-xl font-bold {loading
        ? 'animate-pulse'
        : ''}">
    <div class="flex justify-center items-center gap-2 mb-2">
        <Pickaxe />
        <div>Todo</div>
    </div>
    <TrackerList
        trackers={$trackers.filter((x) => x.completed === false)}
        ondelete={(id) => {
            itemToDelete = id;
            activeDialog = "discard";
        }} />
</Card>

<!-- COMPLETED DAILIES -->
<Card
    class="flex flex-col mt-4 gap-2 p-4 text-xl font-bold {loading
        ? 'animate-pulse'
        : ''}">
    <div class="flex justify-center items-center gap-2 mb-2">
        <TimerReset />
        <span>Completed Dailies</span>
    </div>
    <TrackerList
        trackers={$trackers.filter((x) => x.completed && x.isDaily)}
        variant="completed"
        ondelete={(id) => {
            itemToDelete = id;
            activeDialog = "discard";
        }} />
</Card>

<!-- COMPLETED -->
<Card
    class="flex flex-col mt-4 gap-2 p-4 text-xl font-bold {loading
        ? 'animate-pulse'
        : ''}">
    <div class="flex justify-center items-center gap-2 mb-2">
        <CalendarCheck />
        <span>Completed</span>
    </div>
    <TrackerList
        trackers={$trackers.filter((x) => x.completed && !x.isDaily)}
        variant="completed"
        ondelete={(id) => {
            itemToDelete = id;
            activeDialog = "discard";
        }} />
</Card>

<Dialog label="New Tracker" visible={activeDialog === "new"}>
    <div class="p-2 flex flex-col gap-2">
        <div class="flex flex-col">
            <label for="newtracker" class="font-bold">Tracker Name</label>
            <input
                type="text"
                name="newtracker"
                id="newtracker"
                class="p-2 rounded-md bg-white"
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
                class="p-2 rounded-md bg-white"
                bind:value={new_amount}
                placeholder="1" />
        </div>

        <Switch
            label="Make Tracker repeat daily"
            name="dailycheckbox"
            bind:checked={new_is_daily} />

        <Button
            onclick={() => {
                loading = true;
                createTracker(new_tracker_name, new_amount, new_is_daily).then(
                    () => {
                        activeDialog = null;
                    },
                );
            }}
            disabled={new_tracker_name || new_amount ? false : true}
            class="w-full"
            icon={BadgePlus}
            label="Add Tracker" />
    </div>
</Dialog>

<Dialog label="Settings" visible={activeDialog === "settings"}>
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
</Dialog>

<Dialog label="Delete Tracker" visible={activeDialog === "discard"}>
    <div class="flex gap-2">
        <Button
            variant="secondary"
            class="w-full"
            label="Cancel"
            onclick={() => (activeDialog = null)} />
        <Button
            class="w-full"
            label="Delete"
            icon={Trash2}
            onclick={() => {
                if (itemToDelete) {
                    TrackerService.remove(itemToDelete);
                }
                activeDialog = null;
            }} />
    </div>
</Dialog>
