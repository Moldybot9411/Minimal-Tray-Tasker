import { db } from "$lib/db/database";
import * as schema from "$lib/db/schema";
import { and, eq, lt } from "drizzle-orm/sqlite-core/expressions";

export async function dbGetAllTrackers(): Promise<schema.SelectTracker[]> {
    let trackers: schema.SelectTracker[] = [];

    await db.query.trackers
        .findMany()
        .execute()
        .then((results) => {
            trackers = results;
        });

    return trackers;
}

export async function dbAddTracker(
    tracker: schema.InsertTracker,
): Promise<schema.SelectTracker> {
    const res = await db
        .insert(schema.trackers)
        .values(tracker)
        .returning()
        .get();

    const ret: schema.SelectTracker = {
        //@ts-ignore
        id: res.id["result"]["lastInsertId"],
        ...(tracker as schema.SelectTracker),
    };

    return ret;
}

export async function dbUpdateTrackerProgress(id: number, progress: number) {
    await db
        .update(schema.trackers)
        .set({ progress: progress })
        .where(eq(schema.trackers.id, id));
}

export async function dbCompleteTracker(id: number) {
    await db
        .update(schema.trackers)
        .set({ completed: true })
        .where(eq(schema.trackers.id, id));
}

export async function dbReopenTracker(id: number) {
    await db
        .update(schema.trackers)
        .set({ completed: false, progress: 0 })
        .where(eq(schema.trackers.id, id));
}

export async function dbDeleteTracker(id: number) {
    await db.delete(schema.trackers).where(eq(schema.trackers.id, id));
}

export async function dbRefreshDailyTrackers() {
    // Get 00:00 from today
    let now = new Date();
    let startOfDay = new Date(now.getFullYear(), now.getMonth(), now.getDate());

    await db
        .update(schema.trackers)
        .set({ completed: false, progress: 0 })
        .where(
            and(
                lt(schema.trackers.lastModifiedAt, startOfDay),
                eq(schema.trackers.isDaily, true),
            ),
        );
}
