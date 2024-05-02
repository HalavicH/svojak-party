<script>
    import BaseModal from "../../components/utils/BaseModal.svelte";
    import Button from "../../components/Button.svelte";
    import HSpacing from "../../components/utils/HSpacing.svelte";
    import VSpacing from "../../components/utils/VSpacing.svelte";
    export let isOpen;
</script>

<BaseModal {isOpen}>
    <h2>HUB Settings</h2>
    <div class="items-block">
        <p class="title">Choose HUB variant</p>
        <VSpacing size={"2em"}/>
        <div class="row">
            <Button text="Physical HUB"/>
            <HSpacing size={"5em"}/>
            <Button text="Web-based HUB"/>
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
