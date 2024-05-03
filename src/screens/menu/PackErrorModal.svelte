<script>
    import BaseModal from "../../components/utils/BaseModal.svelte";
    import Button from "../../components/Button.svelte";
    import ItemsBlock from "../../components/ItemsBlock.svelte";
    import Row from "../../components/Row.svelte";

    // Provided by 'modals'
    export let isOpen;
    export let title;
    export let message;
</script>

<BaseModal {isOpen}>
    <h2>Pack error</h2>
    <ItemsBlock title="Sema nuts">
        <div>{message}</div>
        <Row>
            <Button text="Ligma balls"/>
        </Row>
    </ItemsBlock>
</BaseModal>

<style>
    h2 {
        margin-top: 0;
    }

    p {
        margin: 5px 0px;
    }
</style>
