<!--<script>-->
<!--    import {currentQuestionStore} from "../../../lib/stores.js";-->

<!--    $: question = $currentQuestionStore;-->
<!--</script>-->

<!--<div class="viewport">-->
<!--    <pre style="text-align: start">-->
<!--        {JSON.stringify(question, null, 2)}-->
<!--    </pre>-->
<!--    &lt;!&ndash;    <div class="slider"></div>&ndash;&gt;-->
<!--    &lt;!&ndash;    <div class="navigation">&ndash;&gt;-->
<!--    &lt;!&ndash;        <button id="prev-button" class="prev-button">&lt;</button>&ndash;&gt;-->
<!--    &lt;!&ndash;        <button id="next-button" class="next-button">&gt;</button>&ndash;&gt;-->
<!--    &lt;!&ndash;    </div>&ndash;&gt;-->
<!--</div>-->

<script>
    // import Swiper core and required modules
    import {A11y, Keyboard, Navigation, Pagination, Scrollbar} from 'swiper';
    import {currentQuestionStore} from "../../../lib/stores.js";

    import {Swiper, SwiperSlide} from 'swiper/svelte';

    // Import Swiper styles
    import 'swiper/css';
    import 'swiper/css/navigation';
    import 'swiper/css/pagination';
    import 'swiper/css/scrollbar';
    import 'swiper/css/keyboard';
    import Scenario from "./Scenario.svelte";

    $: question = $currentQuestionStore;
</script>

<div class="viewport">
    <Swiper
            modules={[Navigation, Pagination, Scrollbar, A11y, Keyboard]}
            slidesPerView={1}
            navigation={{enabled: true}}
            pagination={{ clickable: true }}
            scrollbar={{ draggable: true }}
            keyboard={{ enabled: true, pageUpDown: true}}
            on:slideChange={() => console.log('slide change')}
            on:swiper={(e) => console.log(e.detail[0])}
    >
        {#each question.scenario as scenario}
            <SwiperSlide>
                <Scenario {scenario}/>
            </SwiperSlide>
        {/each}
    </Swiper>
</div>


<style>
    .viewport1 {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        flex: 1;

        margin: 0.2em;
        height: 100%;
        background-color: rgba(181, 229, 172, 0.16);
        border: solid 3px var(--items-block-border-color);
        border-radius: inherit;
    }

    .viewport {
        display: flex;
        height: 66vh;
        width: 70vw;
    }

    .question-text {
        display: flex;
        justify-content: center;
        flex-wrap: wrap;
        flex: 1;
        text-align: center;

        padding: 1em;
        border-radius: 0.3em;

        box-sizing: border-box;

        font-weight: bold;
        font-size: 400%;
        line-height: 120%;
        font-family: sans-serif;

        background-color: var(--modal-table-background-color);
        color: var(--text-color);
    }

    .slide {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .slide img,
    .slide video {
        object-fit: contain;
        max-width: 100%;
        max-height: 100%;
        width: auto;
        height: auto;
    }
</style>