<script>
    import {Modals, closeModal} from 'svelte-modals'
    import {doWithSound, getWhooshSound} from "../../lib/sound.js";

    function handleCloseModal() {
        doWithSound(closeModal, getWhooshSound());
    }

</script>

<Modals>
    <div
            slot="backdrop"
            class="backdrop"
            on:click={handleCloseModal}
    />
</Modals>

<style>
    .backdrop {
        position: fixed;
        top: 0;
        bottom: 0;
        right: 0;
        left: 0;
        background: rgba(0, 0, 0, 0.50)
    }
</style>