use tauri::{AppHandle, Emitter,Manager};
use tracing::{Event, Subscriber};
use tracing_subscriber::{ layer::Context ,Layer,registry::LookupSpan};

pub struct FrontendLogger {
    pub(crate) app: AppHandle,
}

impl<S> Layer<S> for FrontendLogger
where S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        // 提取message的访问器
        let mut visitor = MessageVisitor::new();
        event.record(&mut visitor);

        let message = visitor.message;
        let level = event.metadata().level().to_string();

        let _ = self.app.emit("log-output",serde_json::json!({
            "level": level,
            "message": message,
            "time": chrono::Utc::now().format("%H %M %S").to_string(),
        }));
    }
}

pub struct MessageVisitor {
    message: String,
}

impl tracing::field::Visit for MessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }
}

impl MessageVisitor {
    pub fn new() -> MessageVisitor {
        MessageVisitor { message: "".to_string() }
    }
}