/// A manufacturer-specific value read from a non-generic request.
pub struct ExtraPid {
    pub id: &'static str,    // stable key, e.g. "hv_soc"
    pub label: &'static str, // shown in the UI
    pub unit: &'static str,
    pub header: Option<&'static str>, // e.g. Some("7E2") for the hybrid ECU
    pub command: &'static str,        // e.g. "2101\r"
    /// Receives the bytes after the response mode byte. For service 21,
    /// data[0] is the echoed PID, so offsets start from there.
    pub decode: fn(&[u8]) -> Option<f32>,
}

pub struct VehicleProfile {
    pub id: &'static str,   // saved in settings; never change it
    pub name: &'static str, // shown in the dropdown
    pub redline_default: u32,
    pub shift_default: u32,
    pub hybrid: bool, // enables the "EV" indicator
    pub extra_pids: &'static [ExtraPid],
}

pub const GENERIC: VehicleProfile = VehicleProfile {
    id: "generic",
    name: "Generic OBD-II",
    redline_default: 6500,
    shift_default: 6000,
    hybrid: false,
    extra_pids: &[],
};
