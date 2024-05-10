# Architecture

## Proper (?)

...

## Clues

### Arch

Yes, in the current setup, the string would be processed twice: once by the backend to identify the symbols and create the tokens, and once by the frontend to apply the formatting based on the tokens.

However, this is a common pattern in web development. The backend is responsible for processing the raw data and transforming it into a format that's easy for the frontend to use. The frontend is then responsible for taking that data and rendering it in a way that's meaningful to the user.

In this case, the backend is identifying the special symbols and creating a list of tokens that the frontend can easily iterate over. The frontend is then applying the formatting based on the tokens. This separation of concerns allows each part of the application to focus on what it does best.

If you're concerned about performance, you could consider optimizing the backend processing. For example, instead of creating a new string for each character in the input string, you could iterate over the input string by index and use slicing to get the current character. This would avoid the overhead of creating a new string for each character. However, this is a micro-optimization and is unlikely to have a significant impact on performance unless you're processing very large strings.

### Realtime

```javascript
// Get the text input element
let textInput = document.querySelector('#your-text-input-id');

// Add an event listener for the 'input' event
textInput.addEventListener('input', (event) => {
    // Get the current text
    let text = event.target.value;

    // Parse the text
    let tokens = parseText(text);

    // Do something with the tokens...
});
```
