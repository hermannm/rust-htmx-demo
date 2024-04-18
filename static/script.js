/**
 * // https://v1.htmx.org/events/#htmx:responseError
 * @typedef {{
 *   detail: {
 *     xhr: XMLHttpRequest
 *   }
 * }} HtmxResponseErrorEvent
 */

document.body.addEventListener(
  "htmx:responseError",
  (/** @type {HtmxResponseErrorEvent} */ event) => {
    const response = event.detail.xhr.responseText;
    console.error(`Server error: ${response}`);

    /** @type {HTMLTemplateElement} */
    const template = document.getElementById("error-popup");
    /** @type {DocumentFragment} */
    const errorFragment = template.content.cloneNode(true);
    /** @type {HTMLPreElement} */
    const content = errorFragment.getElementById("error-popup-content");
    /** @type {HTMLDivElement} */
    const container = document.getElementById("error-popups");
    /** @type {HTMLButtonElement} */
    const closer = errorFragment.getElementById("error-popup-closer");

    content.innerHTML = response;
    container.prepend(errorFragment);
    // Getting the Element here after adding it, since removing the Fragment doesn't work
    const errorElement = container.children[0];

    closer.addEventListener("click", () => {
      container.removeChild(errorElement);
    });
  }
);
