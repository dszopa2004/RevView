// `\r` tells the adapter that the command is finished
const RPM_COMMAND: &str = "010C\r";
const SPEED_COMMAND: &str = "010D\r";
const COOLANT_TEMP_COMMAND: &str = "0105\r";
const INTAKE_TEMP_COMMAND: &str = "010F\r";

// 41 0C 1F 40
// 41 = Positive response to Mode 01
// 0C = RPM PID
// 1F = Data byte A
// 40 = Data byte B
