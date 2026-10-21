<p align="center">
  <img src="../src-tauri/icons/128x128@2x.png" width="128" alt="Icono de Risco">
</p>

<h1 align="center">Risco</h1>

<p align="center">
  <a href="../README.md">English</a> · Español · <a href="README.gl.md">Galego</a> · <a href="README.fr.md">Français</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.zh-CN.md">简体中文</a> · <a href="README.tr.md">Türkçe</a> · <a href="README.az.md">Azərbaycanca</a> · <a href="README.hi.md">हिन्दी</a>
</p>

<h3 align="center">Subraya cualquier Markdown. Sin tocarlo.</h3>

<p align="center">
  <a href="https://api.reuse.software/info/github.com/CarlosCaoLopez/risco"><img src="https://api.reuse.software/badge/github.com/CarlosCaoLopez/risco" alt="REUSE status"></a>
  <a href="https://www.bestpractices.dev/projects/0000"><img src="https://www.bestpractices.dev/projects/0000/badge" alt="OpenSSF Best Practices"></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/CarlosCaoLopez/risco"><img src="https://api.scorecard.dev/projects/github.com/CarlosCaoLopez/risco/badge" alt="OpenSSF Scorecard"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0--or--later-blue" alt="License: GPL-3.0-or-later"></a>
</p>

<p align="center">
  Libre y de código abierto. Para Linux, Windows y macOS.
  <br>
  <a href="https://github.com/CarlosCaoLopez/risco/releases/latest">Descargar&nbsp;&rsaquo;</a>
</p>

<br>

## Instalar

Descarga el instalador para tu sistema desde la
[última versión](https://github.com/CarlosCaoLopez/risco/releases/latest):

| | |
|:--|:--|
| **macOS** | Abre el `.dmg` y arrastra Risco a Aplicaciones. |
| **Windows** | Ejecuta el `-setup.exe`. |
| **Linux** | Instala el `.deb` o el `.rpm`, o ejecuta el `.AppImage`. |

O instálalo con Homebrew:

```sh
brew install --cask CarlosCaoLopez/tap/risco
```

Risco está firmado con un Developer ID y notarizado por Apple, así que se abre
como cualquier otra app. En el primer arranque macOS pide acceso de
Accesibilidad, que Risco necesita para ver el gesto. Windows puede mostrar un
aviso de SmartScreen porque el instalador aún no está firmado: elige
*Más información › Ejecutar de todas formas*.

Y ya está. Risco vive en la bandeja, arranca con tu ordenador e instala solo su
plugin en cada bóveda de Obsidian y en VS Code.

<br>

## Mantén. Arrastra. Subrayado.

Mantén <kbd>Ctrl</kbd> (<kbd>⌥</kbd> en macOS) y arrastra sobre cualquier texto
en la vista de lectura de Obsidian, la vista previa de VS Code u Orca. Suena
como un rotulador, y el subrayado se queda.

<!-- demo.gif -->

<br>

## Primero, la lectura.

Los subrayados viven en la página renderizada, así que cámbiate a ella antes de arrastrar:

| | |
|:--|:--|
| **Obsidian** | Vista de lectura: <kbd>Ctrl</kbd>&thinsp;<kbd>E</kbd> (<kbd>⌘</kbd>&thinsp;<kbd>E</kbd>), o el icono del libro arriba a la derecha de la nota. |
| **VS Code** | Vista previa de Markdown: <kbd>Ctrl</kbd>&thinsp;<kbd>Shift</kbd>&thinsp;<kbd>V</kbd> (<kbd>⇧</kbd>&thinsp;<kbd>⌘</kbd>&thinsp;<kbd>V</kbd>), o <kbd>Ctrl</kbd>&thinsp;<kbd>K</kbd> <kbd>V</kbd> para abrirla junto al editor. Haz clic en la vista previa y arrastra. |
| **Orca** | Nada que cambiar: subraya directamente en el editor. |

En la vista de edición de Obsidian o en el editor de VS Code, el gesto no hace nada.

<br>

## Un gesto para todo.

| | |
|:--|:--|
| <kbd>Ctrl</kbd> + arrastrar | Subrayarlo. |
| <kbd>Ctrl</kbd> + arrastrar sobre un subrayado | Quitarlo. |
| <kbd>Ctrl</kbd>&thinsp;<kbd>Alt</kbd>&thinsp;<kbd>1</kbd>–<kbd>5</kbd> | Amarillo, rosa, verde, azul o lila. |
| *Deshacer último* en la bandeja | Deshacer el último. |

<br>

## Tu Markdown. Intacto.

Los subrayados viven en un archivo oculto junto a cada nota, no en la nota.
El plugin de Obsidian y la extensión de VS Code los pintan en su sitio.

Orca no puede cargar plugins, así que allí el subrayado va dentro de la nota
como `++texto++`, el único marcado que renderiza.

Si el editor tiene cambios sin guardar, no se escribe nada.

<br>

## Privado por diseño.

Sin cuenta. Sin red. Sin analíticas.
Risco funciona por completo en tu ordenador, y tus notas nunca salen de él.

<br>

## Especificaciones

| | |
|:--|:--|
| **Plataformas** | Linux (X11), Windows, macOS |
| **Visores** | Obsidian, VS Code, Orca |
| **Hecho con** | Rust, Tauri 2 y TypeScript |
| **Acceso a la red** | Ninguno |
| **Precio** | Gratis |

<br>

## Compilar desde el código

```sh
npm install
npm run tauri build
```

En Linux también necesitas los paquetes habituales de Tauri/WebKitGTK,
`libxtst` y una biblioteca de appindicator.

La app instala por sí misma los plugins de los visores la primera vez que se ejecuta.

<br>

## Licencia

Risco es software libre bajo la [GNU General Public License v3.0 o posterior](../LICENSE), y se distribuye **sin ninguna garantía**. Para contribuir, lee [CONTRIBUTING.md](../CONTRIBUTING.md) (en inglés); los problemas de seguridad se informan en privado según [SECURITY.md](../SECURITY.md).
