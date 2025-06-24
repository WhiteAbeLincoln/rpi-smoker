# Task 1.2.2: Implement Mock Sensors/Fans for Development

## Task Type

Implementation/Testing

## Priority

High

## Story Points

5

## Summary

Create mock implementations of the `TemperatureSensor` and `Fan` traits. These mocks will allow for development and testing on machines without a Raspberry Pi's GPIO hardware, such as standard x86/x64 developer laptops.

## Description

The mock implementations should simulate the behavior of real hardware. For sensors, this means generating predictable or configurable data streams. For fans, it means storing and reporting the last set duty cycle. This allows the rest of the application to be developed and tested without needing physical hardware.

## Acceptance Criteria

- [ ] A `mock.rs` file is created in the `backend/src/hardware/` directory.
- [ ] A `MockTemperatureSensor` struct is created that implements the `TemperatureSensor` trait.
- [ ] The `MockTemperatureSensor` returns a configurable or predictable series of values.
- [ ] A `MockFan` struct is created that implements the `Fan` trait.
- [ ] The `MockFan` stores its state (duty cycle) in memory and provides methods to inspect it.
- [ ] The mock implementations are only compiled when a specific feature flag (e.g., `mock_hardware`) is enabled.
- [ ] Unit tests are added to verify the behavior of the mock implementations.

## Technical Requirements

- **File Location**: `backend/src/hardware/mock.rs`
- **Functionality**:
  - `MockTemperatureSensor`:
    - Can be configured to return a constant value.
    - Can be configured to return a value from a predefined sequence.
    - Should log its activity (e.g., "Reading from mock sensor").
  - `MockFan`:
    - `set_duty_cycle` updates an internal `Arc<Mutex<f64>>` or similar thread-safe field.
    - `get_duty_cycle` returns the current value of the internal field.
    - Should log its activity (e.g., "Setting mock fan duty cycle to X").
- **Conditional Compilation**: Use `#[cfg(feature = "mock_hardware")]` to control the compilation of the mock module.

## Implementation Steps

1.  Create the `backend/src/hardware/mock.rs` file.
2.  Implement the `MockTemperatureSensor` struct, deriving `Debug` and `Clone`.
3.  Implement the `async_trait` for `TemperatureSensor` on `MockTemperatureSensor`.
4.  Implement the `MockFan` struct, deriving `Debug` and `Clone`.
5.  Implement the `async_trait` for `Fan` on `MockFan`.
6.  Add unit tests to a `#[cfg(test)]` block within `mock.rs` to validate the mock objects' behavior.
7.  Wrap the entire `mock` module in a `cfg` attribute for conditional compilation.
8.  Update `backend/src/hardware/mod.rs` to include `pub mod mock;` under the correct `cfg` flag.

## Definition of Done

- Mock implementations are complete and fully tested.
- Mocks can be used in the application when the `mock_hardware` feature is enabled.
- The code is well-documented and follows project standards.

## Dependencies

- Task 1.2.1: Design Trait-Based Hardware Interface

## Blocked By

None

## Related Tasks

- Task 1.2.3: Add conditional compilation for Pi vs dev environments
- Task 6.1.2: Hardware mock integration tests
