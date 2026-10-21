<p align="center">
  <img src="../src-tauri/icons/128x128@2x.png" width="128" alt="Risco आइकन">
</p>

<h1 align="center">Risco</h1>

<p align="center">
  <a href="../README.md">English</a> · <a href="README.es.md">Español</a> · <a href="README.gl.md">Galego</a> · <a href="README.fr.md">Français</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.zh-CN.md">简体中文</a> · <a href="README.tr.md">Türkçe</a> · <a href="README.az.md">Azərbaycanca</a> · हिन्दी
</p>

<h3 align="center">किसी भी Markdown को हाइलाइट करें। उसे छुए बिना।</h3>

<p align="center">
  <a href="https://api.reuse.software/info/github.com/CarlosCaoLopez/risco"><img src="https://api.reuse.software/badge/github.com/CarlosCaoLopez/risco" alt="REUSE status"></a>
  <a href="https://www.bestpractices.dev/projects/0000"><img src="https://www.bestpractices.dev/projects/0000/badge" alt="OpenSSF Best Practices"></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/CarlosCaoLopez/risco"><img src="https://api.scorecard.dev/projects/github.com/CarlosCaoLopez/risco/badge" alt="OpenSSF Scorecard"></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0--or--later-blue" alt="License: GPL-3.0-or-later"></a>
</p>

<p align="center">
  मुफ़्त और ओपन सोर्स। Linux, Windows और macOS के लिए।
  <br>
  <a href="https://github.com/CarlosCaoLopez/risco/releases/latest">डाउनलोड करें&nbsp;&rsaquo;</a>
</p>

<br>

## इंस्टॉल करें

अपने सिस्टम के लिए इंस्टॉलर
[नवीनतम रिलीज़](https://github.com/CarlosCaoLopez/risco/releases/latest) से डाउनलोड करें:

| | |
|:--|:--|
| **macOS** | `.dmg` खोलें और Risco को Applications में खींचें। |
| **Windows** | `-setup.exe` चलाएँ। |
| **Linux** | `.deb` या `.rpm` इंस्टॉल करें, या `.AppImage` चलाएँ। |

या Homebrew से इंस्टॉल करें:

```sh
brew install --cask CarlosCaoLopez/tap/risco
```

Risco एक Developer ID से साइन किया गया है और Apple द्वारा नोटराइज़ किया गया है, इसलिए यह किसी भी दूसरे ऐप की तरह खुलता है। पहली बार चलाने पर macOS एक्सेसिबिलिटी एक्सेस माँगता है, जो Risco को जेस्चर देखने के लिए चाहिए। इंस्टॉलर अभी साइन नहीं है, इसलिए Windows SmartScreen चेतावनी दिखा सकता है: *अधिक जानकारी › फिर भी चलाएँ* चुनें।

बस इतना ही। Risco ट्रे में रहता है, आपके कंप्यूटर के साथ शुरू होता है, और हर Obsidian वॉल्ट और VS Code में अपना प्लगइन खुद इंस्टॉल कर देता है।

<br>

## दबाएँ। खींचें। हाइलाइट।

<kbd>Ctrl</kbd> (macOS पर <kbd>⌥</kbd>) दबाए रखें और Obsidian के रीडिंग व्यू, VS Code प्रीव्यू या Orca में किसी भी टेक्स्ट पर खींचें। यह फ़ेल्ट-टिप पेन जैसा सुनाई देता है, और हाइलाइट बना रहता है।

<!-- demo.gif -->

<br>

## पहले रीडिंग व्यू।

हाइलाइट रेंडर किए गए पेज पर होते हैं, इसलिए खींचने से पहले उस पर जाएँ:

| | |
|:--|:--|
| **Obsidian** | रीडिंग व्यू: <kbd>Ctrl</kbd>&thinsp;<kbd>E</kbd> (<kbd>⌘</kbd>&thinsp;<kbd>E</kbd>), या नोट के ऊपर दाईं ओर किताब का आइकन। |
| **VS Code** | Markdown प्रीव्यू: <kbd>Ctrl</kbd>&thinsp;<kbd>Shift</kbd>&thinsp;<kbd>V</kbd> (<kbd>⇧</kbd>&thinsp;<kbd>⌘</kbd>&thinsp;<kbd>V</kbd>), या एडिटर के बगल में खोलने के लिए <kbd>Ctrl</kbd>&thinsp;<kbd>K</kbd> <kbd>V</kbd>। प्रीव्यू पर क्लिक करें, फिर खींचें। |
| **Orca** | कुछ बदलने की ज़रूरत नहीं: सीधे एडिटर में हाइलाइट करें। |

Obsidian के एडिटिंग व्यू या VS Code एडिटर में जेस्चर कुछ नहीं करता।

<br>

## हर काम के लिए एक जेस्चर।

| | |
|:--|:--|
| <kbd>Ctrl</kbd> + खींचें | हाइलाइट करें। |
| किसी हाइलाइट पर <kbd>Ctrl</kbd> + खींचें | उसे हटाएँ। |
| <kbd>Ctrl</kbd>&thinsp;<kbd>Alt</kbd>&thinsp;<kbd>1</kbd>–<kbd>5</kbd> | पीला, गुलाबी, हरा, नीला या बैंगनी। |
| ट्रे में *पिछला पूर्ववत करें* | आख़िरी वाला पूर्ववत करें। |

<br>

## आपका Markdown। जैसा था वैसा।

हाइलाइट नोट में नहीं, बल्कि हर नोट के पास एक छिपी फ़ाइल में रहते हैं। Obsidian प्लगइन और VS Code एक्सटेंशन उन्हें उसी जगह दिखाते हैं।

Orca प्लगइन लोड नहीं कर सकता, इसलिए वहाँ हाइलाइट नोट में `++टेक्स्ट++` के रूप में जाता है, जो इकलौता मार्कअप है जिसे वह रेंडर करता है।

अगर एडिटर में बिना सहेजे बदलाव हैं, तो कुछ भी नहीं लिखा जाता।

<br>

## डिज़ाइन से ही निजी।

कोई अकाउंट नहीं। कोई नेटवर्क नहीं। कोई एनालिटिक्स नहीं।
Risco पूरी तरह आपके कंप्यूटर पर चलता है, और आपके नोट्स कभी बाहर नहीं जाते।

<br>

## तकनीकी विवरण

| | |
|:--|:--|
| **प्लेटफ़ॉर्म** | Linux (X11), Windows, macOS |
| **व्यूअर** | Obsidian, VS Code, Orca |
| **बनाया गया** | Rust, Tauri 2 और TypeScript से |
| **नेटवर्क एक्सेस** | कोई नहीं |
| **कीमत** | मुफ़्त |

<br>

## सोर्स से बिल्ड करें

```sh
npm install
npm run tauri build
```

Linux पर आपको सामान्य Tauri/WebKitGTK पैकेज, `libxtst` और एक appindicator लाइब्रेरी भी चाहिए।

ऐप पहली बार चलने पर व्यूअर प्लगइन खुद इंस्टॉल कर देता है।

<br>

## लाइसेंस

Risco [GNU General Public License v3.0 या बाद के संस्करण](../LICENSE) के तहत मुक्त सॉफ़्टवेयर है, और **बिना किसी वारंटी** के वितरित किया जाता है। योगदान करने के लिए [CONTRIBUTING.md](../CONTRIBUTING.md) (अंग्रेज़ी) पढ़ें; सुरक्षा समस्याओं की रिपोर्ट [SECURITY.md](../SECURITY.md) के अनुसार निजी रूप से करें।
