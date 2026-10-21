<p align="center">
  <img src="../src-tauri/icons/128x128@2x.png" width="128" alt="Risco ikonu">
</p>

<h1 align="center">Risco</h1>

<p align="center">
  <a href="../README.md">English</a> · <a href="README.es.md">Español</a> · <a href="README.gl.md">Galego</a> · <a href="README.fr.md">Français</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.zh-CN.md">简体中文</a> · <a href="README.tr.md">Türkçe</a> · Azərbaycanca · <a href="README.hi.md">हिन्दी</a>
</p>

<h3 align="center">İstənilən Markdown-u vurğulayın. Ona toxunmadan.</h3>

<p align="center">
  Pulsuz və açıq mənbəli. Linux, Windows və macOS üçün.
  <br>
  <a href="https://github.com/CarlosCaoLopez/risco/releases/latest">Yüklə&nbsp;&rsaquo;</a>
</p>

<br>

## Quraşdırma

Sisteminiz üçün quraşdırıcını
[son buraxılışdan](https://github.com/CarlosCaoLopez/risco/releases/latest) yükləyin:

| | |
|:--|:--|
| **macOS** | `.dmg` faylını açın və Risco-nu Applications qovluğuna sürüşdürün. |
| **Windows** | `-setup.exe` faylını işə salın. |
| **Linux** | `.deb` və ya `.rpm` paketini quraşdırın, ya da `.AppImage` faylını işə salın. |

Və ya Homebrew ilə quraşdırın:

```sh
brew install --cask CarlosCaoLopez/tap/risco
```

Risco Developer ID ilə imzalanıb və Apple tərəfindən notarizasiya olunub, ona görə
də digər tətbiqlər kimi açılır. İlk açılışda macOS Əlçatanlıq icazəsi istəyir;
Risco-ya jesti görmək üçün bu lazımdır. Quraşdırıcı hələ imzalanmadığı üçün
Windows SmartScreen xəbərdarlığı göstərə bilər: *Ətraflı məlumat › Yenə də işə sal*
seçin.

Bu qədər. Risco sistem panelində yaşayır, kompüterinizlə birlikdə başlayır və
plaginini hər Obsidian anbarına və VS Code-a özü quraşdırır.

<br>

## Saxla. Sürüşdür. Vurğulandı.

<kbd>Ctrl</kbd> (macOS-da <kbd>⌥</kbd>) düyməsini saxlayın və Obsidian-ın oxu
görünüşündə, VS Code önizləməsində və ya Orca-da istənilən mətnin üzərindən
sürüşdürün. Flomaster kimi səslənir və vurğu qalır.

<!-- demo.gif -->

<br>

## Əvvəlcə oxu görünüşü.

Vurğular göstərilən səhifədə yaşayır, ona görə sürüşdürməzdən əvvəl ora keçin:

| | |
|:--|:--|
| **Obsidian** | Oxu görünüşü: <kbd>Ctrl</kbd>&thinsp;<kbd>E</kbd> (<kbd>⌘</kbd>&thinsp;<kbd>E</kbd>) və ya qeydin sağ yuxarısındakı kitab ikonu. |
| **VS Code** | Markdown önizləməsi: <kbd>Ctrl</kbd>&thinsp;<kbd>Shift</kbd>&thinsp;<kbd>V</kbd> (<kbd>⇧</kbd>&thinsp;<kbd>⌘</kbd>&thinsp;<kbd>V</kbd>) və ya redaktorun yanında açmaq üçün <kbd>Ctrl</kbd>&thinsp;<kbd>K</kbd> <kbd>V</kbd>. Önizləməyə klikləyin, sonra sürüşdürün. |
| **Orca** | Heç nəyi dəyişməyə ehtiyac yoxdur: birbaşa redaktorda vurğulayın. |

Obsidian-ın redaktə görünüşündə və ya VS Code redaktorunda jest heç nə etmir.

<br>

## Hər şey üçün bir jest.

| | |
|:--|:--|
| <kbd>Ctrl</kbd> + sürüşdür | Vurğula. |
| Vurğunun üzərində <kbd>Ctrl</kbd> + sürüşdür | Vurğunu sil. |
| <kbd>Ctrl</kbd>&thinsp;<kbd>Alt</kbd>&thinsp;<kbd>1</kbd>–<kbd>5</kbd> | Sarı, çəhrayı, yaşıl, mavi və ya bənövşəyi. |
| Paneldəki *Sonuncunu geri al* | Sonuncunu geri al. |

<br>

## Sizin Markdown-unuz. Toxunulmaz.

Vurğular qeydin özündə deyil, hər qeydin yanındakı gizli faylda yaşayır.
Obsidian plagini və VS Code genişlənməsi onları yerində rəngləyir.

Orca plagin yükləyə bilmir, ona görə orada vurğu qeydə `++mətn++` kimi yazılır —
Orca-nın göstərdiyi yeganə işarələmə budur.

Redaktorda yadda saxlanmamış dəyişikliklər varsa, heç nə yazılmır.

<br>

## Dizayn etibarilə məxfi.

Hesab yoxdur. Şəbəkə yoxdur. Analitika yoxdur.
Risco tamamilə kompüterinizdə işləyir və qeydləriniz heç vaxt oradan çıxmır.

<br>

## Texniki göstəricilər

| | |
|:--|:--|
| **Platformalar** | Linux (X11), Windows, macOS |
| **Görüntüləyicilər** | Obsidian, VS Code, Orca |
| **Texnologiyalar** | Rust, Tauri 2 və TypeScript |
| **Şəbəkə girişi** | Yoxdur |
| **Qiymət** | Pulsuz |

<br>

## Mənbədən yığmaq

```sh
npm install
npm run tauri build
```

Linux-da həmçinin adi Tauri/WebKitGTK paketləri, `libxtst` və bir appindicator
kitabxanası lazımdır.

Tətbiq görüntüləyici plaginlərini ilk işə düşəndə özü quraşdırır.
