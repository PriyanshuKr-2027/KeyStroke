#[allow(unused_imports)]
use crate::accessibility::{is_accessibility_granted, is_focused_element_secure, open_accessibility_settings};
use crate::context::ContextBuffer;
#[allow(unused_imports)]
use crate::events::{Event, Modifiers};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;

/// Helper function computing exponential backoff duration given attempt index (0-indexed).
pub fn get_backoff_duration(attempt: usize) -> Duration {
    let secs = match attempt {
        0 => 1,
        1 => 2,
        2 => 4,
        3 => 8,
        4 => 16,
        _ => 16,
    };
    Duration::from_secs(secs)
}

/// Spawns the CGEventTap background thread.
pub fn spawn_event_tap_thread(sender: mpsc::Sender<Event>) {
    thread::spawn(move || {
        if !is_accessibility_granted() {
            let _ = sender.blocking_send(Event::PermissionRequired);
            open_accessibility_settings();
            return;
        }

        #[allow(unused_mut, unused_variables)]
        let mut context_buffer = ContextBuffer::default();
        #[allow(unused_mut, unused_variables)]
        let mut consecutive_failures = 0;

        loop {
            #[cfg(target_os = "macos")]
            {
                let run_result = run_macos_event_tap(&sender, &mut context_buffer);
                if run_result.is_err() {
                    if consecutive_failures < 5 {
                        let delay = get_backoff_duration(consecutive_failures);
                        consecutive_failures += 1;
                        thread::sleep(delay);
                    } else {
                        let _ = sender.blocking_send(Event::EngineError("tap_dead"));
                        break;
                    }
                } else {
                    consecutive_failures = 0;
                }
            }

            #[cfg(not(target_os = "macos"))]
            {
                // Fallback for non-macOS environments
                thread::sleep(Duration::from_secs(1));
            }
        }
    });
}

#[cfg(target_os = "macos")]
fn run_macos_event_tap(
    sender: &mpsc::Sender<Event>,
    context_buffer: &mut ContextBuffer,
) -> Result<(), &'static str> {
    use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
    use core_graphics::event::{
        CGEventFlags, CGEventTap, CGEventTapOptions, CGEventTapPlacement, CGEventType,
        EventTapLocation,
    };

    let sender_clone = sender.clone();
    let mut ctx_buf = context_buffer.clone();

    let tap = CGEventTap::new(
        CGEventTapLocation::HID,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::ListenOnly,
        vec![CGEventType::KeyDown],
        move |_proxy, event_type, event| {
            if event_type == CGEventType::KeyDown {
                let flags = event.get_flags();
                let modifiers = Modifiers {
                    shift: flags.contains(CGEventFlags::CGEventFlagShift),
                    control: flags.contains(CGEventFlags::CGEventFlagControl),
                    option: flags.contains(CGEventFlags::CGEventFlagAlternate),
                    command: flags.contains(CGEventFlags::CGEventFlagCommand),
                };

                let is_secure = is_focused_element_secure();

                if is_secure {
                    let _ = sender_clone.try_send(Event::SensitiveFieldKeyPress);
                } else {
                    // Extract unicode character
                    let mut buf = [0u16; 4];
                    let mut length = 0;
                    event.keyboard_get_unicode_string(4, &mut length, buf.as_mut_ptr());

                    if length > 0 {
                        if let Ok(s) = String::from_utf16(&buf[..length as usize]) {
                            for c in s.chars() {
                                let _ = sender_clone.try_send(Event::KeyPress {
                                    key: c,
                                    modifiers,
                                });

                                if let Some((word, context)) = ctx_buf.push_char(c) {
                                    let _ = sender_clone.try_send(Event::WordCompleted {
                                        word,
                                        context,
                                    });
                                }
                            }
                        }
                    }
                }
            }
            None
        },
    );

    match tap {
        Ok(tap) => {
            let loop_source = tap
                .mach_port
                .create_runloop_source(0)
                .map_err(|_| "Failed to create runloop source")?;

            unsafe {
                let run_loop = CFRunLoop::get_current();
                run_loop.add_source(&loop_source, kCFRunLoopCommonModes);
                tap.enable();
                CFRunLoop::run_current();
            }
            Ok(())
        }
        Err(_) => Err("Tap creation failed"),
    }
}
