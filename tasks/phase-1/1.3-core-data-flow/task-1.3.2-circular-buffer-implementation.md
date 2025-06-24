# TODO: Manual refinement

Need to see what must change if we use an off-the-shelf library for the concurrent data store.

# Task 1.3.2: Circular Buffer Implementation

## Task Type

Implementation/Data Structure

## Priority

High

## Story Points

5

## Summary

Modify the `DataStore` to use a `VecDeque` (a double-ended queue) to function as a circular buffer. This is a more efficient data structure for managing a fixed-size collection of time-series data where old entries are frequently removed.

## Description

This task builds upon the initial `DataStore` implementation. The internal `Vec<SensorReading>` will be replaced with a `VecDeque<SensorReading>`. This change is critical for efficiently implementing the data retention policy, as removing elements from the front of a `VecDeque` is a constant-time operation (O(1)), whereas for a `Vec` it is linear (O(n)).

## Acceptance Criteria

- [ ] The `DataStore`'s internal storage is changed from `Vec<SensorReading>` to `VecDeque<SensorReading>`.
- [ ] The `add_reading` method is updated to use `push_back` on the `VecDeque`.
- [ ] A `remove_oldest_reading` method is added that uses `pop_front`.
- [ ] The `get_all_readings` method is updated to correctly return a `Vec<SensorReading>` from the `VecDeque`.
- [ ] All existing unit tests for the `DataStore` are updated and continue to pass.
- [ ] New unit tests are added to verify the circular buffer behavior (e.g., adding more items than the capacity and seeing the oldest ones removed).

## Technical Requirements

- **File Location**: `backend/src/utils/data_store.rs`
- **Data Structure**: `std::collections::VecDeque`
- **Configuration**: The `DataStore` should be configured with a maximum capacity (number of readings).

## Implementation Steps

1.  Modify the `DataStore` struct to replace `Vec` with `VecDeque`.
2.  Update the `new` function to initialize an empty `VecDeque` and accept a `max_size` parameter.
3.  Update the `add_reading` method to first check if the `VecDeque` is at capacity. If it is, it should call `pop_front` before calling `push_back`.
4.  Implement the `remove_oldest_reading` method.
5.  Update the `get_all_readings` method to clone the `VecDeque`'s contents into a new `Vec`.
6.  Refactor the unit tests to accommodate the new circular buffer logic.

## Definition of Done

- The `DataStore` correctly uses a `VecDeque` as a circular buffer.
- The data store never exceeds its configured maximum size.
- The implementation remains thread-safe and all tests pass.

## Dependencies

- Task 1.3.1: Implement In-Memory Data Store with Retention Policy

## Blocked By

None

## Related Tasks

- Task 1.3.3: Automatic cleanup mechanism for oldest data when limits are exceeded
