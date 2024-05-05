<script>
    import BaseModal from "../../components/abstract/BaseModal.svelte";
    import Button from "../../components/generic/Button.svelte";
    import ItemsBlock from "../../components/generic/ItemsBlock.svelte";
    import Row from "../../components/generic/Row.svelte";

    // Provided by 'modals'
    export let isOpen;
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
