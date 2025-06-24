# Task 1.2.1: Design Trait-Based Hardware Interface

## Task Type

Architecture/Design

## Priority

High

## Story Points

3

## Summary

Define a set of Rust traits that create a hardware abstraction layer for sensors and fans. This will decouple the core application logic from the specific hardware drivers.

## Description

To ensure the application is testable and can run on different platforms (Raspberry Pi vs. development machines), we need to define a clear boundary between the application logic and the hardware control code. This task involves designing and creating `async_trait`s for the primary hardware components: temperature sensors and fans.

## Acceptance Criteria

- [ ] A new module is created at `backend/src/hardware/mod.rs`.
- [ ] A `TemperatureSensor` trait is defined with methods for reading sensor data (e.g., `read_voltage`, `read_temperature_celsius`).
- [ ] A `Fan` trait is defined with methods for controlling fan speed (e.g., `set_duty_cycle`, `get_duty_cycle`).
- [ ] A `HardwareBridge` trait or struct is defined to act as a factory or container for the hardware implementations.
- [ ] The traits are well-documented with explanations of their purpose and methods.
- [ ] The traits use `async_trait` to support asynchronous operations.

## Technical Requirements

- **File Location**: `backend/src/hardware/mod.rs`
- **Traits**:
  - `TemperatureSensor`: Should be generic enough to support different ADC (Analog-to-Digital Converter) chips or sensor types. It needs to be `Send + Sync`.
  - `Fan`: Should abstract PWM (Pulse-Width Modulation) control. It needs to be `Send + Sync`.
- **Asynchronicity**: Use the `async_trait` crate for defining async methods within the traits.

## Implementation Steps

1.  Create the `backend/src/hardware/` directory and `mod.rs` file.
2.  Add the `async-trait` dependency to `backend/Cargo.toml` if not already present.
3.  Define the `TemperatureSensor` trait with the required async methods.
4.  Define the `Fan` trait with the required async methods.
5.  Define a top-level `HardwareBridge` that will provide access to the instantiated hardware components.
6.  Ensure the new module is correctly integrated into the crate by adding `pub mod hardware;` in `backend/src/lib.rs`.

## Definition of Done

- The trait definitions are complete and compile successfully.
- The new module is correctly exposed in the backend crate.
- The design is approved and ready for mock and real implementations.

## Dependencies

- Task 1.1.1: Create Cargo Workspace with Backend Crate

## Blocked By

None

## Related Tasks

- Task 1.2.2: Implement mock sensors/fans for development
- Task 1.2.4: Basic GPIO PWM control structure
