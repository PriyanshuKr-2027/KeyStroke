use tracing::info;

pub const SERVICE_NAME: &str = "KeyMind Engine";

/// Windows Service entry point using windows-service crate.
pub fn service_main() {
    #[cfg(target_os = "windows")]
    {
        use windows_service::service::ServiceControl;
        use windows_service::service_control_handler::{self, ServiceControlHandlerResult};

        let event_handler = move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                ServiceControl::Stop | ServiceControl::Shutdown => {
                    info!("KeyMind Windows Service stopping...");
                    ServiceControlHandlerResult::NoError
                }
                _ => ServiceControlHandlerResult::NoError,
            };
            ServiceControlHandlerResult::NoError
        };

        if let Ok(_status_handle) = service_control_handler::register(SERVICE_NAME, event_handler) {
            info!("KeyMind Engine Windows Service started successfully.");
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        info!("[Mock Windows Service] KeyMind Engine service running");
    }
}
