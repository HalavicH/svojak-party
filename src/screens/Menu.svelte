<script>
    import { openModal } from 'svelte-modals'
    import Button from "../components/Button.svelte";
    import ModalPlaceholder from "../components/utils/ModalPlaceholder.svelte";
    import SettingsModal from "./menu/SettingsModal.svelte";

    function openSetup() {
        openModal(SettingsModal, { title: 'Alert', message: 'This is an alert' })
    }
</script>


<div>
    <ModalPlaceholder/>
    <h1>Welcome to Svojak!</h1>
    <p>Powered by BronuCon commuity</p>
    <div class="row">
<!--        <a href="https://bronucon.in.ua" target="_blank">-->
            <img
                    src="public/bc-logo.png"
                    class="logo bronucon"
                    alt="BronuCon logo"
            />
<!--        </a>-->
    </div>
<!--    <p>Clicked  {count} {count === 1 ? 'time' : 'times'}</p>-->

    <div class="row">
        <div>
<!--            <Modal isVisible={settingsVisible}/>-->
            <Button text="Check setup (HW & Players)" onClick={openSetup}/>
            <p>then</p>
            <Button text="Open pack" onClick={openSetup}/>
            <p></p>
            <Button text="Debug menu" onClick={openSetup}/>
        </div>
    </div>
</div>