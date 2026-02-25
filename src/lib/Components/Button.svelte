<script lang="ts">
    import type { Snippet } from "svelte";
    import { type Icon as IconType } from "@lucide/svelte";
    import type { HTMLButtonAttributes } from "svelte/elements";

    interface Props extends HTMLButtonAttributes {
        children?: Snippet;
        label?: string;
        icon?: typeof IconType;
        title?: string;
        variant?: "primary" | "secondary" | "accept" | "warning" | "danger";
        class?: string;
    }

    let {
        children,
        label,
        icon,
        title,
        variant = "primary",
        class: classes,
        ...others
    }: Props = $props();
</script>

<button
    class={[
        `p-2 rounded-md font-bold flex items-center justify-center gap-2 cursor-pointer shadow-md ${classes}`,
        variant === "primary" &&
            "bg-indigo-400 hover:bg-indigo-500 dark:bg-blue-500 dark:hover:bg-blue-600",
        variant === "secondary" &&
            "bg-gray-300 hover:bg-gray-400 dark:bg-gray-700 dark:hover:bg-gray-800",
        variant === "accept" &&
            "bg-lime-400 hover:bg-lime-500 dark:bg-green-500 dark:hover:bg-green-600",
        variant === "warning" &&
            "bg-yellow-500 hover:bg-yellow-600 dark:bg-amber-400 darK:hover:bg-amber-500",
        variant === "danger" && "bg-red-500 hover:bg-red-600",
    ]}
    {title}
    {...others}>
    {#if icon}
        {@const Icon = icon}
        <Icon />
    {/if}

    {label ? label : ""}
</button>
