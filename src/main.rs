use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My App", native_options, Box::new(|_cc| Box::new(WrapApp::default())));
}

#[derive(Default)]
struct DashApp {}

impl eframe::App for DashApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.set_width(110.0);
                    ui.set_height(150.0);
                    ui.vertical(|ui| {
                        ui.label("Competition Timer");
                        ui.button("Start");
                        let progress = 45.0 * 60.0; // number of seconds in 45 minutes;
                        let progress_bar = egui::ProgressBar::new(progress);
                        ui.add(progress_bar);

                    });
                });
                ui.group(|ui| {
                    ui.set_width(110.0);
                    ui.set_height(150.0);
                    ui.vertical(|ui| {
                        ui.label("Intervention Timer");
                        ui.label("TIMER GOES HERE");
                    });
                });
            });
            ui.horizontal_wrapped(|ui| {
                ui.group(|ui| {
                    ui.set_width(110.0);
                    ui.set_height(150.0);
                    ui.vertical(|ui| {
                        ui.label("Memory Usage");
                        ui.label("GRAPH GOES HERE");
                    });
                });
                ui.group(|ui| {
                    ui.set_width(110.0);
                    ui.set_height(150.0);
                    ui.vertical(|ui| {
                        ui.label("CPU Usage");
                        ui.label("GRAPH GOES HERE");
                    });
                });
                ui.group(|ui| {
                    ui.set_width(110.0);
                    ui.set_height(150.0);
                    ui.vertical(|ui| {
                        ui.label("Downloads");
                        ui.label("GRAPH GOES HERE");
                    });
                });
                ui.group(|ui| {
                    ui.set_width(110.0);
                    ui.set_height(150.0);
                    ui.vertical(|ui| {
                        ui.label("Uploads");
                        ui.label("GRAPH GOES HERE");
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
