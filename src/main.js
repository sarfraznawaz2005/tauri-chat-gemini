document.addEventListener('DOMContentLoaded', () => {
    const invoke = window.__TAURI__.invoke;
    const chatbox = document.getElementById('chatbox');
    const userInput = document.getElementById('userInput');
    const sendButton = document.getElementById('sendButton');
    let conversationHistory = [];

    userInput.focus();

    function createMessageBubble(text, isUser, isError) {
        const messageDiv = document.createElement('div');
		isError = isError || false;
		
		if (isError) {
			messageDiv.classList.add('ai_error');
		}else {
			messageDiv.classList.add(isUser ? 'right-side' : 'left-side');
		}
        
        messageDiv.classList.add('message');
		messageDiv.textContent = text;
        chatbox.appendChild(messageDiv);
        chatbox.scrollTop = chatbox.scrollHeight;
		
		if (isError) {
			return;
		}

        if (isUser) {
            conversationHistory.push(`Human: ${text}`);
        } else {
            conversationHistory.push(`AI: ${text}`);
        }
    }

    async function aiResponse(userMessage) {
        try {
            userInput.disabled = true;
            sendButton.disabled = true;

            const loadingMessage = "Please wait...";
            createMessageBubble(loadingMessage, false);

            const historyString = conversationHistory.join('\n');
			
            const aiMessage = await invoke('get_ai_response', { prompt: userMessage + "\nConversation History:\n" + historyString });

            chatbox.removeChild(chatbox.lastChild); // remove loading message
            createMessageBubble(aiMessage, false);
        } catch (error) {
            createMessageBubble("Failed to get AI response. Please try again later." + error, false, true);
        } finally {
            userInput.disabled = false;
            sendButton.disabled = false;
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

	userInput.addEventListener('keydown', function (event) {
        if (event.key === 'Enter' && !event.shiftKey) {
            event.preventDefault();
			
            sendMessage();
        }
    });


    window.clearContent = function () {
        chatbox.innerHTML = '';
        conversationHistory = [];
    };
});
