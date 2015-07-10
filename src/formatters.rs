//! Formatters used by `YdResponse::explain`

use notify_rust::Notification;

/// Base trait for formatters
pub trait Formatter {
    fn red       (&self, &str) -> String;
    fn yellow    (&self, &str) -> String;
    fn purple    (&self, &str) -> String;
    fn cyan      (&self, &str) -> String;
    fn underline (&self, &str) -> String;
    fn default   (&self, &str) -> String;

    fn print (&mut self, word: &str, body: &str);
}

/// Plain text formatter
pub struct PlainFormatter;

impl Formatter for PlainFormatter {
    fn default   (&self, s: &str) -> String { s.to_string() }
    fn red       (&self, s: &str) -> String { s.to_string() }
    fn yellow    (&self, s: &str) -> String { s.to_string() }
    fn purple    (&self, s: &str) -> String { s.to_string() }
    fn cyan      (&self, s: &str) -> String { s.to_string() }
    fn underline (&self, s: &str) -> String { s.to_string() }
    fn print (&mut self, _: &str, body: &str) { println!("{}", body); }
}

/// Ansi escaped colored formatter
pub struct AnsiFormatter;

impl Formatter for AnsiFormatter {
    fn default   (&self, s: &str) -> String { s.to_string() }
    fn red       (&self, s: &str) -> String { format!("\x1b[31m{}\x1b[0m", s) }
    fn yellow    (&self, s: &str) -> String { format!("\x1b[33m{}\x1b[0m", s) }
    fn purple    (&self, s: &str) -> String { format!("\x1b[35m{}\x1b[0m", s) }
    fn cyan      (&self, s: &str) -> String { format!("\x1b[36m{}\x1b[0m", s) }
    fn underline (&self, s: &str) -> String { format!("\x1b[4m{}\x1b[0m", s) }
    fn print (&mut self, _: &str, body: &str) { println!("{}", body); }
}

/// HTML-style formatter, suitable for desktop notification
pub struct HtmlFormatter{
    notify: bool,
    notifier: Notification,
}

impl HtmlFormatter{
    pub fn new(notify: bool) -> HtmlFormatter {
        HtmlFormatter{
            notify: notify,
            notifier: Notification::new()
        }
    }
}

impl Formatter for HtmlFormatter {
    fn red       (&self, s: &str) -> String { format!(r#"<span color="red">{}</span>"#, s) }
    fn yellow    (&self, s: &str) -> String { format!(r#"<span color="goldenrod">{}</span>"#, s) }
    fn purple    (&self, s: &str) -> String { format!(r#"<span color="purple">{}</span>"#, s) }
    fn cyan      (&self, s: &str) -> String { format!(r#"<span color="navy">{}</span>"#, s) }
    fn underline (&self, s: &str) -> String { format!(r#"<u>{}</u>"#, s) }
    fn default   (&self, s: &str) -> String { format!(r#"{}"#, s) }

    fn print (&mut self, word: &str, body: &str) {
        if self.notify {
            self.notifier
                .appname("ydcv")
                .summary(word)
                .body(body)
                .timeout(30000)
                .update();
        }else{
            println!("{}", body);
        }
    }
}