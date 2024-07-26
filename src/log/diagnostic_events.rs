use serde_json;
use stellar_xdr::curr::{DiagnosticEvent, Limits, ReadXdr};

pub fn diagnostic_events(events: &[String], level: tracing::Level) {
    for (i, event_xdr) in events.iter().enumerate() {
        let event = DiagnosticEvent::from_xdr_base64(event_xdr, Limits::none())
            .expect("Failed to decode Diagnostic Event XDR");
        let json = serde_json::to_string(&event).expect("JSON encoding failed");
        let log_message = format!("{i}: \"{event_xdr:#?}\" {json}");
        match level {
            tracing::Level::TRACE => tracing::trace!("{}", log_message),
            tracing::Level::INFO => tracing::info!("{}", log_message),
            tracing::Level::ERROR => tracing::error!("{}", log_message),
            _ => tracing::debug!("{}", log_message), // Default to debug for other levels
        }
    }
}
