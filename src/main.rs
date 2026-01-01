#![windows_subsystem = "windows"]
use eframe::egui;
use chrono::Local;
mod alerts;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 300.0]).with_transparent(true).with_decorations(true),
        ..Default::default()
    };

    eframe::run_native(
        "Real-Time Clock",
        options,
        Box::new(|_cc| Ok(Box::new(TimeApp::default()))),
    )
}

struct TimeApp {
    timer_seconds: f64,
    duration: f64,
    timer_running: bool,
    has_triggered: bool,
}

impl Default for TimeApp {
    fn default() -> Self {
        Self {
            timer_seconds: 60.0, // Default to 60 second timer
            duration: 60.0,
            timer_running: false,
            has_triggered: false,
        }
    }
}

impl eframe::App for TimeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- LOGIC ---
        if self.timer_running && self.timer_seconds > 0.0 {
            self.timer_seconds -= ctx.input(|i| i.stable_dt) as f64;
            self.has_triggered = false; // Reset guard while running
        }   else if self.timer_seconds <= 0.0 {
                self.timer_running = false;
                self.timer_seconds = 0.0;

                if !self.has_triggered {
                    ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnBottom));
                    ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(true));

                    let ctx_clone = ctx.clone();
                    std::thread::spawn(move || {
                    alerts::trigger_timer_end(&ctx_clone); // <-- Start module alerts.rs function trigger_timer_end()
                    ctx_clone.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
                    ctx_clone.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(false));
                    ctx_clone.send_viewport_cmd(egui::ViewportCommand::Focus);
                    ctx_clone.request_repaint();
                });

                self.has_triggered = true; // Set guard to prevent multiple alerts
                }
        }

        // --- UI ---
        egui::CentralPanel::default().show(ctx, |ui| {
            let current_time = Local::now().format("%H:%M:%S").to_string();
            
            // 1. Get the current available window size
            let window_size = ui.available_size();
            
            // 2. Calculate font size based on window width
            // This multiplier (0.15) can be adjusted to fit the text perfectly
            let font_size = (window_size.x * 0.15).min(window_size.y * 0.4);
            

            // 1. Calculate a changing Hue (0.0 to 1.0) based on time
            // ctx.input(|i| i.time) gives seconds since the app started
            let seconds = ctx.input(|i| i.time);
            let hue = (seconds * 0.2) % 1.0; // Change 0.2 to speed up/slow down

            // 2. Convert HSV to a Color32 (Hue, Saturation, Value)
            let rainbow_color = egui::Color32::from(
                egui::ecolor::Hsva::new(hue as f32, 0.8, 1.0, 1.0)
            );

            ui.vertical_centered(|ui| {
                ui.add_space(window_size.y * 0.1); // Add a small top margin
                ui.heading("Current Time");
                
                // 3. Apply the dynamic size to the text
                ui.label(
                    egui::RichText::new(current_time)
                        .size(font_size)
                        .monospace()
                        .color(rainbow_color) // <---set the color here
                );
                ui.separator();

                // Render the Timer
                ui.label("Set Timer Duration (seconds):");
                ui.add(
                    egui::Slider::new(&mut self.duration, 1.0..=7200.0)
                        .text("s")
                        .logarithmic(true) // Helpful for Large ranges (1s to 2hr)
                );
                ui.label(
                        egui::RichText::new(format!("{:.1}s", self.timer_seconds))
                        .size(30.0)
                        .strong()
                );

                // Button to Start/Stop Timer
                let button_text = if self.timer_running { "Stop Timer" } else {"Start Timer" };
                if ui.button(button_text).clicked() {
                    if self.timer_running {
                        self.timer_running = false;
                    }   else {
                        self.timer_seconds = self.duration;
                        self.timer_running = true;
                    }
                }

                // Reset Button
                if ui.button("Reset").clicked() {
                    self.timer_running = false;
                    self.timer_seconds = 60.0;
                }
            });
        });

        ctx.request_repaint();
    }
}

