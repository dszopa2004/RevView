// Defines OBD-II requests and adapter commands, including their associated PIDs.

use revview_core::diagnostics::CodeKind;

// `\r` tells the adapter that the command is finished
pub const RPM_COMMAND: &str = "010C\r";
pub const SPEED_COMMAND: &str = "010D\r";
pub const COOLANT_TEMP_COMMAND: &str = "0105\r";
pub const INTAKE_TEMP_COMMAND: &str = "010F\r";
pub const THROTTLE_POSITION_COMMAND: &str = "0111\r";

// Adapter commands
pub const RESET_ADAPTER_COMMAND: &str = "ATZ\r";
pub const ECHO_OFF_COMMAND: &str = "ATE0\r";
pub const AUTO_PROTOCOL_SELECT_COMMAND: &str = "ATSP0\r";
pub const IDENTIFY_ADAPTER_COMMAND: &str = "ATI\r";

// Engine code commands
pub const STORED_CODES_COMMAND: &str = "03\r";
pub const PENDING_CODES_COMMAND: &str = "07\r";
pub const PERMANENT_CODES_COMMAND: &str = "0A\r";

pub fn codes_command(kind: CodeKind) -> &'static str {
    match kind {
        CodeKind::Stored => STORED_CODES_COMMAND,
        CodeKind::Pending => PENDING_CODES_COMMAND,
        CodeKind::Permanent => PERMANENT_CODES_COMMAND,
    }
}

// 41 0C 1F 40
// 41 = Positive response to Mode 01
// 0C = RPM PID
// 1F = Data byte A
// 40 = Data byte B
