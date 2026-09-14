const joinDiv = document.getElementById("join");
const chatDiv = document.getElementById("chat");

function JoinChat() {
  const username = document.getElementById("username").value.trim();
  const room = document.getElementById("room").value.trim();
  if (username && room) {
    ws.send(JSON.stringify({ type: "join", username, room }));
    chatDiv.style.display = "block";
    joinDiv.style.display = "none";
  } else {
    alert("Fyll inn både brukernavn og rom.");
  }
}

function sendMessage() {
  const messageInput = document.getElementById("message");
  const message = messageInput.value;
  if (message) {
    ws.send(JSON.stringify({ type: "chat", text: message }));
    messageInput.value = "";
  }
}

function addLine(text) {
  const box = document.getElementById("messages");
  const div = document.createElement("div");
  const room = document.getElementById("room").value;
  div.textContent = text;
  box.appendChild(div);
  box.scrollTop = box.scrollHeight;
}

const ws = new WebSocket(`ws://${location.host}/ws`);

ws.onopen = () => {
  console.log("WebSocket connection established");
};

ws.onclose = () => {
  console.log("WebSocket connection closed");
};

ws.onerror = (error) => {
  console.error("WebSocket error:", error);
};

ws.onmessage = (event) => {
  const msg = JSON.parse(event.data);
  switch (msg.type) {
    case "history":
      msg.messages.forEach((m) => addLine(`${m.username}: ${m.text}`));
      break;
    case "chat":
      addLine(`${msg.username}: ${msg.text}`);
      break;
    case "userJoined":
      addLine(`${msg.username} kom inn`);
      break;
    case "userLeft":
      addLine(`${msg.username} dro`);
      break;
    default:
      console.warn("ukjent meldingstype:", msg);
      break;
  }
};
