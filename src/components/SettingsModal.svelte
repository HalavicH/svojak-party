<script>
    import BaseModal from "./BaseModal.svelte";
    export let isOpen;
</script>

<BaseModal {isOpen}>
    <h2>HUB Settings</h2>
    <div class="items-block">
        <p class="title">Choose HUB variant</p>
        <div class="row">
            <button id="hw-hub-btn">Physical HUB</button>
            <button id="web-hub-btn">Web-based HUB</button>
        </div>
    </div>
</BaseModal>

<style>
    h2 {
        margin-top: 0;
    }

    p {
        margin: 5px 0px;
    }

    .row {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }
</style>
