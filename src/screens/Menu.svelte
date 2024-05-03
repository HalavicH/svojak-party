<script>
    import { openModal } from 'svelte-modals'
    import Button from "../components/Button.svelte";
    import ModalPlaceholder from "../components/utils/ModalPlaceholder.svelte";
    import SettingsModal from "./menu/SettingsModal.svelte";
    import GamePackModal from "./menu/GamePackModal.svelte";

    function openSetup() {
        openModal(SettingsModal, { title: 'Alert', message: 'This is an alert' })
    }

    function openGamePack() {
        openModal(GamePackModal, { title: 'Alert', message: 'This is an alert' })
    }
</script>


<div>
    <ModalPlaceholder/>
    <h1>Welcome to Svojak!</h1>
    <p>Powered by BronuCon commuity</p>
    <div class="row">
            <img
                    src="public/bc-logo.png"
                    class="logo bronucon"
                    alt="BronuCon logo"
            />
    </div>

    <div class="row">
        <div>
            <Button text="Check setup (HW & Players)" onClick={openSetup}/>
            <p>then</p>
            <Button text="Open pack" onClick={openGamePack}/>
            <p></p>
            <Button text="Debug menu" onClick={openSetup}/>
        </div>
    </div>
</div>