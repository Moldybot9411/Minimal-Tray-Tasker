<script lang="ts">
    import type { HTMLInputAttributes } from "svelte/elements";

    interface Props extends Omit<HTMLInputAttributes, "type" | "class"> {
        name: string;
        label?: string;
        checked?: boolean;
        class?: string;
    }

    let {
        name,
        label,
        checked = $bindable(false),
        class: classes,
        ...others
    }: Props = $props();
</script>

<div class="flex items-center gap-2 {classes}">
    <label for={name} class="relative cursor-pointer">
        <input
            bind:checked
            {...others}
            type="checkbox"
            {name}
            id={name}
            class="toggle-input" />
        <span class="switch shadow-md"></span>
    </label>

    {#if label}
        <p>{label}</p>
    {/if}
</div>

<style>
    .toggle-input {
        position: absolute;
        opacity: 0;
        width: 100%;
        height: 100%;
        cursor: pointer;
    }

    .switch {
        pointer-events: none;
        display: flex;
        align-items: center;
        width: 2.5rem;
        height: 1.3rem;
        border: none;
        border-radius: 1rem;
        background-color: rgb(149, 149, 149);
        transition: all ease-in-out 300ms;
    }

    .switch::before {
        content: "";
        position: absolute;
        width: 1rem;
        height: 1rem;
        left: 0.15rem;
        background-color: white;
        border-radius: 50%;
        transition: all ease-in-out 200ms;
    }

    .toggle-input:focus-visible + .switch {
        outline: 2px solid black;
    }

    .toggle-input:checked + .switch {
        background-color: var(--color-indigo-400);
    }

    .toggle-input:checked + .switch::before {
        transform: translateX(1.15rem);
    }
</style>
