<script>
    import BaseModal from "../../components/abstract/BaseModal.svelte";
    import Button from "../../components/generic/Button.svelte";
    import ItemsBlock from "../../components/generic/ItemsBlock.svelte";
    import Row from "../../components/generic/Row.svelte";
    import VSpacing from "../../components/generic/VSpacing.svelte";

    // Provided by 'modals'
    export let isOpen;
    export let message;
    console.log(message);
</script>

<BaseModal {isOpen}>
    <h2>{message.cause}</h2>
    <ItemsBlock title={message.path}>
        <div class="json-view">
            <pre> {@html message.details} </pre>
        </div>
        <VSpacing size="0.5em"/>
        <Row>
            <Button text="Gotcha... Well, no game for today"/>
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
