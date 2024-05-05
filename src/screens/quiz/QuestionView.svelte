<div id="question-screen">
    <div class="question-viewport">
        <div id="question-slider" class="question-slider"></div>
        <div class="navigation">
            <button id="prev-button" class="prev-button">&lt;</button>
            <button id="next-button" class="next-button">&gt;</button>
        </div>
    </div>
    <div id="question-sidebar">
        <button type="button" class="go-to-main-menu">Menu</button>
        <div class="question-meta-data-box">
            <div class="title-bar">
                <p class="round-label">Round: (number)</p>
            </div>
            <div class="question-meta-data" id="question-number">Question: Number</div>
            <div class="question-meta-data" id="question-category">Category: Memes</div>
            <div class="question-meta-data" id="question-price">Price: 300</div>
            <div class="question-meta-data" id="answer-status forbidden">
                Answer: forbidden
            </div>
        </div>
        <div id="controls">
            <button type="button" id="allow-answer-btn">Allow answer</button>
            <button type="button" id="correct-answer-btn">Correct answer</button>
            <button type="button" id="wrong-answer-btn">Wrong answer</button>
            <button type="button" id="show-answer-btn">Show answer and return</button>
        </div>
    </div>
</div>

<style>
    .question-meta-data-box > .title-bar {
        display: flex;
        flex-direction: column;
    }

    .question-meta-data-box > .title-bar > .round-label {
        font-size: 2em;
    }
</style>