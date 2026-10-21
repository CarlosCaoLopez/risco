<p align="center">
  <img src="../src-tauri/icons/128x128@2x.png" width="128" alt="Ícone do Risco">
</p>

<h1 align="center">Risco</h1>

<p align="center">
  <a href="../README.md">English</a> · <a href="README.es.md">Español</a> · <a href="README.gl.md">Galego</a> · <a href="README.fr.md">Français</a> · Português (Brasil) · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.zh-CN.md">简体中文</a> · <a href="README.tr.md">Türkçe</a> · <a href="README.az.md">Azərbaycanca</a> · <a href="README.hi.md">हिन्दी</a>
</p>

<h3 align="center">Destaque qualquer Markdown. Sem mexer nele.</h3>

<p align="center">
  Livre e de código aberto. Para Linux, Windows e macOS.
  <br>
  <a href="https://github.com/CarlosCaoLopez/risco/releases/latest">Baixar&nbsp;&rsaquo;</a>
</p>

<br>

## Instalar

Baixe o instalador para o seu sistema na
[versão mais recente](https://github.com/CarlosCaoLopez/risco/releases/latest):

| | |
|:--|:--|
| **macOS** | Abra o `.dmg` e arraste o Risco para Aplicativos. |
| **Windows** | Execute o `-setup.exe`. |
| **Linux** | Instale o `.deb` ou o `.rpm`, ou execute o `.AppImage`. |

Ou instale com o Homebrew:

```sh
brew install --cask CarlosCaoLopez/tap/risco
```

O Risco é assinado com um Developer ID e notarizado pela Apple, então abre como
qualquer outro app. Na primeira execução, o macOS pede acesso de
Acessibilidade, que o Risco precisa para ver o gesto. O Windows pode mostrar um
aviso do SmartScreen porque o instalador ainda não é assinado: escolha
*Mais informações › Executar assim mesmo*.

Pronto. O Risco fica na bandeja, inicia com o computador e instala sozinho o seu
plugin em cada cofre do Obsidian e no VS Code.

<br>

## Segure. Arraste. Destacado.

Segure <kbd>Ctrl</kbd> (<kbd>⌥</kbd> no macOS) e arraste sobre qualquer texto no
modo de leitura do Obsidian, na pré-visualização do VS Code ou no Orca. Soa como
um marca-texto, e o destaque fica.

<!-- demo.gif -->

<br>

## Primeiro, a leitura.

Os destaques vivem na página renderizada, então mude para ela antes de arrastar:

| | |
|:--|:--|
| **Obsidian** | Modo de leitura: <kbd>Ctrl</kbd>&thinsp;<kbd>E</kbd> (<kbd>⌘</kbd>&thinsp;<kbd>E</kbd>), ou o ícone de livro no canto superior direito da nota. |
| **VS Code** | Pré-visualização de Markdown: <kbd>Ctrl</kbd>&thinsp;<kbd>Shift</kbd>&thinsp;<kbd>V</kbd> (<kbd>⇧</kbd>&thinsp;<kbd>⌘</kbd>&thinsp;<kbd>V</kbd>), ou <kbd>Ctrl</kbd>&thinsp;<kbd>K</kbd> <kbd>V</kbd> para abri-la ao lado do editor. Clique na pré-visualização e arraste. |
| **Orca** | Nada a mudar: destaque direto no editor. |

No modo de edição do Obsidian ou no editor do VS Code, o gesto não faz nada.

<br>

## Um gesto para tudo.

| | |
|:--|:--|
| <kbd>Ctrl</kbd> + arrastar | Destacar. |
| <kbd>Ctrl</kbd> + arrastar sobre um destaque | Removê-lo. |
| <kbd>Ctrl</kbd>&thinsp;<kbd>Alt</kbd>&thinsp;<kbd>1</kbd>–<kbd>5</kbd> | Amarelo, rosa, verde, azul ou lilás. |
| *Desfazer o último* na bandeja | Desfazer o último. |

<br>

## Seu Markdown. Intocado.

Os destaques ficam em um arquivo oculto ao lado de cada nota, não na nota.
O plugin do Obsidian e a extensão do VS Code os pintam no lugar.

O Orca não carrega plugins, então lá o destaque vai para dentro da nota como
`++texto++`, a única marcação que ele renderiza.

Se o editor tiver alterações não salvas, nada é gravado.

<br>

## Privado por design.

Sem conta. Sem rede. Sem análises.
O Risco roda inteiramente no seu computador, e suas notas nunca saem dele.

<br>

## Especificações

| | |
|:--|:--|
| **Plataformas** | Linux (X11), Windows, macOS |
| **Visualizadores** | Obsidian, VS Code, Orca |
| **Feito com** | Rust, Tauri 2 e TypeScript |
| **Acesso à rede** | Nenhum |
| **Preço** | Grátis |

<br>

## Compilar a partir do código

```sh
npm install
npm run tauri build
```

No Linux você também precisa dos pacotes habituais do Tauri/WebKitGTK,
`libxtst` e uma biblioteca de appindicator.

O app instala os plugins dos visualizadores sozinho na primeira execução.
