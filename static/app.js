const joinDiv = document.getElementById("join");
const chatDiv = document.getElementById("chat");

function JoinChat() {
    const username = document.getElementById('username').value;
    if (username) {
        ws.send(JSON.stringify({ type: 'join', username }));
    }
    chatDiv.style.display = 'block';
    joinDiv.style.display = 'none';
}

function sendMessage() {
    const messageInput = document.getElementById('message');
    const message = messageInput.value;
    if (message) {
        ws.send(JSON.stringify({ type: 'chat', text: message }));
        messageInput.value = '';
    }
}   

const ws = new WebSocket(`ws://${location.host}/ws`);

ws.onopen = () => {
    console.log('WebSocket connection established');
}

ws.onmessage = (event) => {
    console.log('Message received:', event.data);
}

ws.onclose = () => {
    console.log('WebSocket connection closed');
}

ws.onerror = (error) => {
    console.error('WebSocket error:', error);
}


ws.onmessage = (event) => {
    const msg = JSON.parse(event.data);
    switch (msg.type) {
        case "chat":
            console.log(`${msg.username}: ${msg.text}`);
            break;
        case "userJoined":
            console.log(`${msg.username} kom inn`);
            break;
        case "userLeft":
            console.log(`${msg.username} dro`);
            break;
            default:
    console.warn("ukjent meldingstype:", msg);
    }
}
