#![windows_subsystem = "windows"]

//! The RetroVerse desktop app. A thin window over the library: everything it does is in the
//! crate's modules, and this file only draws it.

use std::path::PathBuf;
use std::process::Child;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::time::Duration;

use eframe::egui::{self, RichText};
use retroverse::catalog::Game;
use retroverse::library::{Build, Library};
use retroverse::{family, launch, scan, theme};

const DEPRECATION: &str = "RetroVerse by Navine is a base project: scanning, detecting and starting builds are not implemented.";

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("RetroVerse")
            .with_inner_size([940.0, 620.0])
            .with_min_inner_size([760.0, 480.0])
            .with_icon(std::sync::Arc::new(egui::IconData {
                rgba: theme::icon_rgba(64),
                width: 64,
                height: 64,
            })),
        ..Default::default()
    };
    eframe::run_native(
        "RetroVerse",
        options,
        Box::new(|cc| {
            theme::apply(&cc.egui_ctx);
            Ok(Box::new(App::new()))
        }),
    )
}

struct App {
    lib: Library,
    tab: Game,
    status: String,
    scanning: Option<Receiver<Vec<PathBuf>>>,
    running: Vec<Child>,
}

impl App {
    fn new() -> Self {
        Self {
            lib: Library::load(),
            tab: Game::Fortnite,
            status: String::new(),
            scanning: None,
            running: Vec::new(),
        }
    }

    fn save(&mut self) {
        if let Err(e) = self.lib.save() {
            self.status = format!("Could not save the library: {e}");
        }
    }

    fn add_folder(&mut self, folder: PathBuf) {
        if self.lib.has(&folder) {
            self.status = format!("{} is already in the library.", folder.display());
            return;
        }
        match Build::from_folder(&folder) {
            Ok(build) => {
                self.tab = build.game;
                self.status = format!("Added {}.", build.label);
                self.lib.add(build);
                self.save();
            }
            Err(e) => self.status = e,
        }
    }

    /// Scans on its own thread, so the window stays responsive while a large folder is walked.
    fn start_scan(&mut self, root: PathBuf) {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(scan::find_builds(&root, 10));
        });
        self.scanning = Some(rx);
        self.status = "Scanning…".into();
    }

    fn poll_scan(&mut self, ctx: &egui::Context) {
        let Some(rx) = &self.scanning else { return };
        match rx.try_recv() {
            Ok(found) => {
                self.scanning = None;
                let before = self.lib.builds.len();
                for folder in found {
                    if !self.lib.has(&folder) {
                        if let Ok(build) = Build::from_folder(&folder) {
                            self.lib.add(build);
                        }
                    }
                }
                let added = self.lib.builds.len() - before;
                self.status = format!("Scan finished: {} added.", count_noun(added, "build"));
                if added > 0 {
                    self.save();
                }
            }
            Err(TryRecvError::Empty) => ctx.request_repaint_after(Duration::from_millis(250)),
            Err(TryRecvError::Disconnected) => self.scanning = None,
        }
    }

    fn play(&mut self, index: usize) {
        let build = self.lib.builds[index].clone();
        match launch::launch(&build) {
            Ok(child) => {
                self.status = format!("Started {}.", build.label);
                self.running.push(child);
            }
            Err(e) => self.status = e,
        }
    }
}

/// `1 build`, `2 builds`.
fn count_noun(n: usize, noun: &str) -> String {
    if n == 1 { format!("1 {noun}") } else { format!("{n} {noun}s") }
}

impl eframe::App for App {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = root.ctx().clone();

        // Folders dropped on the window.
        let dropped: Vec<PathBuf> = ctx.input(|i| {
            i.raw.dropped_files.iter().map(|f| f.path().to_path_buf()).filter(|p| p.is_dir()).collect()
        });
        for folder in dropped {
            self.add_folder(folder);
        }
        self.poll_scan(&ctx);
        // Reap finished games so the count stays honest.
        self.running.retain_mut(|c| !matches!(c.try_wait(), Ok(Some(_))));

        egui::Panel::top("header")
            .frame(egui::Frame::new().fill(theme::SURFACE).inner_margin(14.0))
            .show(root, |ui| {
                ui.horizontal(|ui| {
                    theme::mark(ui, 34.0);
                    ui.add_space(4.0);
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        ui.label(theme::wordmark("RETROVERSE").size(23.0));
                        ui.label(theme::label("BY NAVINE"));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add(theme::primary("Add build")).clicked() {
                            if let Some(folder) =
                                rfd::FileDialog::new().set_title("Select a build folder").pick_folder()
                            {
                                self.add_folder(folder);
                            }
                        }
                        let scanning = self.scanning.is_some();
                        let label = if scanning { "Scanning…" } else { "Scan folder" };
                        if ui.add_enabled(!scanning, theme::secondary(label)).clicked() {
                            if let Some(folder) =
                                rfd::FileDialog::new().set_title("Scan a folder for builds").pick_folder()
                            {
                                self.start_scan(folder);
                            }
                        }
                        if !self.running.is_empty() {
                            ui.add_space(10.0);
                            theme::pill(ui, format!("{} running", self.running.len()), theme::OK);
                        }
                    });
                });
            });

        egui::Panel::bottom("status")
            .frame(egui::Frame::new().fill(theme::SURFACE).inner_margin(8.0))
            .show(root, |ui| {
                ui.horizontal(|ui| {
                    ui.label(theme::muted(&self.status));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(theme::label(format!(
                            "{} IN YOUR LIBRARY",
                            count_noun(self.lib.builds.len(), "build").to_uppercase()
                        )));
                    });
                });
            });

        egui::CentralPanel::default().show(root, |ui| {
            theme::callout(ui, theme::ROYAL_PURPLE_BRIGHT, DEPRECATION);
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                for family in family::all() {
                    let game = family.game();
                    let text = format!("{} ({})", family.name(), self.lib.count(game));
                    if ui.add(theme::segment(&text, self.tab == game)).clicked() {
                        self.tab = game;
                    }
                }
            });
            ui.add_space(8.0);

            let indices: Vec<usize> = self
                .lib
                .builds
                .iter()
                .enumerate()
                .filter(|(_, b)| b.game == self.tab)
                .map(|(i, _)| i)
                .collect();

            if indices.is_empty() {
                theme::empty_state(
                    ui,
                    &format!("No {} builds yet", self.tab.name()),
                    "Add build, Scan folder, or drag a build folder onto this window",
                );
                return;
            }

            let mut play = None;
            let mut remove = None;
            let mut changed = false;
            egui::ScrollArea::vertical().show(ui, |ui| {
                for i in indices {
                    theme::card().show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        let missing = !self.lib.builds[i].exe.is_file();
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                let b = &self.lib.builds[i];
                                ui.label(RichText::new(&b.label).size(16.0).strong());
                                ui.label(theme::muted(format!("{} · {}", b.version, b.folder.display())).small());
                                if missing {
                                    ui.label(RichText::new("The executable is missing.").color(theme::DANGER).small());
                                }
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.add(theme::danger("Remove")).clicked() {
                                    remove = Some(i);
                                }
                                if ui.add_enabled(!missing, theme::primary("Play")).clicked() {
                                    play = Some(i);
                                }
                            });
                        });
                        egui::CollapsingHeader::new("Launch options").id_salt(("opts", i)).show(ui, |ui| {
                            let b = &mut self.lib.builds[i];
                            ui.horizontal(|ui| {
                                ui.label(theme::label("ARGUMENTS"));
                                changed |= ui
                                    .add(egui::TextEdit::singleline(&mut b.extra_args).desired_width(360.0).hint_text("-windowed"))
                                    .lost_focus();
                            });
                            ui.horizontal(|ui| {
                                ui.label(theme::label("RUN THROUGH"));
                                changed |= ui
                                    .add(egui::TextEdit::singleline(&mut b.wrapper).desired_width(360.0).hint_text("wine"))
                                    .lost_focus();
                            });
                        });
                    });
                    ui.add_space(6.0);
                }
            });

            if let Some(i) = play {
                self.play(i);
            }
            if let Some(i) = remove {
                self.lib.remove(i);
                changed = true;
            }
            if changed {
                self.save();
            }
        });
    }
}
