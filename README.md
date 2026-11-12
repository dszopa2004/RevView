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

## Adding a new setting

1. **`revview-core/src/settings.rs`**: add the field to `AppSettings` and its value in the `Default` impl. Add any new enum or min/max consts here too.
2. **`revview/src/main.rs` (startup)**: load the saved value into the UI with `app.set_<name>(...)`. If you skip this, the UI starts at 0 and the next save overwrites the stored value.
3. **`revview/src/main.rs` (`on_settings_changed`)**: read it back with `ui.get_<name>()`, convert it, clamp it, and put it in the `AppSettings` being saved.
4. **`dashboard.slint`**: add an `in-out property` and bind it to `Settings` with `<name> <=> root.<name>;`.
5. **`settings.slint`**: add an `in-out property` and a row with a control that calls `root.settings-changed()` when the value changes.
6. **Where it's used** (for example `speedometer.slint`): add an `in property`, pass it down from `Dashboard`, and use it.

Slint properties use kebab-case (`shift-rpm`), and the Rust getters and setters use snake_case (`get_shift_rpm`, `set_shift_rpm`).