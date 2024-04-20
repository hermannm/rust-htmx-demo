document.body.addEventListener("htmx:responseError", (event) => {
    const response = event.detail.xhr.responseText;
    console.error(`Server error: ${response}`);
    const template = document.getElementById("error-popup");
    const errorFragment = template.content.cloneNode(true);
    const content = errorFragment.getElementById("error-popup-content");
    const container = document.getElementById("error-popups");
    const closer = errorFragment.getElementById("error-popup-closer");
    content.innerHTML = response;
    container.prepend(errorFragment);
    const errorElement = container.children[0];
    closer.addEventListener("click", () => {
        container.removeChild(errorElement);
    });
});
