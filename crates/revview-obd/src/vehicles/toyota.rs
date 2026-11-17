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
pub const TOYOTA_HYBRID_PIDS: [ExtraPid; 0] = [];

pub const PRIUS_GEN3: VehicleProfile = VehicleProfile {
    id: "toyota_prius_gen3",
    name: "Toyota Prius (2010-2015)",
    redline_default: 5200,
    shift_default: 4800,
    hybrid: true,
    extra_pids: &TOYOTA_HYBRID_PIDS,
};
