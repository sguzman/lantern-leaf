use eframe::egui::{self, Color32, ComboBox, Image, Label, ScrollArea, Ui, Vec2};
use lanternleaf_app::pipeline::AppCommand;
use lanternleaf_app::state::AppState;
use tracing::{trace, warn};

use std::path::PathBuf;

use crate::app::ui::format::{format_bytes, format_relative_unix_secs};
use crate::app::{
    CalibreSort, LanternLeafApp, StarterViewModel, THUMB_HEIGHT, THUMB_ROW_HEIGHT, THUMB_WIDTH,
};

const STARTER_TWO_COLUMN_MIN_WIDTH: f32 = 1120.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StarterLayoutMode {
    OneColumn,
    TwoColumn,
}

fn starter_layout_mode(available_width: f32) -> StarterLayoutMode {
    if available_width >= STARTER_TWO_COLUMN_MIN_WIDTH {
        StarterLayoutMode::TwoColumn
    } else {
        StarterLayoutMode::OneColumn
    }
}

fn starter_text_capacity(available_width: f32) -> usize {
    (available_width.max(1.0) / 7.5).floor().max(1.0) as usize
}

fn starter_text_needs_wrapping(text: &str, available_width: f32) -> bool {
    text.chars().count() > starter_text_capacity(available_width)
}

impl LanternLeafApp {
    pub(crate) fn render_starter_content(&mut self, ui: &mut Ui, state: &AppState) {
        let model = StarterViewModel::from_state(state);
        ui.heading("Starter shell");
        ui.add_space(8.0);
        let available_width = ui.available_width();
        match starter_layout_mode(available_width) {
            StarterLayoutMode::TwoColumn => {
                ui.columns(2, |columns| {
                    self.render_starter_open_controls(&mut columns[0], &model);
                    self.render_starter_recents(&mut columns[0], &model);
                    self.render_starter_calibre(&mut columns[1], &model);
                    self.render_starter_browser_tabs(&mut columns[1], &model);
                });
            }
            StarterLayoutMode::OneColumn => {
                self.render_starter_open_controls(ui, &model);
                self.render_starter_recents(ui, &model);
                self.render_starter_calibre(ui, &model);
                self.render_starter_browser_tabs(ui, &model);
            }
        }
        ui.add_space(8.0);
        self.render_starter_diagnostics(ui, &model);
    }

    fn render_starter_open_controls(&mut self, ui: &mut Ui, model: &StarterViewModel<'_>) {
        ui.group(|ui| {
            ui.set_min_width(0.0);
            ui.set_max_width(ui.available_width());
            ui.label("Open source");
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.starter_open_path_input)
                        .desired_width((ui.available_width() - 90.0).max(120.0))
                        .hint_text("Path to file"),
                );
                if ui.button("Open file").clicked() {
                    let path = self.starter_open_path_input.trim().to_string();
                    if path.is_empty() {
                        warn!("Starter open path empty");
                        self.push_status("Starter: open path is empty".to_string());
                    } else {
                        trace!(path = %path, "Starter open path");
                        self.execute_command(AppCommand::OpenSourcePath { path });
                    }
                }
            });
            ui.horizontal_wrapped(|ui| {
                if ui.button("Open clipboard").clicked() {
                    trace!("Starter open clipboard");
                    self.execute_command(AppCommand::OpenClipboard);
                }
                if model.operations.source_open {
                    ui.label("Opening…");
                }
            });
            ui.add_space(6.0);
            ui.label("Open clipboard text");
            ui.add(
                egui::TextEdit::multiline(&mut self.starter_clipboard_text_input)
                    .desired_width(ui.available_width())
                    .hint_text("Paste text to open")
                    .desired_rows(3),
            );
            if ui.button("Open clipboard text").clicked() {
                let text = self.starter_clipboard_text_input.trim().to_string();
                if text.is_empty() {
                    warn!("Starter clipboard text empty");
                    self.push_status("Starter: clipboard text is empty".to_string());
                } else {
                    trace!(bytes = text.len(), "Starter open clipboard text");
                    self.execute_command(AppCommand::OpenClipboardText { text });
                }
            }
        });
    }

    fn render_starter_recents(&mut self, ui: &mut Ui, model: &StarterViewModel<'_>) {
        ui.add_space(8.0);
        ui.group(|ui| {
            ui.set_min_width(0.0);
            ui.set_max_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                ui.label("Recents");
                if ui.button("Refresh").clicked() {
                    trace!("Starter refresh recents");
                    self.execute_command(AppCommand::RefreshRecents { limit: Some(30) });
                }
                if model.loading_recents || model.operations.starter_command {
                    ui.label("Loading…");
                }
            });
            if model.recents.is_empty() && !model.loading_recents {
                ui.label("No recent books yet.");
                return;
            }
            ScrollArea::vertical()
                .id_source("starter_recents_scroll")
                .max_height(260.0)
                .show(ui, |ui| {
                    for recent in model.recents {
                        ui.separator();
                        ui.horizontal_top(|ui| {
                            self.render_thumbnail(
                                ui,
                                recent.thumbnail_path.as_deref().map(PathBuf::from),
                            );
                            ui.vertical(|ui| {
                                ui.set_min_width(0.0);
                                ui.set_max_width(ui.available_width());
                                ui.horizontal_wrapped(|ui| {
                                    ui.add(Label::new(&recent.display_title).wrap(
                                        starter_text_needs_wrapping(
                                            &recent.display_title,
                                            ui.available_width(),
                                        ),
                                    ));
                                    ui.add_space(6.0);
                                    ui.label(format_relative_unix_secs(
                                        recent.last_opened_unix_secs,
                                    ));
                                });
                                ui.add(Label::new(&recent.snippet).wrap(
                                    starter_text_needs_wrapping(
                                        &recent.snippet,
                                        ui.available_width(),
                                    ),
                                ));
                                ui.add(Label::new(&recent.source_path).truncate(true));
                                if let Some(tab_id) = recent.browser_tab_id {
                                    ui.label(format!("Browser tab: {}", tab_id));
                                }
                                ui.horizontal_wrapped(|ui| {
                                    if ui.button("Open").clicked() {
                                        trace!(path = %recent.source_path, "Starter open recent");
                                        self.execute_command(AppCommand::OpenSourcePath {
                                            path: recent.source_path.clone(),
                                        });
                                    }
                                    if ui.button("Delete").clicked() {
                                        let close_browser_tab = model
                                            .bootstrap
                                            .map(|bootstrap| {
                                                bootstrap.config.close_browser_tab_on_recent_delete
                                            })
                                            .unwrap_or(false)
                                            && recent.browser_tab_id.is_some();
                                        trace!(
                                            path = %recent.source_path,
                                            close_browser_tab,
                                            "Starter delete recent"
                                        );
                                        self.execute_command(AppCommand::DeleteRecent {
                                            source_path: recent.source_path.clone(),
                                            close_browser_tab,
                                        });
                                    }
                                });
                            });
                        });
                    }
                });
        });
    }

    fn render_starter_calibre(&mut self, ui: &mut Ui, model: &StarterViewModel<'_>) {
        ui.group(|ui| {
            ui.set_min_width(0.0);
            ui.set_max_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                ui.label("Calibre");
                if ui.button("Refresh").clicked() {
                    trace!(
                        force = self.starter_calibre_force_refresh,
                        "Starter refresh Calibre"
                    );
                    self.execute_command(AppCommand::LoadCalibreBooks {
                        force_refresh: self.starter_calibre_force_refresh,
                    });
                }
                ui.checkbox(&mut self.starter_calibre_force_refresh, "Force refresh");
                if model.loading_calibre || model.operations.calibre_load {
                    ui.label("Loading…");
                }
                if model.operations.source_open {
                    ui.label("Opening book…");
                }
            });
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.starter_calibre_query)
                        .desired_width((ui.available_width() - 110.0).max(120.0))
                        .hint_text("Search title or author"),
                );
                ComboBox::from_id_source("calibre_sort")
                    .selected_text(self.starter_calibre_sort.label())
                    .show_ui(ui, |ui| {
                        for option in CalibreSort::OPTIONS {
                            ui.selectable_value(
                                &mut self.starter_calibre_sort,
                                option,
                                option.label(),
                            );
                        }
                    });
            });
            if model.calibre_books.is_empty() && !model.loading_calibre {
                ui.label("No Calibre books loaded.");
                return;
            }
            let query = self.starter_calibre_query.trim().to_lowercase();
            let should_rebuild = self.starter_calibre_last_query != query
                || self.starter_calibre_last_sort != self.starter_calibre_sort
                || self.starter_calibre_last_count != model.calibre_books.len();
            if should_rebuild {
                #[derive(Clone)]
                struct CalibreViewEntry {
                    idx: usize,
                    title_lower: String,
                    authors_lower: String,
                    year: Option<i32>,
                }

                self.starter_calibre_last_query = query.clone();
                self.starter_calibre_last_sort = self.starter_calibre_sort;
                self.starter_calibre_last_count = model.calibre_books.len();

                let mut entries: Vec<CalibreViewEntry> = model
                    .calibre_books
                    .iter()
                    .enumerate()
                    .map(|(idx, book)| CalibreViewEntry {
                        idx,
                        title_lower: book.title.to_lowercase(),
                        authors_lower: book.authors.to_lowercase(),
                        year: book.year,
                    })
                    .collect();
                if !query.is_empty() {
                    entries.retain(|entry| {
                        entry.title_lower.contains(&query) || entry.authors_lower.contains(&query)
                    });
                }
                match self.starter_calibre_sort {
                    CalibreSort::Title => entries.sort_by(|a, b| a.title_lower.cmp(&b.title_lower)),
                    CalibreSort::Author => {
                        entries.sort_by(|a, b| a.authors_lower.cmp(&b.authors_lower))
                    }
                    CalibreSort::Year => entries.sort_by(|a, b| {
                        b.year
                            .cmp(&a.year)
                            .then_with(|| a.title_lower.cmp(&b.title_lower))
                    }),
                }
                self.starter_calibre_view = entries.into_iter().map(|entry| entry.idx).collect();
                trace!(
                    total = model.calibre_books.len(),
                    visible = self.starter_calibre_view.len(),
                    "Rebuilt calibre list view"
                );
            }

            let row_height = THUMB_ROW_HEIGHT;
            let total_rows = self.starter_calibre_view.len();
            ScrollArea::vertical()
                .id_source("starter_calibre_scroll")
                .max_height(240.0)
                .show_rows(ui, row_height, total_rows, |ui, range| {
                    for row in range {
                        let book = &model.calibre_books[self.starter_calibre_view[row]];
                        ui.separator();
                        ui.horizontal_top(|ui| {
                            self.render_thumbnail(
                                ui,
                                book.cover_thumbnail.as_deref().map(PathBuf::from),
                            );
                            ui.vertical(|ui| {
                                ui.set_min_width(0.0);
                                ui.set_max_width(ui.available_width());
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(&book.title);
                                    if let Some(year) = book.year {
                                        ui.label(format!("({year})"));
                                    }
                                });
                                ui.add(Label::new(&book.authors).wrap(true));
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(format!(
                                        "{} • {}",
                                        book.extension,
                                        format_bytes(book.file_size_bytes)
                                    ));
                                    if book.cover_thumbnail.is_some() {
                                        ui.label("Thumbnail cached");
                                    }
                                });
                                ui.horizontal_wrapped(|ui| {
                                    if ui
                                        .add_enabled(
                                            !model.operations.source_open,
                                            egui::Button::new("Open"),
                                        )
                                        .clicked()
                                    {
                                        trace!(id = book.id, "Starter open Calibre book");
                                        self.execute_command(AppCommand::OpenCalibreBook {
                                            book: book.clone(),
                                        });
                                    }
                                    if ui.button("Ensure thumbnail").clicked() {
                                        trace!(id = book.id, "Starter ensure Calibre thumbnail");
                                        self.execute_command(AppCommand::EnsureCalibreThumbnail {
                                            id: book.id,
                                        });
                                    }
                                });
                            });
                        });
                    }
                });
        });
    }

    fn render_starter_browser_tabs(&mut self, ui: &mut Ui, model: &StarterViewModel<'_>) {
        ui.add_space(8.0);
        ui.group(|ui| {
            ui.set_min_width(0.0);
            ui.set_max_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                ui.label("Browser tabs");
                if ui.button("Health").clicked() {
                    trace!("Starter load browser tabs health");
                    self.execute_command(AppCommand::LoadBrowserTabsHealth);
                }
                if ui.button("List windows").clicked() {
                    trace!("Starter load browser tab windows");
                    self.execute_command(AppCommand::ListBrowserTabWindows);
                }
                if ui.button("List tabs").clicked() {
                    let window_id = self
                        .starter_browser_window_id_input
                        .trim()
                        .parse::<u64>()
                        .ok();
                    let query = self.starter_browser_tab_query.trim();
                    trace!(window_id = ?window_id, query = %query, refresh = self.starter_browser_tabs_force_refresh, "Starter load browser tabs");
                    self.execute_command(AppCommand::ListBrowserTabs {
                        window_id,
                        query: if query.is_empty() { None } else { Some(query.to_string()) },
                        refresh: self.starter_browser_tabs_force_refresh,
                    });
                }
                ui.checkbox(&mut self.starter_browser_tabs_force_refresh, "Force refresh");
                if model.loading_browser_tabs || model.operations.browser_tab_refresh {
                    ui.label("Loading…");
                }
            });
            let browser_tabs_enabled = model
                .bootstrap
                .map(|bootstrap| bootstrap.config.browser_tabs_enabled)
                .unwrap_or(true);
            if !browser_tabs_enabled {
                ui.colored_label(Color32::YELLOW, "Browser tabs are disabled in config.");
            }
            match model.browser_tabs_health {
                Some(health) => {
                    if !health.ok {
                        ui.colored_label(Color32::RED, "Browser tabs service offline.");
                    } else if !health.extension_connected {
                        ui.colored_label(Color32::YELLOW, "Browser extension disconnected.");
                    } else {
                        ui.label("Browser tabs service healthy.");
                    }
                }
                None => {
                    ui.label("Browser tabs health unknown.");
                }
            }
            if !model.browser_tabs_windows.is_empty() {
                let mut selected_window = self
                    .starter_browser_window_id_input
                    .trim()
                    .parse::<u64>()
                    .ok();
                ComboBox::from_id_source("browser_window_select")
                    .selected_text(
                        selected_window
                            .map(|id| format!("Window {}", id))
                            .unwrap_or_else(|| "All windows".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut selected_window, None, "All windows");
                        for window in model.browser_tabs_windows {
                            let label = if window.focused {
                                format!("Window {} (focused)", window.id)
                            } else {
                                format!("Window {}", window.id)
                            };
                            ui.selectable_value(&mut selected_window, Some(window.id), label);
                        }
                    });
                match selected_window {
                    Some(id) => self.starter_browser_window_id_input = id.to_string(),
                    None => self.starter_browser_window_id_input.clear(),
                }
            } else {
                ui.add(
                    egui::TextEdit::singleline(&mut self.starter_browser_window_id_input)
                        .hint_text("Window id (optional)"),
                );
            }
            ui.add(
                egui::TextEdit::singleline(&mut self.starter_browser_tab_id_input)
                    .desired_width(ui.available_width())
                    .hint_text("Tab id"),
            );
            ui.add(
                egui::TextEdit::singleline(&mut self.starter_browser_tab_query)
                    .desired_width(ui.available_width())
                    .hint_text("Search/filter tabs"),
            );
            ui.horizontal_wrapped(|ui| {
                if ui.button("Open tab").clicked() {
                    self.dispatch_browser_tab_open(false);
                }
                if ui.button("Import bundle").clicked() {
                    self.dispatch_browser_tab_open(true);
                }
                if ui.button("Refresh tab").clicked() {
                    self.dispatch_browser_tab_refresh();
                }
            });
            if model.browser_tabs_tabs.is_empty() && !model.loading_browser_tabs {
                ui.label("No browser tabs loaded.");
                return;
            }
            let query = self.starter_browser_tab_query.trim().to_lowercase();
            ScrollArea::vertical()
                .id_source("starter_browser_tabs_scroll")
                .max_height(220.0)
                .show(ui, |ui| {
                    for tab in model.browser_tabs_tabs {
                        if !query.is_empty()
                            && !tab.title.to_lowercase().contains(&query)
                            && !tab.url.to_lowercase().contains(&query)
                        {
                            continue;
                        }
                        ui.separator();
                        ui.horizontal_wrapped(|ui| {
                            ui.add(
                                Label::new(&tab.title).wrap(starter_text_needs_wrapping(
                                    &tab.title,
                                    ui.available_width(),
                                )),
                            );
                            if tab.active.unwrap_or(false) {
                                ui.label("(active)");
                            }
                        });
                        ui.add(Label::new(&tab.url).truncate(true));
                        ui.label(format!("Tab {} • Window {}", tab.id, tab.window_id));
                        ui.horizontal_wrapped(|ui| {
                            if ui.button("Open").clicked() {
                                trace!(tab_id = tab.id, window_id = tab.window_id, "Starter open browser tab from list");
                                self.execute_command(AppCommand::OpenBrowserTab {
                                    tab_id: tab.id,
                                    window_id: Some(tab.window_id),
                                });
                            }
                            if ui.button("Import bundle").clicked() {
                                trace!(tab_id = tab.id, window_id = tab.window_id, "Starter import browser tab bundle from list");
                                self.execute_command(AppCommand::OpenBrowserTabBundle {
                                    tab_id: tab.id,
                                    window_id: Some(tab.window_id),
                                });
                            }
                            if ui.button("Refresh").clicked() {
                                trace!(tab_id = tab.id, window_id = tab.window_id, "Starter refresh browser tab from list");
                                self.execute_command(AppCommand::RefreshBrowserTab {
                                    tab_id: tab.id,
                                    window_id: Some(tab.window_id),
                                });
                            }
                        });
                    }
                });
        });
    }

    fn render_starter_diagnostics(&mut self, ui: &mut Ui, model: &StarterViewModel<'_>) {
        ui.group(|ui| {
            ui.set_min_width(0.0);
            ui.set_max_width(ui.available_width());
            ui.label("Starter diagnostics");
            if let Some(event) = model.source_open_event {
                ui.label(format!(
                    "Source open: {} ({})",
                    event.phase,
                    event
                        .message
                        .clone()
                        .unwrap_or_else(|| "no message".to_string())
                ));
            }
            if let Some(event) = model.calibre_load_event {
                ui.label(format!(
                    "Calibre load: {} (count {:?})",
                    event.phase, event.count
                ));
            }
            if model.operations.source_open {
                ui.label("Source open in progress");
            }
            if model.operations.calibre_load {
                ui.label("Calibre load in progress");
            }
            if model.operations.browser_tab_refresh {
                ui.label("Browser tab refresh in progress");
            }

            ui.separator();
            if let Some(url) = model.remote_url {
                ui.label(format!("Remote sync: {}", url));
                if model.last_remote_update_at > 0 {
                    ui.label(format!(
                        "Last update: {} ms (epoch)",
                        model.last_remote_update_at
                    ));
                } else {
                    ui.label("No remote updates received yet.");
                }
            } else {
                ui.label("Remote sync: disabled");
            }
        });
    }

    fn dispatch_browser_tab_refresh(&mut self) {
        let tab_id = match self.starter_browser_tab_id_input.trim().parse::<u64>() {
            Ok(id) => id,
            Err(_) => {
                warn!("Invalid browser tab id");
                self.push_status("Starter: invalid browser tab id".to_string());
                return;
            }
        };
        let window_id = self
            .starter_browser_window_id_input
            .trim()
            .parse::<u64>()
            .ok();
        trace!(tab_id, window_id = ?window_id, "Starter refresh browser tab");
        self.execute_command(AppCommand::RefreshBrowserTab { tab_id, window_id });
    }

    fn render_thumbnail(&mut self, ui: &mut Ui, path: Option<PathBuf>) {
        let size = Vec2::new(THUMB_WIDTH as f32, THUMB_HEIGHT as f32);
        if let Some(path) = path {
            if let Some(texture) = self.thumbnail_cache.texture_for(ui.ctx(), &path) {
                ui.add(Image::new(&texture).fit_to_exact_size(size));
                return;
            }
        }
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
        let painter = ui.painter();
        painter.rect_filled(rect, 2.0, Color32::from_gray(24));
        painter.rect_stroke(rect, 2.0, (1.0, Color32::from_gray(60)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starter_breakpoint_is_stable_and_uses_actual_center_width() {
        assert_eq!(starter_layout_mode(1119.0), StarterLayoutMode::OneColumn);
        assert_eq!(
            starter_layout_mode(STARTER_TWO_COLUMN_MIN_WIDTH - 0.01),
            StarterLayoutMode::OneColumn
        );
        assert_eq!(
            starter_layout_mode(STARTER_TWO_COLUMN_MIN_WIDTH),
            StarterLayoutMode::TwoColumn
        );
        assert_eq!(starter_layout_mode(1400.0), StarterLayoutMode::TwoColumn);
    }

    #[test]
    fn long_starter_content_wraps_inside_a_measured_owned_group() {
        let ctx = egui::Context::default();
        let long_path = "C:\\library\\a-very-long-title\\".repeat(18);
        let long_title = "A very long title with many words ".repeat(12);
        assert!(starter_text_needs_wrapping(&long_path, 260.0));
        assert!(!starter_text_needs_wrapping("Short title", 260.0));

        let mut group_rect = None;
        let mut label_rect = None;
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.set_min_width(260.0);
                    ui.set_max_width(260.0);
                    let group = ui.group(|ui| {
                        ui.set_min_width(0.0);
                        ui.set_max_width(ui.available_width());
                        let response = ui.add(Label::new(&long_title).wrap(true));
                        label_rect = Some(response.rect);
                    });
                    group_rect = Some(group.response.rect);
                });
            },
        );

        let group_rect = group_rect.expect("starter group should be laid out");
        let label_rect = label_rect.expect("long content should be laid out");
        assert!(label_rect.width() <= group_rect.width());
        assert!(label_rect.right() <= group_rect.right());
        assert!(label_rect.height() >= 18.0);
    }
}
