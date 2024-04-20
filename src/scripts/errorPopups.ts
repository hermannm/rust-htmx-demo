// Displays server errors in a popup in the bottom right corner
document.body.addEventListener("htmx:responseError", (event) => {
  const response = event.detail.xhr.responseText;
  console.error(`Server error: ${response}`);

  const template = document.getElementById("error-popup") as HTMLTemplateElement;
  const errorFragment = template.content.cloneNode(true) as DocumentFragment;
  const content = errorFragment.getElementById("error-popup-content");
  const container = document.getElementById("error-popups");
  const closer = errorFragment.getElementById("error-popup-closer");

  content.innerHTML = response;
  container.prepend(errorFragment);
  // Getting the Element here after adding it, since removing the Fragment doesn't work
  const errorElement = container.children[0];

  closer.addEventListener("click", () => {
    container.removeChild(errorElement);
  });
});
