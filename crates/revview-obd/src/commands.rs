// Defines OBD-II requests and adapter commands, including their associated PIDs.

// `\r` tells the adapter that the command is finished
pub const RPM_COMMAND: &str = "010C\r";
pub const SPEED_COMMAND: &str = "010D\r";
pub const COOLANT_TEMP_COMMAND: &str = "0105\r";
pub const INTAKE_TEMP_COMMAND: &str = "010F\r";

// Adapter commands
pub const RESET_ADAPTER_COMMAND: &str = "ATZ\r";
pub const ECHO_OFF_COMMAND: &str = "ATE0\r";
pub const AUTO_PROTOCOL_SELECT_COMMAND: &str = "ATSP0\r";
pub const IDENTIFY_ADAPTER_COMMAND: &str = "ATI\r";

// 41 0C 1F 40
// 41 = Positive response to Mode 01
// 0C = RPM PID
// 1F = Data byte A
// 40 = Data byte B
