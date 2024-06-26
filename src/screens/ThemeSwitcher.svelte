<script>
    import { onMount } from 'svelte';
    import { writable } from 'svelte/store';
    import Switch from "../components/generic/Switch.svelte";

    let theme = 'default';
    $: {
        console.log('Toggling theme')
        if (theme === 'force dark') {
            console.log('Setting theme to dark')
            document.body.setAttribute('data-theme', 'dark');
        } else if (theme === 'default') {
            console.log('Setting theme to default')
            document.body.removeAttribute('data-theme');
        }
    }

    let isDarkMode;

    $: {
        console.log(`isDarkMode: ${isDarkMode}`)
        if (isDarkMode === 'on') {
            document.body.setAttribute('data-theme', 'dark');
        } else {
            document.body.removeAttribute('data-theme');
        }
    }

    // Set the initial theme based on the prefers-color-scheme
    // onMount(() => {
    //     if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    //         isDarkMode.set(true);
    //         document.body.setAttribute('data-theme', 'dark');
    //     }
    // });
</script>


<!-- Theme Switcher Checkbox -->
<label class="theme-switch">
    <Switch bind:value={isDarkMode} label="Force dark theme" design="slider" fontSize={12}/>
</label>

<style>
    .theme-switch {
        position: absolute;
        top: 1.7em;
        left: 2em;
        display: flex;
        align-items: center;
        background-color: inherit;
        padding: 0.5em;
        border-radius: 0.5em;
    }
    .theme-switch input {
        margin-right: 0.5em;
    }
</style>