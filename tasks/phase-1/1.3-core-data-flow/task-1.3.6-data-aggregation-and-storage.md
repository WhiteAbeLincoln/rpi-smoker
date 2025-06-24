# Task 1.3.6: Data Aggregation and Storage

## Task Type

Implementation/Core Feature

## Priority

Medium

## Story Points

6

## Summary

Implement logic within the `DataStore` to perform basic data aggregation (e.g., calculating moving averages) and ensure the raw data is stored correctly.

## Description

While the primary function of the `DataStore` is to hold raw sensor readings, it can be optimized to pre-calculate aggregated data that might be frequently requested by the frontend or control loops. This task involves adding functionality to compute and cache simple aggregations, such as a 1-minute or 5-minute moving average for each sensor. This is a foundational step towards more advanced analytics.

## Acceptance Criteria

- [ ] The `DataStore` is extended to optionally store aggregated data points.
- [ ] A new method, `update_aggregates(&self, reading: &SensorReading)`, is implemented.
- [ ] This method is called internally whenever a new reading is added.
- [ ] The method calculates a moving average for the sensor that produced the new reading.
- [ ] A method `get_aggregated_data(&self, sensor_id: &str) -> Option<AggregatedData>` is available to retrieve the latest aggregated values.
- [ ] The aggregation logic is efficient and does not significantly slow down the `add_reading` operation.
- [ ] Unit tests are added to verify the correctness of the aggregation calculations.

## Technical Requirements

- **File Location**: `backend/src/utils/data_store.rs`
- **Aggregation Logic**:
  - The `DataStore` could maintain a separate `HashMap<String, MovingAverageCalculator>` to track the state for each sensor.
  - The moving average can be a simple Simple Moving Average (SMA) over a configurable window (e.g., the last 60 readings).
- **Data Structures**:
  ```rust
  pub struct AggregatedData {
      pub moving_average_1m: f64,
      pub moving_average_5m: f64,
  }
  ```
- **Efficiency**: The calculation should be incremental if possible, rather than re-calculating from the full dataset on every new reading.

## Implementation Steps

1.  Define the `AggregatedData` struct.
2.  Add a field to `DataStore` to hold the aggregation state, e.g., `aggregates: HashMap<String, AggregatedData>`.
3.  Modify the `add_reading` method to call a new private `update_aggregates` method.
4.  Implement `update_aggregates`. This method will need to look at the last N readings for the given sensor to compute the new average.
5.  A more efficient approach would be to create a helper struct for calculating moving averages incrementally.
6.  Implement the `get_aggregated_data` method to allow other parts of the application to access the computed values.
7.  Write unit tests that add a series of readings and then check if the calculated aggregates are correct.

## Definition of Done

- The `DataStore` can now calculate and store aggregated data alongside raw readings.
- The aggregation logic is tested and correct.
- The feature is ready for use by the API and control loops.

## Dependencies

- Task 1.3.5: Basic Sensor Polling Loop

## Blocked By

None

## Related Tasks

- Phase 4: Frontend Development (the graphs will consume this aggregated data)
- Phase 5: Advanced Features (more complex analytics will build on this)
