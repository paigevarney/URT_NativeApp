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
            
            // Timer counts down from 45mins
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

                        let progress_bar = egui::ProgressBar::new(progress);
                        ui.add(progress_bar);

                        ui.label(format!("Time remaining: {:02}:{:02}", minutes, seconds));

                        ui. horizontal(|ui| {

                            let start = ui.button("Start");
                            let stop = ui.button("Stop");
                            let reset = ui.button("Reset");
                            let intervene = ui.button("Intervene");

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
                            // IMPLEMENT: intervention timer (refer to old web console)
                            //     I believe it paused the current countdown timer and started counting from 10 mins
                        });                     
                    });

                    ctx.request_repaint();
                });
            });
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.set_width(ui.available_width() / 4.0);
                    ui.set_height(300.0);
                    ui.vertical(|ui| {
                        ui.label("Memory Usage");
                        
                        let plot = Plot::new("Memory Usage");

                        // placeholder data
                        // IMPLEMENT: graphs to recieve live data from UDP sockets to map
                        let graph: Vec<[f64; 2]> = vec![[0.0, 0.0], [1.0, 1.0], [2.0, 4.0], [3.0, 9.0], [4.0, 16.0], [5.0, 25.0]];


                        plot.y_axis_position(Left).show(ui, |plot_ui| {
                            plot_ui.line(Line::new(PlotPoints::from(graph)));
                        });

                        // Update label to reflect data as well
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
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.set_width(ui.available_width()/1.5);
                    ui.set_height(620.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                        ui.group(|ui| {
                            ui.set_width(ui.available_width()/3.0);
                            ui.set_height(200.0);
                            ui.vertical(|ui| {
                                ui.label("Scoop Arm");
                                ui.horizontal(|ui| {
                                    ui.button("Up");
                                    ui.button("Down");
                                });
                                ui.horizontal(|ui| {
                                    ui.button("Creep On");
                                    ui.button("Creep Off");
                                });
                                ui.button("Stop");
                            });
                        });
                        ui.group(|ui| {
                            ui.set_width(ui.available_width()/2.0);
                            ui.set_height(200.0);
                            ui.vertical(|ui| {
                                ui.label("Scoop Rotation");
                                ui.horizontal(|ui| {
                                    ui.button("Clockwise");
                                    ui.button("Counter Clockwise");
                                });
                                ui.button("Stop");
                            });
                        });
                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            ui.set_height(200.0);
                            ui.vertical(|ui| {
                                ui.label("Processor");
                                ui.horizontal(|ui| {
                                    ui.button("Out");
                                    ui.button("In");
                                });
                                ui.horizontal(|ui| {
                                    ui.button("Creep On");
                                    ui.button("Creep Off");
                                });
                                ui.button("Stop");
                            });
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.group(|ui| {
                            ui.set_width(ui.available_width()/3.0);
                            ui.set_height(200.0);
                            ui.vertical(|ui| {
                                ui.label("Heating");
                                ui.horizontal(|ui| {
                                    ui.button("Pad Up");
                                    ui.button("Pad Down");
                                });
                                ui.horizontal(|ui| {
                                    ui.button("Heater On");
                                    ui.button("Heater Off");
                                });
                            });
                        });
                        ui.group(|ui| {
                            ui.set_width(ui.available_width()/2.0);
                            ui.set_height(200.0);
                            ui.vertical(|ui| {
                                ui.label("Moisture");
                                ui.horizontal(|ui| {
                                    ui.button("Up");
                                    ui.button("Down");
                                });
                            });
                        });
                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            ui.set_height(200.0);
                            ui.label("Moist Meter");
                        });
                    });
                    ui.group(|ui| {
                            ui.set_width(ui.available_width()/3.0);
                            ui.set_height(200.0);
                            ui.label("Thermistor Temperature");
                    });
                });
            });
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.set_height(300.0);
                        ui.label("[add camera stream]");
                        ui.horizontal(|ui| {
                                ui.button("HD");
                                ui.button("◑");
                                ui.button("🔍");
                                ui.button("⏸");
                                ui.button("🔀");
                            });
                    });
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.set_height(300.0);
                        ui.label("[add digital twin]");
                    });
                });
            });
        });
    }
}

#[derive(Default)]
struct ArmApp {}

impl eframe::App for ArmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.set_width(ui.available_width()/1.5);
                    ui.set_height(620.0);
                    ui.label("[add stuff here]");
                });
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.set_height(300.0);
                        ui.label("[add camera stream]");
                        ui.horizontal(|ui| {
                                ui.button("HD");
                                ui.button("◑");
                                ui.button("🔍");
                                ui.button("⏸");
                                ui.button("🔀");
                            });
                    });
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.set_height(300.0);
                        ui.label("[add digital twin]");
                    });
                });
            });
        });
    }
}

#[derive(Default)]
struct ElectricalApp {}

impl eframe::App for ElectricalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("This is the electrical panel");

        });
    }
}

#[derive(Default)]
struct ChassisApp {}

impl eframe::App for ChassisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("This is the chassis panel");

        });
    }
}
// IMPLEMENT: ability to dynamically add and remove camera streams and choose which to view from dropdown (selectable_value)
#[derive(Debug, PartialEq)]
enum Cameras { 
    CameraOne, 
    CameraTwo, 
    CameraThree, 
    CameraFour,
    CameraFive,
}

#[derive(Default)]
struct CamApp {
    selected: Cameras,
}

impl Default for Cameras { 
    fn default() -> Self {
        Cameras::CameraOne
    }
}

impl CamApp {
    pub fn new() -> Self {
        Self {
            selected: Cameras::CameraOne,
        }
    }
}


impl eframe::App for CamApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.set_width(ui.available_width() / 2.0);
                        ui.set_height(300.0);
                        ui.vertical(|ui| {
                            egui::ComboBox::from_label("")
                                .selected_text(format!("{:?}", self.selected))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.selected, Cameras::CameraOne, "Camera One");
                                    ui.selectable_value(&mut self.selected, Cameras::CameraTwo, "Camera Two");
                                    ui.selectable_value(&mut self.selected, Cameras::CameraThree, "Camera Three");
                                    ui.selectable_value(&mut self.selected, Cameras::CameraFour, "Camera Four");
                                });
                            ui.label("[add camera stream]");
                            ui.horizontal(|ui| {
                                ui.button("HD");
                                ui.button("◑");
                                ui.button("🔍");
                                ui.button("⏸");
                            });
                        });
                     });
                     ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.set_height(300.0);
                        ui.vertical(|ui| {
                            ui.label("Camera Two");
                            ui.label("[add camera stream]");
                            ui.horizontal(|ui| {
                                ui.button("HD");
                                ui.button("◑");
                                ui.button("🔍");
                                ui.button("⏸");
                                    
                            });

                        });
                    });
                });
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.set_width(ui.available_width() / 2.0);
                        ui.set_height(300.0);
                        ui.vertical(|ui| {
                            ui.label("Camera Three");
                            ui.label("[add camera stream]");
                            ui.horizontal(|ui| {
                                ui.button("HD");
                                ui.button("◑");
                                ui.button("🔍");
                                ui.button("⏸");
                             });
                        });
                    });
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.set_height(300.0);
                        ui.vertical(|ui| {
                            ui.label("Camera Four");
                            ui.label("[add camera stream]");
                            ui.horizontal(|ui| {
                                ui.button("HD");
                                ui.button("◑");
                                ui.button("🔍");
                                ui.button("⏸");         
                            });      
                        });
                    });
                });
            });
        });         
    }
}


#[derive(Default)]
struct WrapApp {
    dash_app: DashApp, 
    science_app: ScienceApp,
    arm_app: ArmApp,
    electrical_app: ElectricalApp,
    chassis_app: ChassisApp,
    cam_app: CamApp,
    selected_app: SelectedApp,
}

#[derive(PartialEq)]
enum SelectedApp {
    Dashboard, 
    Science, 
    Arm, 
    Electrical, 
    Chassis,
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

// WrapApp contains functionality to display the chosen page from the menu bar at the top of the screen
impl eframe::App for WrapApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
           ui.horizontal(|ui| {
               ui.heading("URT Native App");
               ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                   ui.horizontal(|ui| {
                       if ui.add(egui::SelectableLabel::new(self.selected_app == SelectedApp::Cameras, "Cameras")).clicked() {
                           self.selected_app = SelectedApp::Cameras;
                       }
                       if ui.add(egui::SelectableLabel::new(self.selected_app == SelectedApp::Chassis, "Chassis")).clicked() {
                           self.selected_app = SelectedApp::Chassis;
                       }
                       if ui.add(egui::SelectableLabel::new(self.selected_app == SelectedApp::Electrical, "Electrical")).clicked() {
                           self.selected_app = SelectedApp::Electrical;
                       }
                       if ui.add(egui::SelectableLabel::new(self.selected_app == SelectedApp::Arm, "Arm")).clicked() {
                           self.selected_app = SelectedApp::Arm;
                       }
                       if ui.add(egui::SelectableLabel::new(self.selected_app == SelectedApp::Science, "Science")).clicked() {
                           self.selected_app = SelectedApp::Science;
                       }
                       if ui.add(egui::SelectableLabel::new(self.selected_app == SelectedApp::Dashboard, "Dashboard")).clicked() {
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
           SelectedApp::Electrical => self.electrical_app.update(ctx, frame),
           SelectedApp::Chassis => self.chassis_app.update(ctx, frame),
           SelectedApp::Cameras => self.cam_app.update(ctx, frame),

       }
   }
}
