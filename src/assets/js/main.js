document.addEventListener('DOMContentLoaded', () => {
    const invoke = window.__TAURI__.invoke;
	const md = window.markdownit();
	
    const chatbox = document.getElementById('chatbox');
    const userInput = document.getElementById('userInput');
	const loadingMessageText = "Please wait...";

	let lastUserMessage = '';
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
		
		if (!isUser && !isError) { // if it's from AI
			const htmlContent = md.render(text);
			messageDiv.innerHTML = htmlContent;
		} else {
			messageDiv.textContent = text;
		}		
		
        chatbox.appendChild(messageDiv);
        chatbox.scrollTop = chatbox.scrollHeight;
		
		if (isError || text === loadingMessageText) {
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

            createMessageBubble(loadingMessageText, false);

            const historyString = conversationHistory.join('\n');
			
            const aiMessage = await invoke('get_ai_response', { prompt: userMessage + "\nConversation History:\n" + historyString });

			if (aiMessage.includes(loadingMessageText)) {
				createMessageBubble(aiMessage, false, true);
			} else {
				createMessageBubble(aiMessage, false);
			}
            
        } catch (error) {
            createMessageBubble("Failed to get AI response. Please try again later." + error.replace(/https:\/\/.*?\s/, ''), false, true);
        } finally {
			removeLoadingMessage();
            userInput.disabled = false;
			userInput.focus();
        }
    }

    async function sendMessage() {
        const userInputField = document.getElementById('userInput');
        const userMessage = userInputField.value.trim();

        if (userMessage) {
			lastUserMessage = userMessage;
            createMessageBubble(userMessage, true);
            await aiResponse(userMessage);
            userInputField.value = '';
        }
    }

	function removeLoadingMessage() {
		for (let i = chatbox.children.length - 1; i >= 0; i--) {
			const child = chatbox.children[i];
			if (child.textContent.includes(loadingMessageText)) {
				chatbox.removeChild(child);
				break;
			}
		}
	}
	
	userInput.addEventListener('keydown', function (event) {
        if (event.key === 'Enter' && !event.shiftKey) {
            event.preventDefault();
            sendMessage();
        } else if (event.key === 'ArrowUp') {
            event.preventDefault();
            document.getElementById('userInput').value = lastUserMessage;

            const input = document.getElementById('userInput');
            input.focus();
            input.setSelectionRange(lastUserMessage.length, lastUserMessage.length);
        }
    });


    window.clearContent = function () {
        chatbox.innerHTML = '';
        conversationHistory = [];
    };
	
});
