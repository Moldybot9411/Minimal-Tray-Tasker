import {
    dbGetAllTrackers,
    dbAddTracker,
    dbUpdateTrackerProgress,
    dbCompleteTracker,
    dbDeleteTracker,
    dbReopenTracker,
    dbRefreshDailyTrackers,
} from "$lib/appdata";
import type { InsertTracker, SelectTracker } from "$lib/db/schema";
import { writable } from "svelte/store";

export const TrackerService = {
    async refresh(): Promise<void> {
        trackers.set(await dbGetAllTrackers());
    },

    async add(name: string, amount?: number, daily?: boolean): Promise<void> {
        const tracker: InsertTracker = {
            id: undefined,
            name,
            amount: amount ?? 1,
            progress: 0,
            completed: false,
            isDaily: daily ?? false,
        };

        const new_tracker = await dbAddTracker(tracker);
        trackers.update((current) => {
            return [...current, new_tracker];
        });
    },

    async increment(tracker: SelectTracker): Promise<void> {
        const newProgress = Math.min(tracker.progress + 1, tracker.amount);

        trackers.update((current) =>
            current.map((item) =>
                item.id === tracker.id
                    ? {
                          ...item,
                          progress: newProgress,
                          completed: newProgress === item.amount,
                      }
                    : item,
            ),
        );

        await dbUpdateTrackerProgress(tracker.id, newProgress);
        if (newProgress === tracker.amount) await dbCompleteTracker(tracker.id);
    },

    async decrement(tracker: SelectTracker): Promise<void> {
        const newProgress = Math.max(tracker.progress - 1, 0);

        trackers.update((current) =>
            current.map((item) =>
                item.id === tracker.id
                    ? { ...item, progress: newProgress }
                    : item,
            ),
        );

        await dbUpdateTrackerProgress(tracker.id, newProgress);
    },

    async remove(id: number): Promise<void> {
        trackers.update((current) => {
            return current.filter((item) => {
                return item.id !== id;
            });
        });
        await dbDeleteTracker(id);
    },

    async reopen(tracker: SelectTracker): Promise<void> {
        trackers.update((current) =>
            current.map((item) =>
                item.id === tracker.id
                    ? { ...item, progress: 0, completed: false }
                    : item,
            ),
        );
        await dbReopenTracker(tracker.id);
    },

    async refreshDailies(): Promise<void> {
        // Get 00:00 from today
        let now = new Date();
        let startOfDay = new Date(
            now.getFullYear(),
            now.getMonth(),
            now.getDate(),
        );

        trackers.update((current) => {
            current.forEach((tracker) => {
                if (!tracker.isDaily) return;
                if (tracker.lastModifiedAt >= startOfDay) return;

                tracker.progress = 0;
                tracker.completed = false;
            });

            return current;
        });

        dbRefreshDailyTrackers();
    },
};

export const trackers = writable<SelectTracker[]>([]);
