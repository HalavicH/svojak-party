// Views
let registerView = document.querySelector('#register');
let collisionView = document.querySelector('#collision');
let unauthorizedView = document.querySelector('#unauthorized');
let gameView = document.querySelector('#content');

// Buttons
let submitPlayer = document.querySelector('#submit-player');
let answerButton = document.querySelector('#answer');

// Inputs
let playerNameInput = document.querySelector('#name');

// Info
let ipDiv = document.querySelector('#ip-address');
let playerIdDiv = document.querySelector('#player-id');
let playerNameDiv = document.querySelector('#player-name');
let playerStatusDiv = document.querySelector('#player-status');

STATE = {
    playerId: undefined,
    name: undefined,
    ip: undefined,
}

async function registerPlayer() {
    const playerName = playerNameInput.value;

    if (playerName === "") {
        console.log("Empty input");
        return;
    }
    console.log("Name: " + playerName);

    const responseIp = await fetch("/ip-loopback", {
        method: "GET"
    });

    if (!responseIp.ok) {
        console.log("Can't receive IP");
        return;
    }
    const ipObj = await responseIp.json();
    const ipAddr = ipObj.ip;

    let body = JSON.stringify({
        id: 0, name: playerName, ip: ipAddr,
    });

    let params = {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: body,
    };

    let response = undefined;
    try {
        response = await fetch("/register", params)
        if (!response.ok) {
            console.error("Failed to register player");
            throw Error("Conflict");
        }

    } catch (e) {
        registerView.style.display = "none";
        collisionView.style.display = 'flex';
    }

    const {id, name, ip} = await response.json();

    // Store the playerId and baseTimestamp in your application as needed
    STATE.playerId = id;
    STATE.name = name;
    STATE.ip = ip;

    console.log("Player state: " + STATE);

    ipDiv.innerText = ipAddr + " |";
    playerIdDiv.innerText = id + " |";
    playerNameDiv.innerText = name;

    // Disable the register screen and enable the content screen
    registerView.style.display = "none";
    gameView.style.display = "flex";
}

async function sendEvent(buttonState) {
    const eventData = {
        id: STATE.playerId,
        ip: STATE.ip,
        state: buttonState,
        timestamp: 0,
    };

    let response = undefined;
    try {
        response = await fetch("/event", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(eventData),
        });

        if (!response.ok) {
            console.error("Failed to send event");
            throw new Error("Bad bad error. Idk It's 4:30 am");
        }
    } catch (e) {
        console.log("Can't send event");
        gameView.style.display = "none";
        unauthorizedView.style.display = 'flex';
    }


    response.json()
        .then(value => {
            console.log(value);

            playerStatusDiv.style.color = value.color;
        })
}


document.addEventListener("DOMContentLoaded", () => {
    collisionView.style.display = 'none';
    unauthorizedView.style.display = 'none';
    gameView.style.display = 'none';

    submitPlayer.addEventListener("click", registerPlayer);
    answerButton.addEventListener("mousedown", () => {
        sendEvent(true); // Button is pressed
    });
    answerButton.addEventListener("mouseup", () => {
        sendEvent(false); // Button is pressed
    });
})

