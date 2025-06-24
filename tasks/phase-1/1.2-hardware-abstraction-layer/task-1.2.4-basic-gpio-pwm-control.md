# Task 1.2.4: Basic GPIO PWM Control Structure

## Task Type

Implementation/Hardware

## Priority

Medium

## Story Points

4

## Summary

Implement the basic structure for controlling a PWM (Pulse-Width Modulation) output pin using the `rppal` crate. This will serve as the foundation for the real fan controller implementation on a Raspberry Pi.

## Description

This task involves creating the initial, real-world implementation of the `Fan` trait. This implementation will use the `rppal` library to interact with the Raspberry Pi's GPIO pins, specifically to configure a pin for PWM output and control its duty cycle. The entire implementation must be conditionally compiled, only activating when the `rpi-hardware` feature flag is enabled.

## Acceptance Criteria

- [ ] A `PwmFan` struct is created within the `backend/src/hardware/rpi.rs` file.
- [ ] The `PwmFan` struct correctly implements the `Fan` async_trait.
- [ ] The implementation successfully uses `rppal::pwm::Pwm` to manage a GPIO pin.
- [ ] The `set_duty_cycle` method correctly configures the PWM duty cycle on the specified pin.
- [ ] The `get_duty_cycle` method accurately returns the last set duty cycle.
- [ ] The code is enclosed in `#[cfg(feature = "rpi-hardware")]` attributes to ensure it only compiles for the Raspberry Pi target.
- [ ] Robust error handling is implemented for all GPIO operations that can fail, returning a clear and descriptive error.

## Technical Requirements

- **File Location**: `backend/src/hardware/rpi.rs`
- **Primary Library**: `rppal`
- **Functionality**:
  - The `PwmFan` struct should contain a `rppal::pwm::Pwm` instance.
  - A `new` function should be provided to initialize the `PwmFan`, accepting a GPIO pin number and a PWM frequency.
  - The `set_duty_cycle` method must take a floating-point number (`f64`) between 0.0 and 1.0 and correctly translate it to the range expected by the `rppal` library.
  - All potential errors from `rppal` should be properly handled and propagated using a custom error type.

## Implementation Steps

1.  Navigate to the `backend/src/hardware/rpi.rs` file.
2.  Define the `PwmFan` struct, which will hold the `Pwm` channel.
3.  Create a `new` function for `PwmFan` that initializes a PWM pin via the `rppal` library.
4.  Implement the `Fan` trait for the `PwmFan` struct.
5.  In `set_duty_cycle`, map the `f64` duty cycle (from 0.0-1.0) to the corresponding `rppal` PWM duty cycle value.
6.  Integrate error handling, preferably using `thiserror`, to wrap and manage potential errors from `rppal`.
7.  Confirm that the entire module is correctly configured for conditional compilation.

## Definition of Done

- The `PwmFan` implementation is complete and compiles successfully when the `rpi-hardware` feature is enabled.
- The code is prepared for integration testing on a physical Raspberry Pi.
- The implementation fully adheres to the `Fan` trait contract defined in Task 1.2.1.

## Dependencies

- Task 1.2.1: Design Trait-Based Hardware Interface
- Task 1.2.3: Add Conditional Compilation for Pi vs. Dev Environments

## Blocked By

None

## Related Tasks

- Task 3.2: Fan Control (PWM)
