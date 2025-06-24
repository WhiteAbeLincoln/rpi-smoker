# Task 1.3.1: Implement In-Memory Data Store with Retention Policy

## Task Type

Implementation/Core Feature

## Priority

High

## Story Points

5

## Summary

Create a thread-safe, in-memory data store for holding time-series `SensorReading` data. This component will be the central in-memory database for the application's real-time data.

## Description

This task involves creating the `DataStore` struct, which will manage all sensor readings. It must be designed for high-frequency, concurrent writes from multiple sensor polling tasks and reads from API handlers. The initial implementation will focus on the core data structure and thread-safe access, while subsequent tasks will add the circular buffer and cleanup logic.

## Acceptance Criteria

- [ ] A new file is created at `backend/src/utils/data_store.rs`.
- [ ] A `DataStore` struct is defined to hold a collection of `SensorReading` objects.
- [ ] Use any appropriate rust library for concurrent data stores.
- [ ] An `add_reading(&self, reading: SensorReading)` method is implemented to insert new data.
- [ ] A `get_all_readings(&self) -> Vec<SensorReading>` method is implemented to retrieve a copy of all current data.
- [ ] The new module is correctly integrated into the project (`backend/src/utils/mod.rs` and `lib.rs`).
- [ ] Basic unit tests are added to verify adding and retrieving readings.

## Technical Requirements

- **File Location**: `backend/src/utils/data_store.rs`
- **Data Structure**: The initial implementation can use a `Vec<SensorReading>`. This will be replaced by a `VecDeque` in a subsequent task.

## Implementation Steps

1.  Create the `backend/src/utils/` directory and its `mod.rs` file if they don't exist.
2.  Create the `data_store.rs` file.
3.  Define the `SensorReading` struct (it can be moved to a shared models file later).
4.  Define the `DataStore` struct.
5.  Implement the `new` function and the `add_reading`/`get_all_readings` methods.
6.  Write unit tests to instantiate the store, add a few readings, and verify they can be retrieved.
7.  Ensure the `utils` module is declared in `lib.rs`.

## Definition of Done

- The `DataStore` struct and its basic methods are implemented and tested.
- The implementation is thread-safe and ready for integration with the sensor polling loop and API.
- The code is well-documented and follows Rust best practices.

## Dependencies

- Task 1.1.4: Create basic data models and validation

## Blocked By

None

## Related Tasks

- Task 1.3.2: Circular buffer implementation
- Task 1.3.5: Basic sensor polling loop
