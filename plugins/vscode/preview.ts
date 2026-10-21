// Runs inside the Markdown preview: paints the records the extension embedded.

import { paint, unpaint, type HlRecord } from "../shared/paint";

function run() {
  const data = document.querySelector<HTMLElement>(".risco-data");
  let records: HlRecord[] = [];
  try {
    records = JSON.parse(data?.dataset.records ?? "[]");
  } catch {
    // Leave the page as rendered.
  }
  observer.disconnect();
  unpaint(document.body);
  paint(document.body, records);
  observer.observe(document.body, { childList: true, subtree: true, attributes: true, attributeFilter: ["data-records"] });
}

// VS Code fills the page after this script runs, and later swaps parts of
// it, without always firing `updateContent`. Repaint whenever it changes.
let pending = 0;
const observer = new MutationObserver(() => {
  cancelAnimationFrame(pending);
  pending = requestAnimationFrame(run);
});

window.addEventListener("vscode.markdown.updateContent", run);
if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", run);
else run();
