import {
    sqliteTable,
    integer,
    text,
    SQLiteBoolean,
    primaryKey,
} from "drizzle-orm/sqlite-core";

export const trackers = sqliteTable("trackers", {
    id: integer().primaryKey(),
    name: text().notNull(),
    amount: integer().notNull(),
    progress: integer().notNull(),
    completed: integer({ mode: "boolean" }).notNull(),
    isDaily: integer("is_daily", { mode: "boolean" }).notNull(),
    lastModifiedAt: integer("last_modified_at", {
        mode: "timestamp",
    })
        .notNull()
        .$defaultFn(() => new Date())
        .$onUpdateFn(() => new Date()),
});

export type SelectTracker = typeof trackers.$inferSelect;
export type InsertTracker = typeof trackers.$inferInsert;
