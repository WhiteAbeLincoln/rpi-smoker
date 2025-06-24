# Task 1.2.3: Add Conditional Compilation for Pi vs. Dev Environments

## Task Type

Configuration/Build

## Priority

High

## Story Points

3

## Summary

Configure the `backend/Cargo.toml` and the application's hardware module to use feature flags for conditionally compiling either the real hardware drivers (for Raspberry Pi) or the mock implementations (for development).

## Description

To support a flexible development workflow, the project must be able to compile and run on different architectures. This task involves setting up Cargo feature flags to control which hardware implementation is active. The default configuration should be for development (mock hardware), while a specific feature flag should enable the Raspberry Pi GPIO drivers.

## Acceptance Criteria

- [ ] A `rpi-hardware` feature is defined in `backend/Cargo.toml`.
- [ ] The `rppal` dependency is marked as optional and tied to the `rpi-hardware` feature.
- [ ] The `mock-hardware` feature is defined and enabled by default.
- [ ] The `hardware` module uses `#[cfg(feature = "...")]` attributes to select the correct implementation (real or mock).
- [ ] The application compiles successfully with `cargo build` (using mocks).
- [ ] The application compiles successfully with `cargo build --no-default-features --features rpi-hardware`.
- [ ] The `HardwareBridge` or equivalent structure correctly initializes either the mock or real hardware based on the active feature flags.
- [ ] The `rpi-hardware` feature is not exclusive with the `mock-hardware` feature. Both can be enabled at the same time.

## Technical Requirements

- **`backend/Cargo.toml`**:

  ```toml
  [features]
  default = ["mock-hardware"]
  mock-hardware = []
  rpi-hardware = ["dep:rppal"]
  ```

- **Code Structure**:
  - The `hardware` module should contain logic that exposes either the `mock` or a new `rpi` submodule's implementations based on the active features.
  - Use `#[cfg(all(feature = "rpi-hardware"))]` for Pi-specific code.
  - Use `#[cfg(feature = "mock_hardware")]` for mock-specific code.

## Implementation Steps

1.  Modify `backend/Cargo.toml` to add the `features` section and make `rppal` optional.
2.  In `backend/src/hardware/mod.rs`, add `cfg` attributes to conditionally include the `mock` module and a new (currently empty) `rpi` module.
3.  Create a `backend/src/hardware/rpi.rs` file to house the real hardware implementations later.
4.  Implement the logic in the `HardwareBridge` to instantiate the correct hardware types based on the feature flags.
5.  Verify both build configurations work as expected.
6.  Document the build process for both environments in the project's `README.md`.

## Definition of Done

- The build system correctly handles both development and Raspberry Pi targets.
- The correct hardware implementation is chosen at compile time based on feature flags.
- The default build configuration is set for mock hardware to ensure a smooth developer onboarding experience.

## Dependencies

- Task 1.2.2: Implement mock sensors/fans for development

## Blocked By

None

## Related Tasks

- Task 3.1: Temperature Sensors (ADS1115)
- Task 3.2: Fan Control (PWM)
