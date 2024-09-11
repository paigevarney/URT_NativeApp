use eframe::egui;
use egui::Vec2;
use egui_plot::{HPlacement::Left, Line, Plot, PlotPoints};
use std::time::{Duration, Instant};

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true),
        ..Default::default()
    };
    eframe::run_native("URT", native_options, Box::new(|_cc| Box::new(WrapApp::default())));
}

struct DashApp {
    comp_timer: CompTimer,
}

struct CompTimer {
    countdown_duration: Duration, 
    remaining_time: Duration, 
    start_time: Option<Instant>,
    is_running: bool
}

impl Default for CompTimer {
    fn default() -> Self {
        Self {
            countdown_duration: Duration::new(2700, 0), // 2700 seconds in 45 mins
            remaining_time: Duration::new(2700, 0),
            start_time: None, 
            is_running: false,
        }
    }
}

impl Default for DashApp {
    fn default() -> Self {
        Self {
            comp_timer: CompTimer::default(),
        }
    }
}

impl eframe::App for DashApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            let comp_timer = &mut self.comp_timer;
            
            if comp_timer.is_running {
                if let Some(start_time) = comp_timer.start_time {
                    let elapsed = start_time.elapsed();
                    comp_timer.remaining_time = if elapsed >= comp_timer.countdown_duration {
                        Duration::new(0,0)
                    } else {
                        comp_timer.countdown_duration - elapsed
                    };
                }
            }

            let minutes = comp_timer.remaining_time.as_secs() / 60;
            let seconds = comp_timer.remaining_time.as_secs() % 60;
            let progress = if comp_timer.countdown_duration.as_secs_f32() > 0.0 {
                 1.0 - (comp_timer.remaining_time.as_secs_f32() / comp_timer.countdown_duration.as_secs_f32())
            } else {
                0.0
            };

            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.set_width(ui.available_width() / 2.0);
                    ui.vertical(|ui| {
                        ui.label("Competition Timer");
                       
                        ui.add_space(25.0);

                        let progress_bar = egui::ProgressBar::new(progress)
                            .show_percentage()
                            .text(format!("{:.1}%", progress * 100.0));
                        ui.add(progress_bar);

                        ui.label(format!("Time remaining: {:02}:{:02}", minutes, seconds));

                        ui. horizontal(|ui| {
                            let start = ui.button("Start");
                            let stop = ui.button("Stop");
                            let reset = ui.button("Reset");

                            if comp_timer.is_running {
                            if stop.clicked() {
                                comp_timer.is_running = false;
                            }
                            } else {
                                if start.clicked() {
                                    comp_timer.start_time = Some(Instant::now());
                                    comp_timer.is_running = true;
                                }
                            }

                            if reset.clicked() {
                                comp_timer.remaining_time = comp_timer.countdown_duration;
                                comp_timer.start_time = None;
                                comp_timer.is_running = false;
                            }
                        });

                       
                    });

                    ctx.request_repaint();
                });
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.vertical(|ui| {
                        ui.label("Intervention Timer");
                        ui.label("TIMER GOES HERE");
                    });
                });
            });
            ui.horizontal_wrapped(|ui| {
                ui.group(|ui| {
                    ui.set_width(ui.available_width() / 4.0);
                    ui.set_height(300.0);
                    ui.vertical(|ui| {
                        ui.label("Memory Usage");
                        
                        let plot = Plot::new("Memory Usage");

                        // placeholder data
                        let graph: Vec<[f64; 2]> = vec![[0.0, 0.0], [1.0, 1.0], [2.0, 4.0], [3.0, 9.0], [4.0, 16.0], [5.0, 25.0]];


                        plot.y_axis_position(Left).show(ui, |plot_ui| {
                            plot_ui.line(Line::new(PlotPoints::from(graph)));
                        });

                        ui.label("0%");
                    });
                });
                ui.group(|ui| {
                    ui.set_width((ui.available_width() - 40.0) / 3.0);
                    ui.set_height(300.0);
                    ui.vertical(|ui| {
                        ui.label("CPU Usage");

                        let plot = Plot::new("CPU Usage");

                        // placeholder data
                        let graph: Vec<[f64; 2]> = vec![[0.0, 0.0], [1.0, 1.0], [2.0, 4.0], [3.0, 9.0], [4.0, 16.0], [5.0, 25.0]];


                        plot.y_axis_position(Left).show(ui, |plot_ui| {
                            plot_ui.line(Line::new(PlotPoints::from(graph)));
                        });

                        ui.label("0%");
                    });
                });
                ui.group(|ui| {
                    ui.set_width((ui.available_width() - 20.0) / 2.0);
                    ui.set_height(300.0);
                    ui.vertical(|ui| {
                        ui.label("Network Downlink");

                        let plot = Plot::new("Network Downlink");

                        // placeholder data
                        let graph: Vec<[f64; 2]> = vec![[0.0, 0.0], [1.0, 1.0], [2.0, 4.0], [3.0, 9.0], [4.0, 16.0], [5.0, 25.0]];


                        plot.y_axis_position(Left).show(ui, |plot_ui| {
                            plot_ui.line(Line::new(PlotPoints::from(graph)));
                        });

                        ui.label("0MB/s");
                    });
                });
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.set_height(300.0);
                    ui.vertical(|ui| {
                        ui.label("Network Uplink");

                        let plot = Plot::new("Network Uplink");

                        // placeholder data
                        let graph: Vec<[f64; 2]> = vec![[0.0, 0.0], [1.0, 1.0], [2.0, 4.0], [3.0, 9.0], [4.0, 16.0], [5.0, 25.0]];


                        plot.y_axis_position(Left).show(ui, |plot_ui| {
                            plot_ui.line(Line::new(PlotPoints::from(graph)));
                        });

                        ui.label("0MB/s");
                    });
                });
            });
        });
    }
}

#[derive(Default)]
struct ScienceApp {}

impl eframe::App for ScienceApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("This is the science panel");
        });
    }
}

#[derive(Default)]
struct ArmApp {}

impl eframe::App for ArmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("This is the arm panel");
        });
    }
}

#[derive(Default)]
struct CamApp {}

impl eframe::App for CamApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("This is the cameras panel");
        });
    }
}


#[derive(Default)]
struct WrapApp {
    dash_app: DashApp, 
    science_app: ScienceApp,
    arm_app: ArmApp,
    cam_app: CamApp,
    selected_app: SelectedApp,
}

enum SelectedApp {
    Dashboard, 
    Science, 
    Arm, 
    Cameras,
}

impl Default for SelectedApp {
    fn default() -> Self {
        SelectedApp::Dashboard
    }
}

impl WrapApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for WrapApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
           ui.horizontal(|ui| {
               ui.heading("URT Native App");
               ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                   ui.horizontal(|ui| {
                       if ui.button("Cameras").clicked() {
                           self.selected_app = SelectedApp::Cameras;
                       }
                       if ui.button("Arm").clicked() {
                           self.selected_app = SelectedApp::Arm;
                       }
                       if ui.button("Science").clicked() {
                           self.selected_app = SelectedApp::Science;
                       }
                       if ui.button("Dashboard").clicked() {
                           self.selected_app = SelectedApp::Dashboard;
                       }
                   });
               });
           });
       });

       match self.selected_app {
           SelectedApp::Dashboard => self.dash_app.update(ctx, frame), 
           SelectedApp::Science => self.science_app.update(ctx, frame),
           SelectedApp::Arm => self.arm_app.update(ctx, frame), 
           SelectedApp::Cameras => self.cam_app.update(ctx, frame),

       }
   }
}
