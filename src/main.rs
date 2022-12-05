use std::{self, time::Duration};
use std::string::ToString;
use strum_macros::{Display, EnumString};
use eframe::egui;
use tokio::sync::broadcast::Sender;


#[derive(Display, Debug, Clone)]
enum TimerState {
    Running,
    Paused,
    Finished,
    Empty,
}

#[derive(Debug, Clone)]
struct Split {
    name: String,
    length: Duration,
}

#[derive(Debug, Clone, Display)]
enum TimerEvent {
    Hi
}

struct TimerWindow {
    title: String,
    splits: Vec<Split>,
    stopwatch: Duration,
    state: TimerState,
    tx: tokio::sync::broadcast::Sender<TimerEvent>,
    rx: tokio::sync::broadcast::Receiver<TimerEvent>
}

impl Default for TimerWindow {
    fn default() -> Self {
        let (tx, rx) = tokio::sync::broadcast::channel(128);
        Self {
            title: String::from("Hello, World!"),
            splits: vec![],
            stopwatch: Duration::from_micros(0),
            state: TimerState::Empty,
            tx,
            rx
        }
    }
}

fn send_timer_event(event: TimerEvent, tx: Sender<TimerEvent>, ctx: egui::Context) {
    tokio::spawn(async move {
        let _ = tx.send(event);
        ctx.request_repaint();
    });
}

impl eframe::App for TimerWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.title);
            if let Ok(timer_event) = self.rx.try_recv() {
                match timer_event {
                    TimerEvent::Hi => println!("hey")
                }
            }
            if ui.button("Click me").clicked() {
                    send_timer_event(TimerEvent::Hi, self.tx.clone(), ctx.clone());
            };
        });
    }
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().expect("Failed to start tokio runtime!");
    let _enter = runtime.enter();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    let timer_window = TimerWindow::default();
    eframe::run_native(
        "Thyme",
        options,
        Box::new(|_cc| Box::new(timer_window)),
    );
}
