use revview_core::vehicle::{ExtraPid, VehicleProfile};

/// Empty until verified against a real car. Template for an entry:
///
/// ExtraPid {
///     id: "hv_soc",
///     label: "Battery",
///     unit: "%",
///     header: Some("7E2"),
///     command: "2101\r",
///     decode: decode_hv_soc,
/// },
///
/// fn decode_hv_soc(data: &[u8]) -> Option<f32> {
///     let raw = *data.get(/* VERIFIED OFFSET */ 0)? as f32;
///     Some(raw * 100.0 / 255.0)   // VERIFIED SCALING
/// }
pub const HONDA_PIDS: [ExtraPid; 0] = [];

pub const CIVIC_GEN9: VehicleProfile = VehicleProfile {
    id: "honda_civic_gen9",
    name: "Honda Civic (2012-2015)",
    redline_default: 7000,
    shift_default: 4800,
    hybrid: false,
    extra_pids: &HONDA_PIDS,
};
