const invoke = window.__TAURI__.invoke;

document.addEventListener('DOMContentLoaded', () => {
    const chatbox = document.getElementById('chatbox');
    const userInput = document.getElementById('userInput');
    const sendButton = document.getElementById('sendButton');

    function createMessageBubble(text, isUser) {
        const messageDiv = document.createElement('div');
        messageDiv.classList.add('message');
        messageDiv.classList.add(isUser ? 'right-side' : 'left-side');
        messageDiv.textContent = text;
        chatbox.appendChild(messageDiv);
        chatbox.scrollTop = chatbox.scrollHeight;
    }

    async function aiResponse(userMessage) {
        try {
            const aiMessage = await invoke('get_ai_response', { prompt: userMessage });
            createMessageBubble(aiMessage, false);
        } catch (error) {
            createMessageBubble("Failed to get AI response. Please try again later." + error, false);
        }
    }

    async function sendMessage() {
        const userInputField = document.getElementById('userInput');
        const userMessage = userInputField.value.trim();
		
        if (userMessage) {
            createMessageBubble(userMessage, true);
            await aiResponse(userMessage);
            userInputField.value = '';
        }
    }

    sendButton.addEventListener('click', sendMessage);

    userInput.addEventListener('keypress', function(event) {
        if (event.key === 'Enter') {
            event.preventDefault();
            sendMessage();
        }
    });

    window.clearContent = function() {
        chatbox.innerHTML = '';
    };
});
