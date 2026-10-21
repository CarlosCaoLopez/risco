// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

import "./styles.css";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { LANGUAGES, setLocale, t, translate } from "./i18n";

type Modifier = "ctrl" | "alt" | "shift" | "meta";

interface Config {
  enabled: boolean;
  color: string;
  modifier: Modifier;
  roots: string[];
  animation: boolean;
  sound: boolean;
  volume: number;
  seen_viewers: string[];
  first_run_done: boolean;
  language: string | null;
}

interface Viewer {
  kind: "obsidian" | "vscode";
  name: string;
  path: string;
  installed: boolean;
  present: boolean;
}

interface Snapshot {
  config: Config;
  viewers: Viewer[];
  detectedRoots: string[];
  autostart: boolean;
  accessibility: boolean;
  locale: string;
}

const COLORS = ["yellow", "pink", "green", "blue", "purple"] as const;
const modLabel = (m: Modifier) => (m === "shift" ? t("modifier.shift") : { ctrl: "Ctrl", alt: "Alt", meta: "Super" }[m]);

const $ = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;
let config: Config;

function save() {
  void invoke("save_config", { config });
}

function toast(text: string) {
  const el = $("toast");
  el.textContent = text;
  el.classList.add("show");
  clearTimeout(Number(el.dataset.timer));
  el.dataset.timer = String(setTimeout(() => el.classList.remove("show"), 2200));
}

/** A marker pen for the cup, cap on top, tinted with its colour. */
function pen(id: string): HTMLButtonElement {
  const b = document.createElement("button");
  b.className = "pen";
  b.setAttribute("role", "radio");
  b.dataset.i18nAria = `color.${id}`;
  b.dataset.value = id;
  b.style.setProperty("--c", `var(--${id})`);
  b.innerHTML = `
    <svg viewBox="0 0 34 118" aria-hidden="true">
      <defs>
        <linearGradient id="barrel-${id}" x1="0" x2="1">
          <stop offset="0" stop-color="#2b2b31"/><stop offset=".45" stop-color="#4b4b54"/><stop offset="1" stop-color="#1d1d1f"/>
        </linearGradient>
        <linearGradient id="tint-${id}" x1="0" x2="1">
          <stop offset="0" stop-color="rgb(var(--${id}))" stop-opacity=".85"/>
          <stop offset=".4" stop-color="rgb(var(--${id}))"/>
          <stop offset="1" stop-color="rgb(var(--${id}))" stop-opacity=".75"/>
        </linearGradient>
      </defs>
      <path d="M12 26 h10 l-1.5 -8 h-7 z" fill="url(#tint-${id})"/>
      <rect x="3" y="26" width="28" height="92" rx="9" fill="url(#barrel-${id})"/>
      <rect x="7" y="30" width="5" height="84" rx="2.5" fill="#fff" opacity=".16"/>
      <rect x="3" y="70" width="28" height="12" fill="url(#tint-${id})" opacity=".9"/>
      <g class="cap">
        <rect x="2" y="2" width="30" height="38" rx="9" fill="url(#tint-${id})"/>
        <rect x="6" y="6" width="5" height="30" rx="2.5" fill="#fff" opacity=".5"/>
        <rect x="24" y="10" width="3" height="26" rx="1.5" fill="#000" opacity=".12"/>
      </g>
    </svg>`;
  b.addEventListener("click", () => {
    config.color = id;
    render();
    save();
  });
  return b;
}

function render() {
  $<HTMLInputElement>("enabled").checked = config.enabled;
  $<HTMLInputElement>("animation").checked = config.animation;
  $<HTMLInputElement>("sound").checked = config.sound;
  $<HTMLInputElement>("volume").value = String(config.volume);
  $("volume-row").classList.toggle("off", !config.sound);
  $("mod-key").textContent = modLabel(config.modifier);
  $<HTMLSelectElement>("language").value = config.language ?? "";

  for (const p of $("cup").querySelectorAll<HTMLElement>(".pen")) {
    p.setAttribute("aria-checked", String(p.dataset.value === config.color));
  }
  for (const b of $("modifier").querySelectorAll<HTMLElement>("button")) {
    b.setAttribute("aria-checked", String(b.dataset.value === config.modifier));
  }
  const live = document.querySelector<HTMLElement>("[data-live]")!;
  live.className = `hl hl-${config.color}`;

  renderRoots();
}

let detected: string[] = [];

function li(html: string): HTMLLIElement {
  const el = document.createElement("li");
  el.innerHTML = html;
  return el;
}

const esc = (s: string) => s.replace(/[&<>"]/g, (c) => `&#${c.charCodeAt(0)};`);
const base = (p: string) => p.split(/[\\/]/).filter(Boolean).pop() ?? p;

function renderRoots() {
  const ul = $("roots");
  ul.replaceChildren();
  for (const r of config.roots) {
    const row = li(`<div><strong>${esc(base(r))}</strong><small>${esc(r)}</small></div>
      <button class="icon-btn" aria-label="${t("roots.remove")}">×</button>`);
    row.querySelector("button")!.addEventListener("click", () => {
      config.roots = config.roots.filter((x) => x !== r);
      render();
      save();
    });
    ul.append(row);
  }
  for (const r of detected.filter((d) => !config.roots.includes(d))) {
    ul.append(li(`<div><strong>${esc(base(r))}</strong><small>${esc(r)}</small></div><span class="tag">${t("roots.detected")}</span>`));
  }
  if (!ul.children.length) {
    ul.append(li(`<div class="empty">${t("roots.empty")}</div>`));
  }
}

let viewers: Viewer[] = [];

function renderViewers() {
  const ul = $("viewers");
  ul.replaceChildren();
  for (const v of viewers) {
    const row = li(`<span class="tag">${v.kind === "obsidian" ? "Obsidian" : "VS Code"}</span>
      <div><strong>${esc(v.name)}</strong><small>${esc(v.path)}</small></div>
      <button class="pill">${t("viewers.install")}</button>`);
    const btn = row.querySelector("button")!;
    const done = () => {
      btn.textContent = t("viewers.active");
      btn.classList.add("done");
    };
    if (v.installed) done();
    btn.addEventListener("click", async () => {
      try {
        await invoke("install_plugin", { kind: v.kind, path: v.path });
        v.installed = true;
        done();
        toast(t(v.kind === "obsidian" ? "viewers.installed.obsidian" : "viewers.installed.vscode"));
      } catch (e) {
        toast(String(e));
      }
    });
    ul.append(row);
  }
  if (!viewers.length) {
    ul.append(li(`<div class="empty">${t("viewers.empty")}</div>`));
  }
}

/** Put the whole window in `locale`. */
function localize(locale: string) {
  setLocale(locale);
  translate();
  $<HTMLSelectElement>("language").options[0].text = t("language.system");
  renderViewers();
}

async function init() {
  const snap = await invoke<Snapshot>("snapshot");
  config = snap.config;
  detected = snap.detectedRoots;
  viewers = snap.viewers;

  const cup = $("cup");
  for (const id of COLORS) cup.append(pen(id));

  const language = $<HTMLSelectElement>("language");
  language.append(new Option("", ""), ...LANGUAGES.map(([code, name]) => new Option(name, code)));
  language.addEventListener("change", () => {
    config.language = language.value || null;
    save();
  });
  localize(snap.locale);

  for (const b of $("modifier").querySelectorAll<HTMLElement>("button")) {
    b.addEventListener("click", () => {
      config.modifier = b.dataset.value as Modifier;
      render();
      save();
    });
  }

  const bind = (id: string, key: "enabled" | "animation" | "sound") =>
    $<HTMLInputElement>(id).addEventListener("change", (e) => {
      config[key] = (e.target as HTMLInputElement).checked;
      render();
      save();
    });
  bind("enabled", "enabled");
  bind("animation", "animation");
  bind("sound", "sound");

  $<HTMLInputElement>("volume").addEventListener("change", (e) => {
    config.volume = Number((e.target as HTMLInputElement).value);
    save();
  });
  $("test-sound").addEventListener("click", () => invoke("test_sound", { volume: config.volume }));

  $("permission").hidden = snap.accessibility;
  $("open-accessibility").addEventListener("click", () => invoke("open_accessibility"));

  $<HTMLInputElement>("autostart").checked = snap.autostart;
  $<HTMLInputElement>("autostart").addEventListener("change", async (e) => {
    const on = (e.target as HTMLInputElement).checked;
    try {
      await invoke("set_autostart", { on });
    } catch (err) {
      toast(String(err));
    }
  });

  $("migrate").addEventListener("click", async () => {
    try {
      const notes = await invoke<number>("migrate_marks");
      toast(notes ? t(notes === 1 ? "migrate.done.one" : "migrate.done.other", { n: notes }) : t("migrate.none"));
    } catch (err) {
      toast(String(err));
    }
  });

  $("add-root").addEventListener("click", async () => {
    const dir = await open({ directory: true, multiple: false, title: t("roots.dialog") });
    if (typeof dir === "string" && !config.roots.includes(dir)) {
      config.roots = [...config.roots, dir];
      render();
      save();
    }
  });

  render();

  // The tray and the global shortcuts change settings too.
  await listen<Config>("config", async (e) => {
    const relocalize = e.payload.language !== config.language;
    config = e.payload;
    // Rust resolves "follow the system" to a language, so ask it which.
    if (relocalize) localize((await invoke<Snapshot>("snapshot")).locale);
    render();
  });
}

void init();
