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
  unpaint(document.body);
  paint(document.body, records);
}

window.addEventListener("vscode.markdown.updateContent", run);
if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", run);
else run();
