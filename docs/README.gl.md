<p align="center">
  <img src="../src-tauri/icons/128x128@2x.png" width="128" alt="Icona de Risco">
</p>

<h1 align="center">Risco</h1>

<p align="center">
  <a href="../README.md">English</a> · <a href="README.es.md">Español</a> · Galego · <a href="README.fr.md">Français</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.zh-CN.md">简体中文</a> · <a href="README.tr.md">Türkçe</a> · <a href="README.az.md">Azərbaycanca</a> · <a href="README.hi.md">हिन्दी</a>
</p>

<h3 align="center">Subliña calquera Markdown. Sen tocalo.</h3>

<p align="center">
  Libre e de código aberto. Para Linux, Windows e macOS.
  <br>
  <a href="https://github.com/CarlosCaoLopez/risco/releases/latest">Descargar&nbsp;&rsaquo;</a>
</p>

<br>

## Instalar

Descarga o instalador para o teu sistema desde a
[última versión](https://github.com/CarlosCaoLopez/risco/releases/latest):

| | |
|:--|:--|
| **macOS** | Abre o `.dmg` e arrastra Risco a Aplicacións. |
| **Windows** | Executa o `-setup.exe`. |
| **Linux** | Instala o `.deb` ou o `.rpm`, ou executa o `.AppImage`. |

Ou instálao con Homebrew:

```sh
brew install --cask CarlosCaoLopez/tap/risco
```

Risco está asinado cun Developer ID e notarizado por Apple, así que se abre
coma calquera outra app. No primeiro arranque macOS pide acceso de
Accesibilidade, que Risco precisa para ver o xesto. Windows pode amosar un
aviso de SmartScreen porque o instalador aínda non está asinado: escolle
*Máis información › Executar de todos modos*.

E xa está. Risco vive na bandexa, arrinca co teu ordenador e instala el só o
seu plugin en cada bóveda de Obsidian e en VS Code.

<br>

## Mantén. Arrastra. Subliñado.

Mantén <kbd>Ctrl</kbd> (<kbd>⌥</kbd> en macOS) e arrastra sobre calquera texto
na vista de lectura de Obsidian, a vista previa de VS Code ou Orca. Soa coma un
rotulador, e o subliñado queda.

<!-- demo.gif -->

<br>

## Primeiro, a lectura.

Os subliñados viven na páxina renderizada, así que cambia a ela antes de arrastrar:

| | |
|:--|:--|
| **Obsidian** | Vista de lectura: <kbd>Ctrl</kbd>&thinsp;<kbd>E</kbd> (<kbd>⌘</kbd>&thinsp;<kbd>E</kbd>), ou a icona do libro arriba á dereita da nota. |
| **VS Code** | Vista previa de Markdown: <kbd>Ctrl</kbd>&thinsp;<kbd>Shift</kbd>&thinsp;<kbd>V</kbd> (<kbd>⇧</kbd>&thinsp;<kbd>⌘</kbd>&thinsp;<kbd>V</kbd>), ou <kbd>Ctrl</kbd>&thinsp;<kbd>K</kbd> <kbd>V</kbd> para abrila xunto ao editor. Fai clic na vista previa e arrastra. |
| **Orca** | Nada que cambiar: subliña directamente no editor. |

Na vista de edición de Obsidian ou no editor de VS Code, o xesto non fai nada.

<br>

## Un xesto para todo.

| | |
|:--|:--|
| <kbd>Ctrl</kbd> + arrastrar | Subliñalo. |
| <kbd>Ctrl</kbd> + arrastrar sobre un subliñado | Quitalo. |
| <kbd>Ctrl</kbd>&thinsp;<kbd>Alt</kbd>&thinsp;<kbd>1</kbd>–<kbd>5</kbd> | Amarelo, rosa, verde, azul ou lila. |
| *Desfacer o último* na bandexa | Desfacer o último. |

<br>

## O teu Markdown. Intacto.

Os subliñados viven nun ficheiro oculto xunto a cada nota, non na nota.
O plugin de Obsidian e a extensión de VS Code píntanos no seu sitio.

Orca non pode cargar plugins, así que alí o subliñado vai dentro da nota como
`++texto++`, a única marcaxe que renderiza.

Se o editor ten cambios sen gardar, non se escribe nada.

<br>

## Privado por deseño.

Sen conta. Sen rede. Sen analíticas.
Risco funciona por completo no teu ordenador, e as túas notas nunca saen del.

<br>

## Especificacións

| | |
|:--|:--|
| **Plataformas** | Linux (X11), Windows, macOS |
| **Visores** | Obsidian, VS Code, Orca |
| **Feito con** | Rust, Tauri 2 e TypeScript |
| **Acceso á rede** | Ningún |
| **Prezo** | De balde |

<br>

## Compilar desde o código

```sh
npm install
npm run tauri build
```

En Linux tamén precisas os paquetes habituais de Tauri/WebKitGTK, `libxtst` e
unha biblioteca de appindicator.

A app instala por si mesma os plugins dos visores a primeira vez que se executa.
