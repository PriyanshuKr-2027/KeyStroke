# KeyStroke — Windows Distribution Notes

## SmartScreen Warning

When first installing KeyStroke, Windows may show a **"Windows protected your PC"** dialog.
This is expected for newly signed software and is NOT a security risk.

**To proceed:**
1. Click **"More info"**
2. Click **"Run anyway"**

This warning will disappear automatically as more users install KeyStroke and Microsoft's
SmartScreen reputation system verifies the binary is safe.

## Why does KeyStroke need keyboard access?

KeyStroke uses a low-level keyboard hook (`WH_KEYBOARD_LL`) to intercept keystrokes
system-wide. This is the same mechanism used by:
- Password managers (1Password, Bitwarden)
- Text expanders (AutoHotkey, Espanso)
- Accessibility tools

**Everything runs locally on your machine. No keystrokes are sent to any server.**

## Installation Location

KeyStroke must be installed in `C:\Program Files\KeyStroke\` to enable full functionality.
Installing elsewhere (Desktop, Downloads) will disable the UIPI bypass required for
KeyStroke to work in elevated windows (admin terminals, Task Manager).

The installer sets this path by default.

## Code Signing

KeyStroke binaries are signed by the **SignPath Foundation** using an OV certificate.
This means:
- Defender does not quarantine the binary
- The keyboard hook is not silently removed by Windows
- You can verify authenticity by right-clicking the `.exe` → Properties → Digital Signatures
