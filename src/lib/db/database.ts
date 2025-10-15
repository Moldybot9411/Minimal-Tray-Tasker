import { drizzle } from "drizzle-orm/sqlite-proxy";
import Database from "@tauri-apps/plugin-sql";
import * as schema from "./schema";

async function getDb(): Promise<Database> {
    return Database.load("sqlite:appdb.sqlite");
}

function isSelectQuery(sql: string): boolean {
    const trimmedSql = sql.trimStart().toUpperCase();
    return trimmedSql.startsWith("SELECT");
}

export const db = drizzle<typeof schema>(
    async (sql, params, method) => {
        const sqlite = await getDb();
        let rows: any = [];
        let results = [];

        if (isSelectQuery(sql)) {
            rows = await sqlite.select(sql, params).catch((e) => {
                console.error("SQL Error:", e);
                return [];
            });
        } else {
            const result = await sqlite.execute(sql, params).catch((e) => {
                console.error("SQL Error:", e);
                return [];
            });
            return { rows: [{ result }] };
        }

        rows = rows.map((row: any) => {
            return Object.values(row);
        });

        results = method === "all" ? rows : rows[0];
        await sqlite.close();
        return { rows: results };
    },
    {
        schema: schema,
        logger: true,
    },
);
