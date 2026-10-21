<p align="center">
  <img src="../src-tauri/icons/128x128@2x.png" width="128" alt="Risco simgesi">
</p>

<h1 align="center">Risco</h1>

<p align="center">
  <a href="../README.md">English</a> · <a href="README.es.md">Español</a> · <a href="README.gl.md">Galego</a> · <a href="README.fr.md">Français</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.zh-CN.md">简体中文</a> · Türkçe · <a href="README.az.md">Azərbaycanca</a> · <a href="README.hi.md">हिन्दी</a>
</p>

<h3 align="center">Her Markdown’u vurgulayın. Ona dokunmadan.</h3>

<p align="center">
  <a href="https://api.reuse.software/info/github.com/CarlosCaoLopez/risco"><img src="https://api.reuse.software/badge/github.com/CarlosCaoLopez/risco" alt="REUSE status"></a>
  <a href="https://www.bestpractices.dev/projects/0000"><img src="https://www.bestpractices.dev/projects/0000/badge" alt="OpenSSF Best Practices"></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/CarlosCaoLopez/risco"><img src="https://api.scorecard.dev/projects/github.com/CarlosCaoLopez/risco/badge" alt="OpenSSF Scorecard"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0--or--later-blue" alt="License: GPL-3.0-or-later"></a>
</p>

<p align="center">
  Özgür ve açık kaynak. Linux, Windows ve macOS için.
  <br>
  <a href="https://github.com/CarlosCaoLopez/risco/releases/latest">İndir&nbsp;&rsaquo;</a>
</p>

<br>

## Kurulum

Sisteminize uygun yükleyiciyi
[son sürümden](https://github.com/CarlosCaoLopez/risco/releases/latest) indirin:

| | |
|:--|:--|
| **macOS** | `.dmg` dosyasını açın ve Risco’yu Uygulamalar’a sürükleyin. |
| **Windows** | `-setup.exe` dosyasını çalıştırın. |
| **Linux** | `.deb` veya `.rpm` paketini kurun ya da `.AppImage` dosyasını çalıştırın. |

Ya da Homebrew ile kurun:

```sh
brew install --cask CarlosCaoLopez/tap/risco
```

Risco bir Developer ID ile imzalanmış ve Apple tarafından onaylanmıştır, bu yüzden
diğer uygulamalar gibi açılır. İlk açılışta macOS, Risco’nun hareketi görebilmesi
için gereken Erişilebilirlik iznini ister. Yükleyici henüz imzalı olmadığından
Windows bir SmartScreen uyarısı gösterebilir: *Ek bilgi › Yine de çalıştır*’ı seçin.

Hepsi bu. Risco sistem tepsisinde yaşar, bilgisayarınızla birlikte başlar ve
eklentisini her Obsidian kasasına ve VS Code’a kendiliğinden kurar.

<br>

## Basılı tut. Sürükle. Vurgulandı.

<kbd>Ctrl</kbd> (macOS’ta <kbd>⌥</kbd>) tuşunu basılı tutun ve Obsidian’ın okuma
görünümünde, VS Code önizlemesinde ya da Orca’da herhangi bir metnin üzerinde
sürükleyin. Keçeli kalem gibi ses çıkarır ve vurgu kalır.

<!-- demo.gif -->

<br>

## Önce okuma görünümü.

Vurgular işlenmiş sayfada yaşar, bu yüzden sürüklemeden önce ona geçin:

| | |
|:--|:--|
| **Obsidian** | Okuma görünümü: <kbd>Ctrl</kbd>&thinsp;<kbd>E</kbd> (<kbd>⌘</kbd>&thinsp;<kbd>E</kbd>) ya da notun sağ üstündeki kitap simgesi. |
| **VS Code** | Markdown önizlemesi: <kbd>Ctrl</kbd>&thinsp;<kbd>Shift</kbd>&thinsp;<kbd>V</kbd> (<kbd>⇧</kbd>&thinsp;<kbd>⌘</kbd>&thinsp;<kbd>V</kbd>) ya da düzenleyicinin yanında açmak için <kbd>Ctrl</kbd>&thinsp;<kbd>K</kbd> <kbd>V</kbd>. Önizlemeye tıklayın, sonra sürükleyin. |
| **Orca** | Geçiş gerekmez: doğrudan düzenleyicide vurgulayın. |

Obsidian’ın düzenleme görünümünde veya VS Code düzenleyicisinde hareket hiçbir şey yapmaz.

<br>

## Her şey için tek hareket.

| | |
|:--|:--|
| <kbd>Ctrl</kbd> + sürükle | Vurgula. |
| Bir vurgunun üzerinde <kbd>Ctrl</kbd> + sürükle | Vurguyu kaldır. |
| <kbd>Ctrl</kbd>&thinsp;<kbd>Alt</kbd>&thinsp;<kbd>1</kbd>–<kbd>5</kbd> | Sarı, pembe, yeşil, mavi veya mor. |
| Tepsideki *Sonuncuyu geri al* | Sonuncuyu geri al. |

<br>

## Markdown’unuz. Olduğu gibi.

Vurgular notun kendisinde değil, her notun yanındaki gizli bir dosyada yaşar.
Obsidian eklentisi ve VS Code uzantısı onları yerinde boyar.

Orca eklenti yükleyemez, bu yüzden orada vurgu nota `++metin++` olarak, Orca’nın
işlediği tek biçimlendirme olarak yazılır.

Düzenleyicide kaydedilmemiş değişiklikler varsa hiçbir şey yazılmaz.

<br>

## Tasarımı gereği gizli.

Hesap yok. Ağ yok. Analitik yok.
Risco tamamen bilgisayarınızda çalışır ve notlarınız oradan hiç çıkmaz.

<br>

## Teknik Özellikler

| | |
|:--|:--|
| **Platformlar** | Linux (X11), Windows, macOS |
| **Görüntüleyiciler** | Obsidian, VS Code, Orca |
| **Yapım** | Rust, Tauri 2 ve TypeScript |
| **Ağ erişimi** | Yok |
| **Fiyat** | Ücretsiz |

<br>

## Kaynaktan derleme

```sh
npm install
npm run tauri build
```

Linux’ta ayrıca alışılmış Tauri/WebKitGTK paketlerine, `libxtst`’ye ve bir
appindicator kütüphanesine ihtiyacınız var.

Uygulama, görüntüleyici eklentilerini ilk çalıştığında kendisi kurar.

<br>

## Lisans

Risco, [GNU General Public License v3.0 veya sonrası](../LICENSE) altında yayımlanan özgür bir yazılımdır ve **hiçbir garanti olmadan** dağıtılır. Katkıda bulunmak için [CONTRIBUTING.md](../CONTRIBUTING.md) (İngilizce) dosyasını okuyun; güvenlik sorunlarını [SECURITY.md](../SECURITY.md) uyarınca gizli olarak bildirin.
