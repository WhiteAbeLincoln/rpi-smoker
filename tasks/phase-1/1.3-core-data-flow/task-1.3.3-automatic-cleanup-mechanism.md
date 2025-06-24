# Task 1.3.3: Automatic Cleanup Mechanism

## Task Type

Implementation/Core Feature

## Priority

High

## Story Points

8

## Summary

Implement an automatic cleanup mechanism in the `DataStore` that periodically removes old data based on a time limit (7 hours) and a memory usage limit (1 GB). This ensures the application does not consume unbounded memory.

## Description

This task enhances the `DataStore` with a background process that enforces the data retention policy. The cleanup should be triggered both when new data is added and on a regular time interval. It will check against two limits: the age of the oldest data point and the total estimated memory usage of the stored readings.

## Acceptance Criteria

- [ ] The `DataStore` is configured with a `max_age` (`chrono::Duration`) and `max_memory_bytes` (`usize`).
- [ ] A private `cleanup` method is implemented within the `DataStore`.
- [ ] The `cleanup` method removes readings from the front of the `VecDeque` if they are older than `max_age`.
- [ ] The `cleanup` method removes the oldest 25% of readings if the estimated memory usage exceeds 80% of `max_memory_bytes`.
- [ ] The `add_reading` method is modified to trigger the `cleanup` method after each new entry.
- [ ] A separate, long-running async task is spawned to run the `cleanup` method periodically (e.g., every 5 minutes).
- [ ] Unit tests are added to verify that data is correctly purged based on both time and memory limits.

## Technical Requirements

- **File Location**: `backend/src/utils/data_store.rs`
- **Time Handling**: Use the `chrono` crate to manage timestamps and durations.
- **Memory Estimation**: The memory usage can be estimated by `std::mem::size_of::<SensorReading>() * self.readings.len()`.
- **Background Task**: Use `tokio::spawn` to run the periodic cleanup task in the background.
- **Configuration**: The `DataStore::new` function should accept the retention policy parameters.

## Implementation Steps

1.  Add `max_age` and `max_memory_bytes` fields to the `DataStore` struct.
2.  Implement the `cleanup` method, which contains the logic for both time-based and memory-based data removal.
3.  Modify the `add_reading` method to call `cleanup`.
4.  Create a new public method on `DataStore`, such as `start_periodic_cleanup`, that spawns the background `tokio` task.
5.  The background task should acquire a lock on the `DataStore` and call the `cleanup` method, then sleep for a configured interval.
6.  Add comprehensive unit tests for the cleanup logic. This may require a time-mocking library or careful test design to simulate the passage of time.

## Definition of Done

- The automatic cleanup mechanism is fully implemented and tested.
- The application's memory usage is effectively bounded by the configured limits.
- The cleanup process is efficient and does not block the main application threads for an extended period.

## Dependencies

- Task 1.3.2: Circular buffer implementation

## Blocked By

None

## Related Tasks

- Task 1.3.4: Memory usage monitoring and reporting
