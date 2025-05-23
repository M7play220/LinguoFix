document.addEventListener('DOMContentLoaded', function() {
    // Отримуємо елементи
    const initialCorrectionForm = document.getElementById('initial-correction-form');
    const correctionForm = document.getElementById('text-form');
    const textInput = document.getElementById('text-input');
    
    const languageSelectorContainer = document.getElementById('language-selector-container'); // Контейнер селектора мови
    const correctionLanguageSelect = document.getElementById('correction-language'); 
    
    const editingCorrectionForm = document.getElementById('editing-correction-form');
    const originalEditableText = document.getElementById('original-editable-text');
    const correctedEditableText = document.getElementById('corrected-editable-text');
    const recheckButton = document.getElementById('recheck-button');

    const originalTextAreaDiv = document.querySelector('#editing-correction-form .original-text-area');

    if (!initialCorrectionForm || !correctionForm || !textInput || !languageSelectorContainer || !correctionLanguageSelect ||
        !editingCorrectionForm || !originalEditableText || !correctedEditableText || !recheckButton || !originalTextAreaDiv) { 
        console.error("Correction Page Error: Не знайдено один або більше необхідних HTML елементів! Перевірте DOM, включаючи 'language-selector-container' та '.original-text-area'.");
        return;
    }

    function autoResizeTextarea(textarea) {
        if (!textarea) return;
        const minHeight = 120; 
        textarea.style.height = 'auto'; 
        const scrollHeight = textarea.scrollHeight; 
        textarea.style.height = Math.max(scrollHeight, minHeight) + 'px'; 
    }

    textInput.addEventListener('input', () => {
        autoResizeTextarea(textInput);
    });
    autoResizeTextarea(textInput);

    // --- Функція для відправки запиту на корекцію ---
    async function sendCorrectionRequest(text, language) {
        const token = localStorage.getItem('authToken'); 
        try {
            const response = await fetch('/api/correct', { // URL API для корекції
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    // Додаємо токен авторизації, якщо він існує
                    ...(token && { 'Authorization': `Bearer ${token}` })
                },
                body: JSON.stringify({ text: text, language: language })
            });

            if (!response.ok) {
                let errorData = null;
                try {
                    errorData = await response.json();
                } catch (e) {
                    errorData = await response.text();
                }
                console.error(`Помилка при відправці запиту на корекцію (Status: ${response.status}):`, errorData);
                throw new Error(errorData.error || `Сервер повернув помилку: ${response.status}`);
            }

            const data = await response.json();
            return data;
        } catch (error) {
            console.error("Помилка мережі або обробки відповіді при відправці запиту на корекцію:", error);
            alert(`Помилка при отриманні виправлення: ${error.message || "Невідома помилка"}`);
            return null;
        }
    }

    // Обробник події 'submit' для початкової форми
    correctionForm.addEventListener('submit', async function(event) {
        event.preventDefault(); 
        const inputText = textInput.value.trim(); 
        const selectedLanguage = correctionLanguageSelect.value;

        if (!inputText) {
            alert("Будь ласка, введіть текст для перевірки.");
            return;
        }

        // Робимо реальний запит до сервера
        const responseData = await sendCorrectionRequest(inputText, selectedLanguage);

        if (responseData && responseData.corrected_text !== undefined) { 
            initialCorrectionForm.style.display = 'none'; 
            editingCorrectionForm.style.display = 'flex'; 
            originalEditableText.value = inputText; 
            correctedEditableText.value = responseData.corrected_text; 

            if (originalTextAreaDiv && languageSelectorContainer) {
                originalTextAreaDiv.after(languageSelectorContainer); 
            } else {
                console.error("Не вдалося перемістити селектор мови: один з елементів не знайдено.");
            }

            autoResizeTextarea(originalEditableText);
            autoResizeTextarea(correctedEditableText);
            if (originalEditableText.style.height) {
                 correctedEditableText.style.height = originalEditableText.style.height;
            }
        }
        // Помилка обробляється всередині sendCorrectionRequest через alert
    });

    originalEditableText.addEventListener('input', () => {
        autoResizeTextarea(originalEditableText);
        if (originalEditableText.style.height) {
            correctedEditableText.style.height = originalEditableText.style.height; 
        }
    });
    autoResizeTextarea(originalEditableText);
    autoResizeTextarea(correctedEditableText);

    // Обробник для кнопки "Перевірити ще раз"
    recheckButton.addEventListener('click', async function() {
        const currentOriginalText = originalEditableText.value.trim();
        const selectedLanguage = correctionLanguageSelect.value;

        if (!currentOriginalText) {
            alert("Будь ласка, введіть текст для повторної перевірки.");
            return;
        }
        
        // Робимо реальний запит до сервера
        const responseData = await sendCorrectionRequest(currentOriginalText, selectedLanguage);

        if (responseData && responseData.corrected_text !== undefined) {
            correctedEditableText.value = responseData.corrected_text;
            autoResizeTextarea(originalEditableText); 
            autoResizeTextarea(correctedEditableText); 
            if (originalEditableText.style.height) {
                correctedEditableText.style.height = originalEditableText.style.height;
            }
        }
        // Помилка обробляється всередині sendCorrectionRequest через alert
    });
});