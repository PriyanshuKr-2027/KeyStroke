use std::process::Command;

/// Checks if Accessibility permission is granted to the current process on macOS.
pub fn is_accessibility_granted() -> bool {
    #[cfg(target_os = "macos")]
    {
        #[link(name = "ApplicationServices", kind = "framework")]
        extern "C" {
            fn AXIsProcessTrusted() -> bool;
        }
        unsafe { AXIsProcessTrusted() }
    }

    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}

/// Opens macOS System Preferences/Settings → Accessibility pane.
pub fn open_accessibility_settings() {
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .status();
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = Command::new("echo")
            .arg("Opening mock accessibility settings")
            .status();
    }
}

/// Checks if the focused macOS AXUIElement is an AXSecureTextField or AXPasswordField.
pub fn is_focused_element_secure() -> bool {
    #[cfg(target_os = "macos")]
    {
        use core_foundation::base::TCFType;
        use core_foundation::string::CFString;
        use std::ffi::c_void;

        type AXUIElementRef = *const c_void;
        type CFStringRef = *const c_void;
        type AXError = i32;

        #[link(name = "ApplicationServices", kind = "framework")]
        extern "C" {
            fn AXUIElementCreateSystemWide() -> AXUIElementRef;
            fn AXUIElementCopyAttributeValue(
                element: AXUIElementRef,
                attribute: CFStringRef,
                value: *mut *const c_void,
            ) -> AXError;
        }

        unsafe {
            let system_wide = AXUIElementCreateSystemWide();
            if system_wide.is_null() {
                return false;
            }

            let attr_focused = CFString::new("AXFocusedUIElement");
            let mut focused_elem: *const c_void = std::ptr::null();
            let err = AXUIElementCopyAttributeValue(
                system_wide,
                attr_focused.as_concrete_TypeRef(),
                &mut focused_elem,
            );

            core_foundation::base::CFRelease(system_wide as *const _);

            if err != 0 || focused_elem.is_null() {
                return false;
            }

            let attr_role = CFString::new("AXRole");
            let mut role_value: *const c_void = std::ptr::null();
            let role_err = AXUIElementCopyAttributeValue(
                focused_elem,
                attr_role.as_concrete_TypeRef(),
                &mut role_value,
            );

            let attr_subrole = CFString::new("AXSubrole");
            let mut subrole_value: *const c_void = std::ptr::null();
            let _ = AXUIElementCopyAttributeValue(
                focused_elem,
                attr_subrole.as_concrete_TypeRef(),
                &mut subrole_value,
            );

            core_foundation::base::CFRelease(focused_elem);

            let is_secure = if role_err == 0 && !role_value.is_null() {
                let role_cf = CFString::wrap_under_get_rule(role_value as CFStringRef);
                let role_str = role_cf.to_string();
                core_foundation::base::CFRelease(role_value);

                let is_role_secure = role_str == "AXSecureTextField" || role_str == "AXPasswordField";

                let is_subrole_secure = if !subrole_value.is_null() {
                    let subrole_cf = CFString::wrap_under_get_rule(subrole_value as CFStringRef);
                    let subrole_str = subrole_cf.to_string();
                    core_foundation::base::CFRelease(subrole_value);
                    subrole_str == "AXSecureTextField" || subrole_str == "AXPasswordField"
                } else {
                    false
                };

                is_role_secure || is_subrole_secure
            } else {
                false
            };

            is_secure
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        false
    }
}
