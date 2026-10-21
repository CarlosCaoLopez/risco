// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! macOS lets an app watch the mouse and keyboard, and send the copy
//! shortcut, only after the user allows it under Accessibility. Elsewhere
//! there is nothing to ask for.

#[cfg(target_os = "macos")]
mod mac {
    use core_foundation::base::TCFType;
    use core_foundation::boolean::CFBoolean;
    use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
    use core_foundation::string::{CFString, CFStringRef};

    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        static kAXTrustedCheckOptionPrompt: CFStringRef;
        fn AXIsProcessTrustedWithOptions(options: CFDictionaryRef) -> bool;
    }

    pub fn trusted(prompt: bool) -> bool {
        let key = unsafe { CFString::wrap_under_get_rule(kAXTrustedCheckOptionPrompt) };
        let options = CFDictionary::from_CFType_pairs(&[(key, CFBoolean::from(prompt))]);
        unsafe { AXIsProcessTrustedWithOptions(options.as_concrete_TypeRef()) }
    }
}

/// Whether the gesture can work. With `prompt`, macOS shows its own dialog
/// pointing at the setting.
pub fn trusted(prompt: bool) -> bool {
    #[cfg(target_os = "macos")]
    return mac::trusted(prompt);
    #[cfg(not(target_os = "macos"))]
    {
        let _ = prompt;
        true
    }
}

/// Opens System Settings at the Accessibility list.
pub fn open_settings() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let url = "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility";
        std::process::Command::new("open").arg(url).spawn().map(drop).map_err(|e| e.to_string())
    }
    #[cfg(not(target_os = "macos"))]
    Ok(())
}
