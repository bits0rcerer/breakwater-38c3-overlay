use breakwater_egui_overlay::{
    DynamicOverlay, eframe,
    egui::{self, Margin},
};

/// breakwater expects this function to report version information
#[unsafe(no_mangle)]
pub extern "C" fn versions() -> breakwater_egui_overlay::Versions {
    breakwater_egui_overlay::VERSIONS
}

/// breakwater expects this function to return our overlay
#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn new() -> breakwater_egui_overlay::DynamicOverlay {
    OVERLAY
}

/// color scheme from 38c3
#[allow(dead_code)]
mod colors {
    use breakwater_egui_overlay::egui::Color32;

    pub const COLOR_PRIMARY: Color32 = Color32::from_rgb(0xFF, 0x50, 0x53);
    pub const COLOR_HIGHLIGHT: Color32 = Color32::from_rgb(0xFE, 0xF2, 0xFF);
    pub const COLOR_ACCENT_A: Color32 = Color32::from_rgb(0xB2, 0xAA, 0xFF);
    pub const COLOR_ACCENT_B: Color32 = Color32::from_rgb(0x6A, 0x5F, 0xDB);
    pub const COLOR_ACCENT_C: Color32 = Color32::from_rgb(0x29, 0x11, 0x4C);
    pub const COLOR_ACCENT_D: Color32 = Color32::from_rgb(0x26, 0x1A, 0x66);
    pub const COLOR_ACCENT_E: Color32 = Color32::from_rgb(0x19, 0x0B, 0x2F);
    pub const COLOR_BACKGROUND: Color32 = Color32::from_rgb(0x0F, 0x00, 0x0A);
}

/// create a static function pointer for our setup function
static NEW_FN: breakwater_egui_overlay::New = ui_new as _;
extern "C" fn ui_new(_data: *mut std::ffi::c_void) {
    println!("Hello breakwater");
}

/// create a static function pointer for our teardown function
static DROP_FN: breakwater_egui_overlay::Drop = drop as _;
extern "C" fn drop(_: *mut std::ffi::c_void) {
    println!("Goodbye breakwater");
}

pub const OVERLAY: DynamicOverlay = DynamicOverlay {
    data: std::ptr::null_mut(),
    new: &raw const NEW_FN,
    draw_ui: draw_ui as _,
    drop: &raw const DROP_FN,
};

#[allow(improper_ctypes_definitions)]
extern "C" fn draw_ui(
    _: *mut std::ffi::c_void,
    viewport_idx: u32,
    ctx: &egui::Context,
    _advertised_endpoints: &[String],
    connections: u32,
    ips: u32,
    legacy_ips: u32,
    bytes_per_s: u64,
) {
    use colors::*;

    if viewport_idx > 0 {
        return;
    }

    let stats_frame = egui::Frame {
        fill: COLOR_BACKGROUND.gamma_multiply(0.7),
        stroke: egui::Stroke::new(1.0, COLOR_PRIMARY),
        corner_radius: egui::CornerRadius::same(10),
        shadow: eframe::epaint::Shadow::default(),
        inner_margin: Margin::same(12),
        outer_margin: Margin::same(12),
    };

    egui::Area::new(egui::Id::new("overlay_area"))
        .movable(true)
        .fixed_pos(egui::pos2(20.0, 20.0)) // Initial position on the screen
        .show(ctx, |ui| {
            stats_frame.show(ui, |ui| {
                ui.label(
                    egui::RichText::new("38c3 Pixelflut")
                        .size(48.0)
                        .color(COLOR_HIGHLIGHT),
                );
                ui.separator();

                egui::Grid::new(egui::Id::new("stats_header_grid")).show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("How to play: ")
                            .color(COLOR_HIGHLIGHT)
                            .size(32.0),
                    );
                    ui.label(
                        egui::RichText::new("c3pixelflut.de")
                            .color(COLOR_HIGHLIGHT)
                            .size(32.0)
                            .strong(),
                    );
                    ui.end_row();
                    ui.label(
                        egui::RichText::new("Flut from 🌍: ")
                            .color(COLOR_HIGHLIGHT)
                            .size(32.0),
                    );
                    ui.label(
                        egui::RichText::new("tcp://wall.c3pixelflut.de:1337")
                            .color(COLOR_HIGHLIGHT)
                            .size(32.0)
                            .strong(),
                    );
                    ui.end_row();
                    ui.label(
                        egui::RichText::new("Flut locally: ")
                            .color(COLOR_HIGHLIGHT)
                            .size(32.0),
                    );
                    ui.label(
                        egui::RichText::new("tcp://table.c3pixelflut.de:1337")
                            .color(COLOR_HIGHLIGHT)
                            .size(32.0)
                            .strong(),
                    );
                });

                ui.separator();
                egui::Grid::new(egui::Id::new("stats_metrics_grid")).show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Connections: ")
                            .color(COLOR_HIGHLIGHT)
                            .size(24.0),
                    );
                    ui.label(
                        egui::RichText::new(format!("{}", connections))
                            .color(COLOR_HIGHLIGHT)
                            .size(24.0)
                            .strong(),
                    );
                    ui.label(
                        egui::RichText::new("Distinct IPs: ")
                            .color(COLOR_HIGHLIGHT)
                            .size(24.0),
                    );
                    ui.label(
                        egui::RichText::new(format!("{}", legacy_ips + ips))
                            .color(COLOR_HIGHLIGHT)
                            .size(24.0)
                            .strong(),
                    );
                    ui.end_row();
                    ui.label(
                        egui::RichText::new("RX: ")
                            .color(COLOR_HIGHLIGHT)
                            .size(24.0),
                    );
                    ui.label(
                        egui::RichText::new(format!(
                            "{:.2} GBit/s       ",
                            (bytes_per_s * 8) as f32 / 1024.0 / 1024.0 / 1024.0
                        ))
                        .color(COLOR_HIGHLIGHT)
                        .size(24.0)
                        .strong(),
                    );
                    ui.label(
                        egui::RichText::new("<3 NOC <3")
                            .color(COLOR_HIGHLIGHT)
                            .size(24.0),
                    );
                });
            });
        });
}
