// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

// Draws the glass ink while the user selects, in a
// transparent click-through window covering one monitor.

import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

interface MarkerEvent {
  kind: "start" | "move" | "end";
  x: number;
  y: number;
  color: string;
  result: "added" | "removed" | "error" | null;
}

const RGB: Record<string, string> = {
  yellow: "255, 232, 102",
  pink: "255, 64, 129",
  green: "52, 211, 105",
  blue: "64, 156, 255",
  purple: "175, 102, 255",
};

/** Rough height of a line of text, in CSS px. */
const LINE = 24;

// Where Rust asked for this window; replaced by where it really is, since
// the window manager may move it (e.g. below a top panel).
const params = new URLSearchParams(location.search);
const origin = { x: Number(params.get("x") ?? 0), y: Number(params.get("y") ?? 0) };
let scale = Number(params.get("s") ?? devicePixelRatio) || 1;

const win = getCurrentWindow();
async function locateSelf() {
  const [pos, factor] = await Promise.all([win.innerPosition(), win.scaleFactor()]);
  origin.x = pos.x;
  origin.y = pos.y;
  scale = factor;
}
void locateSelf();
void win.onMoved(() => void locateSelf());
void win.onResized(() => void locateSelf());
void win.onScaleChanged(() => void locateSelf());

const canvas = document.getElementById("ink") as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;

function resize() {
  canvas.width = Math.round(innerWidth * devicePixelRatio);
  canvas.height = Math.round(innerHeight * devicePixelRatio);
  ctx.setTransform(devicePixelRatio, 0, 0, devicePixelRatio, 0, 0);
}
addEventListener("resize", resize);
resize();

/** Global physical pixels to this window's CSS pixels. */
function local(e: MarkerEvent) {
  return { x: (e.x - origin.x) / scale, y: (e.y - origin.y) / scale };
}

function inside(p: { x: number; y: number }) {
  return p.x >= 0 && p.y >= 0 && p.x <= innerWidth && p.y <= innerHeight;
}

interface Band {
  y: number;
  x0: number;
  x1: number;
  seed: number;
}

type Phase = "idle" | "drawing" | "added" | "removed" | "error";

const state = {
  phase: "idle" as Phase,
  mine: false, // the stroke started on this monitor
  color: RGB.yellow,
  bands: [] as Band[],
  endedAt: 0,
};

// ---------- Ink ----------

/** Small deterministic noise so each band's edges wobble the same every frame. */
function wobble(seed: number, t: number) {
  return Math.sin(seed * 12.9898 + t * 0.21) * 0.6 + Math.sin(seed * 78.233 + t * 0.057) * 0.9;
}

function inkPath(b: Band, h: number) {
  const left = Math.min(b.x0, b.x1) - 3;
  const right = Math.max(b.x0, b.x1) + 3;
  const top = b.y - h / 2;
  const bottom = b.y + h / 2;
  const r = Math.min(h * 0.45, (right - left) / 2);
  const p = new Path2D();
  p.moveTo(left + r, top + wobble(b.seed, left));
  for (let x = left + r; x < right - r; x += 8) p.lineTo(x, top + wobble(b.seed, x));
  p.quadraticCurveTo(right + 1, top, right, b.y);
  p.quadraticCurveTo(right - 1, bottom, right - r, bottom + wobble(b.seed + 1, right));
  for (let x = right - r; x > left + r; x -= 8) p.lineTo(x, bottom + wobble(b.seed + 1, x));
  p.quadraticCurveTo(left - 2, bottom, left, b.y);
  p.quadraticCurveTo(left + 1, top, left + r, top + wobble(b.seed, left));
  p.closePath();
  return p;
}

function drawBand(b: Band, alpha: number, squash = 1) {
  const h = LINE * 0.9 * squash;
  if (Math.abs(b.x1 - b.x0) < 1 || h < 1) return;
  const path = inkPath(b, h);
  const top = b.y - h / 2;
  const c = state.color;

  ctx.save();
  ctx.globalAlpha = alpha;
  ctx.shadowColor = `rgba(${c}, .30)`;
  ctx.shadowBlur = 10;
  ctx.shadowOffsetY = 2;
  const body = ctx.createLinearGradient(0, top, 0, top + h);
  body.addColorStop(0, `rgba(${c}, .34)`);
  body.addColorStop(1, `rgba(${c}, .50)`);
  ctx.fillStyle = body;
  ctx.fill(path);
  ctx.shadowColor = "transparent";

  // Specular sheen lit from above.
  const sheen = ctx.createLinearGradient(0, top, 0, top + h);
  sheen.addColorStop(0, "rgba(255,255,255,.55)");
  sheen.addColorStop(0.4, "rgba(255,255,255,.12)");
  sheen.addColorStop(0.58, "rgba(255,255,255,0)");
  ctx.fillStyle = sheen;
  ctx.fill(path);

  ctx.clip(path);
  ctx.strokeStyle = "rgba(255,255,255,.85)";
  ctx.lineWidth = 1.5;
  ctx.stroke(path);
  ctx.restore();

  ctx.save();
  ctx.globalAlpha = alpha;
  ctx.strokeStyle = "rgba(0,0,0,.10)";
  ctx.lineWidth = 0.5;
  ctx.stroke(path);
  ctx.restore();
}

// ---------- Events ----------

function startStroke(e: MarkerEvent) {
  const p = local(e);
  state.mine = inside(p);
  if (!state.mine) return;
  state.phase = "drawing";
  state.color = RGB[e.color] ?? RGB.yellow;
  state.bands = [{ y: p.y, x0: p.x, x1: p.x, seed: Math.random() * 100 }];
  kick();
}

function moveStroke(e: MarkerEvent) {
  if (!state.mine || state.phase !== "drawing") return;
  const p = local(e);
  let band = state.bands[state.bands.length - 1];
  // Moving to another line starts a new band there, like a real selection.
  if (Math.abs(p.y - band.y) > LINE * 0.75) {
    band = { y: p.y, x0: p.x, x1: p.x, seed: Math.random() * 100 };
    state.bands.push(band);
  }
  band.x1 = p.x;
}

function endStroke(e: MarkerEvent) {
  if (!state.mine || state.phase !== "drawing") return;
  state.phase = e.result ?? "error";
  state.endedAt = performance.now();
}

const ease = (t: number) => 1 - Math.pow(1 - Math.min(1, Math.max(0, t)), 3);

function frame(now: number) {
  ctx.clearRect(0, 0, innerWidth, innerHeight);

  if (state.phase === "drawing") {
    for (const b of state.bands) drawBand(b, 1);
  } else if (state.phase !== "idle") {
    const t = now - state.endedAt;
    if (state.phase === "added") {
      // Settle: a brief swell, then fade as the real <mark> takes over.
      const swell = t < 180 ? 1 + 0.04 * Math.sin((t / 180) * Math.PI) : 1;
      const alpha = 1 - ease((t - 220) / 380);
      for (const b of state.bands) drawBand(b, alpha, swell);
      if (t > 620) state.phase = "idle";
    } else if (state.phase === "removed") {
      // Wipe from left to right, like an eraser.
      const k = ease(t / 420);
      for (const b of state.bands) {
        const l = Math.min(b.x0, b.x1);
        const r = Math.max(b.x0, b.x1);
        drawBand({ ...b, x0: l + (r - l) * k, x1: r }, 1 - k * 0.6);
      }
      if (t > 440) state.phase = "idle";
    } else {
      // Nothing changed: the ink shrinks back to its centre and is gone.
      const k = ease(t / 300);
      for (const b of state.bands) {
        const mid = (b.x0 + b.x1) / 2;
        drawBand({ ...b, x0: b.x0 + (mid - b.x0) * k, x1: b.x1 + (mid - b.x1) * k }, 1 - k, 1 - k * 0.5);
      }
      if (t > 320) state.phase = "idle";
    }
  }
  if (state.phase === "idle") {
    ctx.clearRect(0, 0, innerWidth, innerHeight);
    running = false;
    return;
  }
  requestAnimationFrame(frame);
}

// Only animate while there is something on screen.
let running = false;
function kick() {
  if (running) return;
  running = true;
  requestAnimationFrame(frame);
}

void listen<MarkerEvent>("marker", ({ payload }) => {
  if (payload.kind === "start") startStroke(payload);
  else if (payload.kind === "move") moveStroke(payload);
  else endStroke(payload);
});
