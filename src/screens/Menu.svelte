<script>
    import {closeModal, openModal} from 'svelte-modals'
    import Button from "../components/Button.svelte";
    import ModalPlaceholder from "../components/utils/ModalPlaceholder.svelte";
    import SettingsModal from "./menu/SettingsModal.svelte";
    import GamePackModal from "./menu/GamePackModal.svelte";
    import {open} from "@tauri-apps/api/dialog";
    import {notify} from "../lib/notifications.js";
    import {invoke} from "@tauri-apps/api/tauri";
    import {TauriApiCommand} from "../lib/commands.js";
    import PackErrorModal from "./menu/PackErrorModal.svelte";

    function openSetup() {
        openModal(SettingsModal)
    }

    async function openGamePack() {
        let filePath = await open({
            multiple: false,
            filters: [{
                name: 'Select game package',
                extensions: ['siq']
            }]
        })

        if (filePath === null || filePath.length === 0) {
            notify.info("Canceled pack selection");
            return;
            // closeModal();
        } else {
            notify.info(`Selected game package path: ${filePath}`);
        }

        invoke(TauriApiCommand.GET_PACK_INFO, {path: filePath})
            .then((newPackInfo) => {
                openModal(GamePackModal, {packInfo: newPackInfo})
            })
            .catch((error) => {
                console.error("Promise rejection:", error);
                // Log the rejection payload or handle the error in any other way
                closeModal();
                // openPackErrorModel(error);
                openModal(PackErrorModal, {message: error});
            });
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
            <Button text="Open game pack" onClick={openGamePack}/>
            <p></p>
            <Button text="Debug menu" onClick={openSetup}/>
        </div>
    </div>
</div>