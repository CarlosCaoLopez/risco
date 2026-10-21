//! User-facing text. The catalogs in `locales/` are shared with the
//! settings window, which picks the same language through `Snapshot::locale`.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

type Catalog = HashMap<String, String>;

/// Every supported language; the first one is the fallback.
const LOCALES: [(&str, &str); 11] = [
    ("en", include_str!("../../locales/en.json")),
    ("es", include_str!("../../locales/es.json")),
    ("gl", include_str!("../../locales/gl.json")),
    ("fr", include_str!("../../locales/fr.json")),
    ("pt-BR", include_str!("../../locales/pt-BR.json")),
    ("ja", include_str!("../../locales/ja.json")),
    ("ko", include_str!("../../locales/ko.json")),
    ("zh-CN", include_str!("../../locales/zh-CN.json")),
    ("tr", include_str!("../../locales/tr.json")),
    ("az", include_str!("../../locales/az.json")),
    ("hi", include_str!("../../locales/hi.json")),
];

static CURRENT: RwLock<&str> = RwLock::new("en");

fn catalogs() -> &'static HashMap<&'static str, Catalog> {
    static CATALOGS: OnceLock<HashMap<&'static str, Catalog>> = OnceLock::new();
    CATALOGS.get_or_init(|| {
        LOCALES
            .iter()
            .map(|(code, json)| (*code, serde_json::from_str(json).expect("valid locale catalog")))
            .collect()
    })
}

/// The supported code for a BCP 47 tag like `pt_PT.UTF-8` or `zh-Hans-CN`.
fn supported(tag: &str) -> Option<&'static str> {
    let lang = tag.split(['-', '_', '.']).next()?.to_ascii_lowercase();
    LOCALES.iter().map(|(code, _)| *code).find(|code| code.split('-').next() == Some(lang.as_str()))
}

/// Use `chosen`, or the system language when it is `None`.
pub fn set(chosen: Option<&str>) {
    let code = chosen
        .and_then(supported)
        .or_else(|| sys_locale::get_locales().find_map(|l| supported(&l)))
        .unwrap_or(LOCALES[0].0);
    *CURRENT.write().expect("locale lock") = code;
}

pub fn current() -> &'static str {
    *CURRENT.read().expect("locale lock")
}

pub fn t(key: &str) -> String {
    [current(), LOCALES[0].0]
        .iter()
        .find_map(|code| catalogs()[code].get(key))
        .cloned()
        .unwrap_or_else(|| key.to_string())
}

/// `t` with each `{name}` replaced by its value.
pub fn tf(key: &str, vars: &[(&str, &str)]) -> String {
    vars.iter().fold(t(key), |s, (name, value)| s.replace(&format!("{{{name}}}"), value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn placeholders(s: &str) -> BTreeSet<&str> {
        s.split('{').skip(1).filter_map(|p| p.split_once('}')).map(|(name, _)| name).collect()
    }

    #[test]
    fn catalogs_match_english() {
        let en = &catalogs()["en"];
        for (code, catalog) in catalogs() {
            let keys: BTreeSet<_> = catalog.keys().collect();
            assert_eq!(keys, en.keys().collect(), "{code} keys");
            for (key, text) in catalog {
                assert_eq!(placeholders(text), placeholders(&en[key]), "{code} {key}");
            }
        }
    }

    #[test]
    fn maps_system_tags() {
        assert_eq!(supported("pt_PT.UTF-8"), Some("pt-BR"));
        assert_eq!(supported("zh-Hans-CN"), Some("zh-CN"));
        assert_eq!(supported("gl-ES"), Some("gl"));
        assert_eq!(supported("de-DE"), None);
    }
}
