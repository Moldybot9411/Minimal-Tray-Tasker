import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";

export enum Settings {
    autostart = "autostart",
    reminders = "reminders",
}

export type SettingsObject = {
    [key in Settings]?: boolean;
};

export const SettingsService = {
    async refresh(): Promise<void> {
        await invoke<SettingsObject>("get_settings").then((result) => {
            settings.set(result);
        });
    },

    async setAutostart(value: boolean): Promise<void> {
        settings.update((current) => {
            current[Settings.autostart] = value;
            return current;
        });

        await invoke("set_autostart", { value: value });
    },

    async setReminders(value: boolean): Promise<void> {
        settings.update((current) => {
            current[Settings.reminders] = value;
            return current;
        });

        await invoke("set_reminders", { value: value });
    },
};

export const settings = writable<SettingsObject>({
    [Settings.autostart]: true,
    [Settings.reminders]: true,
});
