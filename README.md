# RevView

## Tools for Development
- Slint (Extension)
- rust-analyzer (Extension)

## How Files Interact
1. `connection.rs` - Receives serial data from the adapter
2. `parser.rs` - Decodes hexadecimal OBD-II responses
3. `telemetry.rs` - Stores the resulting vehicle measurements
4. `main.rs` + `dashboard.slint` - Presents the measurements to the user

`calculations.rs` and `parameters.rs` provide supporting functionality to the other components.