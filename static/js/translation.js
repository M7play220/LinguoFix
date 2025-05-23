document.addEventListener('DOMContentLoaded', function() {
  // Get elements relevant ONLY to the translation page
  const sourceLanguageSelect = document.getElementById('source-language');
  const targetLanguageSelect = document.getElementById('target-language');
  const sourceTextarea = document.getElementById('source-text');
  const targetTextarea = document.getElementById('target-text');
  const translateButton = document.getElementById('translate-button');

  // Check if all necessary elements for translation exist
  if (!sourceLanguageSelect || !targetLanguageSelect || !sourceTextarea || !targetTextarea || !translateButton) {
    return;
  }

  // Function for auto-resizing textareas
  function autoResizeTextarea(event) {
    const textarea = event.target;
    const minHeight = 550;
    textarea.style.height = 'auto';
    const scrollHeight = textarea.scrollHeight;
    const newHeight = Math.max(scrollHeight, minHeight);
    textarea.style.height = newHeight + 'px';

    let otherTextarea = null;
    if (textarea === sourceTextarea) {
      otherTextarea = targetTextarea;
    } else if (textarea === targetTextarea) {
      otherTextarea = sourceTextarea;
    }

    if (otherTextarea) {
      otherTextarea.style.height = 'auto';
      const otherScrollHeight = otherTextarea.scrollHeight;
      otherTextarea.style.height = Math.max(otherScrollHeight, newHeight, minHeight) + 'px';
    }
  }

  // --- Функція для відправки запиту на переклад ---
  async function sendTranslationRequest(sourceText, sourceLanguage, targetLanguage) {
    try {
      const response = await fetch('/api/translate', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          text: sourceText,
          source_language: sourceLanguage,
          target_language: targetLanguage
        })
      });

      const body = await response.text(); // Отримуємо тіло як текст
      console.log("Response status:", response.status);
      console.log("Response body:", body);

      if (!response.ok) {
        console.error("Помилка при відправці запиту на переклад.");
        alert("Помилка при отриманні перекладу."); // Повідомлення користувачеві про помилку
        return null;
      }

      try {
        const data = JSON.parse(body); // Парсимо JSON, якщо запит успішний
        return data;
      } catch (error) {
        console.error("Помилка парсингу JSON:", error);
        alert("Помилка обробки відповіді сервера."); // Повідомлення про помилку парсингу
        return null;
      }
    } catch (error) {
      console.error("Помилка мережі при запиті:", error);
      alert("Помилка мережі."); // Повідомлення про помилку мережі
      return null;
    }
  }


  // Add input event listeners for auto-resizing
  sourceTextarea.addEventListener('input', autoResizeTextarea);
  targetTextarea.addEventListener('input', autoResizeTextarea);

  // Event listener for the translate button
  translateButton.addEventListener('click', async function() {
    const sourceText = sourceTextarea.value;
    const sourceLanguage = sourceLanguageSelect.value;
    const targetLanguage = targetLanguageSelect.value;

    const responseData = await sendTranslationRequest(sourceText, sourceLanguage, targetLanguage);

    if (responseData && responseData.translated_text !== undefined) {
      targetTextarea.value = responseData.translated_text;
      autoResizeTextarea({ target: targetTextarea });
    } else if (responseData !== null) {
      // Якщо responseData не null, але translated_text відсутній,
      // спробуємо відобразити безпосередньо (припускаючи, що сервер повертає просто текст)
      targetTextarea.value = responseData;
      autoResizeTextarea({ target: targetTextarea });
    } else {
      // responseData є null, отже сталася помилка (про це вже повідомлено)
      console.log("Не вдалося отримати або обробити переклад.");
    }
  });
});