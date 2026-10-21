<p align="center">
  <img src="../src-tauri/icons/128x128@2x.png" width="128" alt="Risco 图标">
</p>

<h1 align="center">Risco</h1>

<p align="center">
  <a href="../README.md">English</a> · <a href="README.es.md">Español</a> · <a href="README.gl.md">Galego</a> · <a href="README.fr.md">Français</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · 简体中文 · <a href="README.tr.md">Türkçe</a> · <a href="README.az.md">Azərbaycanca</a> · <a href="README.hi.md">हिन्दी</a>
</p>

<h3 align="center">高亮任何 Markdown。原文丝毫不动。</h3>

<p align="center">
  <a href="https://api.reuse.software/info/github.com/CarlosCaoLopez/risco"><img src="https://api.reuse.software/badge/github.com/CarlosCaoLopez/risco" alt="REUSE status"></a>
  <a href="https://www.bestpractices.dev/projects/0000"><img src="https://www.bestpractices.dev/projects/0000/badge" alt="OpenSSF Best Practices"></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/CarlosCaoLopez/risco"><img src="https://api.scorecard.dev/projects/github.com/CarlosCaoLopez/risco/badge" alt="OpenSSF Scorecard"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0--or--later-blue" alt="License: GPL-3.0-or-later"></a>
</p>

<p align="center">
  免费开源。支持 Linux、Windows 和 macOS。
  <br>
  <a href="https://github.com/CarlosCaoLopez/risco/releases/latest">下载&nbsp;&rsaquo;</a>
</p>

<br>

## 安装

从[最新版本](https://github.com/CarlosCaoLopez/risco/releases/latest)下载适合你系统的安装包：

| | |
|:--|:--|
| **macOS** | 打开 `.dmg`，把 Risco 拖进“应用程序”。 |
| **Windows** | 运行 `-setup.exe`。 |
| **Linux** | 安装 `.deb` 或 `.rpm`，或者直接运行 `.AppImage`。 |

也可以用 Homebrew 安装：

```sh
brew install --cask CarlosCaoLopez/tap/risco
```

Risco 使用 Developer ID 签名并经过 Apple 公证，所以能像其他应用一样直接打开。首次启动时，macOS 会请求“辅助功能”权限，Risco 需要它来识别手势。由于安装程序尚未签名，Windows 可能会显示 SmartScreen 警告：选择 *更多信息 › 仍要运行* 即可。

就这些。Risco 常驻托盘，随电脑启动，并会自动把插件安装到每个 Obsidian 库和 VS Code 中。

<br>

## 按住。划过。已高亮。

按住 <kbd>Ctrl</kbd>（macOS 上为 <kbd>⌥</kbd>），在 Obsidian 的阅读视图、VS Code 的预览或 Orca 中划过任意文本。你会听到荧光笔的声音，高亮就留下了。

<!-- demo.gif -->

<br>

## 先切到阅读视图。

高亮作用于渲染后的页面，所以划之前先切换过去：

| | |
|:--|:--|
| **Obsidian** | 阅读视图：<kbd>Ctrl</kbd>&thinsp;<kbd>E</kbd>（<kbd>⌘</kbd>&thinsp;<kbd>E</kbd>），或点击笔记右上角的书本图标。 |
| **VS Code** | Markdown 预览：<kbd>Ctrl</kbd>&thinsp;<kbd>Shift</kbd>&thinsp;<kbd>V</kbd>（<kbd>⇧</kbd>&thinsp;<kbd>⌘</kbd>&thinsp;<kbd>V</kbd>），或用 <kbd>Ctrl</kbd>&thinsp;<kbd>K</kbd> <kbd>V</kbd> 在编辑器旁打开。先点一下预览，再划。 |
| **Orca** | 无需切换：直接在编辑器里高亮。 |

在 Obsidian 的编辑视图或 VS Code 编辑器中，这个手势不起作用。

<br>

## 一个手势，搞定一切。

| | |
|:--|:--|
| <kbd>Ctrl</kbd> + 拖动 | 高亮。 |
| 在高亮上 <kbd>Ctrl</kbd> + 拖动 | 取消高亮。 |
| <kbd>Ctrl</kbd>&thinsp;<kbd>Alt</kbd>&thinsp;<kbd>1</kbd>–<kbd>5</kbd> | 黄、粉、绿、蓝、紫。 |
| 托盘中的 *撤销上一次* | 撤销最近一次。 |

<br>

## 你的 Markdown，原封不动。

高亮保存在每篇笔记旁边的隐藏文件里，而不是笔记本身。Obsidian 插件和 VS Code 扩展会把它们绘制在原位。

Orca 无法加载插件，所以在那里高亮会以 `++文本++` 的形式写进笔记——这是它唯一能渲染的标记。

如果编辑器有未保存的更改，就什么都不写入。

<br>

## 从设计上保护隐私。

无需账户。不联网。无统计分析。
Risco 完全在你的电脑上运行，你的笔记永远不会离开它。

<br>

## 技术规格

| | |
|:--|:--|
| **平台** | Linux (X11)、Windows、macOS |
| **查看器** | Obsidian、VS Code、Orca |
| **构建技术** | Rust、Tauri 2 和 TypeScript |
| **网络访问** | 无 |
| **价格** | 免费 |

<br>

## 从源码构建

```sh
npm install
npm run tauri build
```

在 Linux 上，还需要常见的 Tauri/WebKitGTK 软件包、`libxtst` 和一个 appindicator 库。

应用首次运行时会自行安装查看器插件。

<br>

## 许可证

Risco 是依据 [GNU 通用公共许可证 v3.0 或更高版本](../LICENSE) 发布的自由软件，**不提供任何担保**。如需贡献，请阅读 [CONTRIBUTING.md](../CONTRIBUTING.md)（英文）；安全问题请按照 [SECURITY.md](../SECURITY.md) 私下报告。
