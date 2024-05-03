<script>
    export let hubStatus = "Undefined";
    let statusStyle = "";
    $: {
        statusStyle = hubStatus === "Serial Port Error" ? "serial-port-error" : "";
        statusStyle = hubStatus === "Unknown Device" ? "unknown-device" : "";
        statusStyle = hubStatus === "Detected" ? "detected" : "";
    }
</script>

<div class="hub-status {statusStyle}">{hubStatus}</div>

<style>
    /************ Statuses ************/
    .hub-status {
        border: 1px solid black;
        text-align: center;
        display: inline-block;
        padding: 0 4px;
        margin: 5px 0;
        border-radius: 5px;
        font-size: 12px;
        font-weight: bolder;
        width: 100px;
    }

    .hub-status.serial-port-error {
        color: #ff0000;
        background: rgba(255, 0, 0, 0.1);
        border: 1px solid #ff0000;
        box-shadow: 0 0 0.5em #ff0000;
        transition: box-shadow 0.5s;
    }

    .hub-status.unknown-device {
        color: #8a8a8a;
        background: rgba(163, 147, 3, 0.1);
        border: 1px solid #818181;
    }

    .hub-status.detected {
        color: #52aa65;
        background: rgba(82, 170, 101, 0.1);
        border: 1px solid #52aa65;
        box-shadow: 0 0 0.5em #58db24;
        transition: box-shadow 0.5s;
    }
</style>