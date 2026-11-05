// `\r` tells the adapter that the command is finished
const RPM_COMMAND: &str = "010C\r";
const SPEED_COMMAND: &str = "010D\r";
const COOLANT_TEMP_COMMAND: &str = "0105\r";
const INTAKE_TEMP_COMMAND: &str = "010F\r";

// Adapter commands
const RESET_ADAPTER_COMMAND: &str = "ATZ\r";
const ECHO_OFF_COMMAND: &str = "ATE0\r";
const AUTO_PROTOCOL_SELECT_COMMAND: &str = "ATSP0\r";
const IDENTIFY_ADAPTER_COMMAND: &str = "ATI\r";

// 41 0C 1F 40
// 41 = Positive response to Mode 01
// 0C = RPM PID
// 1F = Data byte A
// 40 = Data byte B
