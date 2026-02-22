/**
 * Rusty-Board Lean v1
 *
 * Developer Notes:
 * - This file contains optional client-side enhancements.
 * - The application does NOT depend on this file.
 * - It is safe to remove without breaking functionality.
 *
 * TODO:
 * - Expand with future UX improvements if desired.
 *
 * End Notes:
 * Keep JavaScript minimal in Lean v1.
 */

document.addEventListener("DOMContentLoaded", function () {
    console.log("Rusty-Board JS loaded.");

    // Example: confirm before submitting forms
    var forms = document.querySelectorAll("form");

    forms.forEach(function (form) {
        form.addEventListener("submit", function (event) {
            var confirmed = true;

            if (form.action.includes("/threads")) {
                confirmed = confirm("Create this thread?");
            }

            if (form.action.includes("/posts")) {
                confirmed = confirm("Post this reply?");
            }

            if (!confirmed) {
                event.preventDefault();
            }
        });
    });
});