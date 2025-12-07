export function mountBinaryBackground(container: HTMLElement, cols = 40, rows = 20) {
  // Clear any existing content
  container.innerHTML = "";

  const fragment = document.createDocumentFragment();

  for (let r = 0; r < rows; r++) {
    const row = document.createElement("div");
    row.className = "binary-row";
    for (let c = 0; c < cols; c++) {
      const span = document.createElement("span");
      span.textContent = Math.random() > 0.5 ? "1" : "0";
      span.className = "binary-digit";
      fragment.appendChild(span);
    }
    row.appendChild(document.createElement("br"));
    fragment.appendChild(row);
  }

  container.appendChild(fragment);

  // Simple shimmer / flicker
  setInterval(() => {
    const digits = container.querySelectorAll<HTMLSpanElement>(".binary-digit");
    for (let i = 0; i < digits.length / 12; i++) {
      const el = digits[Math.floor(Math.random() * digits.length)];
      el.classList.toggle("binary-digit-bright");
    }
  }, 400);
}
