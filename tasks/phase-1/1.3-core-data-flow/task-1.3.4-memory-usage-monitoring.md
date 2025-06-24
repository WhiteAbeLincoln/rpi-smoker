# Task 1.3.4: Memory Usage Monitoring and Reporting

## Task Type

Implementation/API

## Priority

Medium

## Story Points

4

## Summary

Implement a method on the `DataStore` to collect and report statistics about its current state, including memory usage, data retention window, and total readings.

## Description

To provide visibility into the application's performance and state, the `DataStore` needs to expose key metrics. This task involves creating a `DataStats` struct and a corresponding public method on `DataStore` to populate and return it. This data will later be exposed via a REST API endpoint.

## Acceptance Criteria

- [ ] A `DataStats` struct is defined in `backend/src/models/api.rs` or a similar public models module.
- [ ] The `DataStats` struct includes fields for `total_readings`, `memory_usage_bytes`, `memory_limit_bytes`, `oldest_reading`, `newest_reading`, and `session_duration_seconds`.
- [ ] A `get_stats(&self) -> DataStats` method is implemented on the `DataStore`.
- [ ] The method accurately calculates the estimated memory usage.
- [ ] The method correctly identifies the timestamps of the oldest and newest readings in the buffer.
- [ ] Unit tests are added to verify that the `get_stats` method returns accurate information under various conditions (empty store, partially full store, full store).

## Technical Requirements

- **File Location**: `backend/src/utils/data_store.rs` for the method, `backend/src/models/api.rs` for the struct.
- **`DataStats` Struct**:
  ```rust
  #[derive(Serialize, Deserialize, Debug, Clone)]
  pub struct DataStats {
      pub total_readings: usize,
      pub memory_usage_bytes: usize,
      pub memory_limit_bytes: usize,
      pub oldest_reading: Option<DateTime<Utc>>,
      pub newest_reading: Option<DateTime<Utc>>,
      pub session_duration_seconds: u64,
      pub cleanup_events: u64, // Can be tracked in DataStore
  }
  ```
- **Logic**: The `get_stats` method should acquire a read lock on the data to ensure consistent results.

## Implementation Steps

1.  Define the `DataStats` struct in a suitable public location.
2.  Add a `get_stats` method to the `DataStore` implementation.
3.  Inside the method, lock the data and calculate all required statistics.
4.  For timestamps, access the `front()` and `back()` of the `VecDeque`.
5.  For memory usage, use `std::mem::size_of::<SensorReading>() * self.readings.len()`.
6.  Add a counter to the `DataStore` to track the number of cleanup events and include it in the stats.
7.  Write unit tests to call `get_stats` and assert the correctness of the returned `DataStats` object.

## Definition of Done

- The `get_stats` method is fully implemented and tested.
- The `DataStats` struct is defined and serializable.
- The reported metrics are accurate and ready to be exposed through an API endpoint.

## Dependencies

- Task 1.3.3: Automatic cleanup mechanism

## Blocked By

None

## Related Tasks

- Task 2.1: REST API Endpoints (specifically the `GET /api/data/stats` endpoint)
