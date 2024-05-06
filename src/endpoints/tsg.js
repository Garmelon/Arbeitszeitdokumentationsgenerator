const form = document.getElementById("form");
const submit = document.getElementById("submit");
const info = document.getElementById("info");

function showStatus(msg) {
  info.classList.remove("success");
  info.classList.remove("error");
  info.textContent = msg;
  info.scrollIntoView();
}

function showSuccess(msg) {
  info.classList.add("success");
  info.classList.remove("error");
  info.textContent = msg;
  info.scrollIntoView();
}

function showError(msg) {
  info.classList.add("error");
  info.classList.remove("success");
  info.textContent = msg;
  info.scrollIntoView();
}

submit.addEventListener("click", async () => {
  const data = new FormData(form);
  const dataJson = JSON.stringify({
    global: JSON.parse(data.get("global")),
    month: JSON.parse(data.get("month")),
  });

  try {
    showStatus("Generiere...");

    const response = await fetch(".", {
      method: "post",
      headers: { "Content-Type": "application/json" },
      body: dataJson,
    });

    if (response.status !== 200) {
      const reason = await response.text();
      showError(`Generieren fehlgeschlagen:\n${reason}`);
      return;
    }

    let blob = await response.blob();

    const reader = new FileReader();
    reader.addEventListener("loadend", () => {
      let element = document.createElement("a");
      element.setAttribute("href", reader.result);
      element.setAttribute("download", "Arbeitszeitdokumentation.pdf");

      element.style.display = "none";
      document.body.appendChild(element);
      element.click();
      document.body.removeChild(element);

      showSuccess("Generieren erfolgreich!");
    });
    reader.readAsDataURL(blob);
  } catch (e) {
    showError(`Generieren fehlgeschlagen:\n${e}`);
  }
});
