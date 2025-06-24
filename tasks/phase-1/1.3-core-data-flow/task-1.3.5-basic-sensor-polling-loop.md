# Task 1.3.5: Basic Sensor Polling Loop

## Task Type

Implementation/Core Feature

## Priority

High

## Story Points

8

## Summary

Create a long-running asynchronous task that periodically polls the temperature sensors for new data and pushes the readings into the central `DataStore`.

## Description

This is a core component of the application's data pipeline. A separate async task will be responsible for managing all configured sensors. It will loop through each sensor, trigger a reading, and store the resulting `SensorReading` object in the shared `DataStore`. The polling interval for each sensor should be configurable.

## Acceptance Criteria

- [ ] A new `sensor_poller.rs` module is created (e.g., in `backend/src/control/`).
- [ ] A `start_sensor_polling_loop` function is defined that takes the shared `DataStore` and the hardware configuration as input.
- [ ] The function spawns a `tokio` task.
- [ ] Inside the task, it iterates through the configured temperature sensors.
- [ ] For each sensor, it calls the appropriate `TemperatureSensor` trait method to get a reading.
- [ ] The new `SensorReading` is pushed to the `DataStore`.
- [ ] The loop respects the `poll_interval_ms` configured for each sensor, sleeping for the appropriate duration between polls.
- [ ] The loop includes robust error handling to log and continue if a single sensor fails to read.

## Technical Requirements

- **File Location**: `backend/src/control/sensor_poller.rs`
- **Asynchronicity**: The entire loop must be non-blocking, using `tokio::time::sleep` for delays.
- **Configuration**: The loop should be driven by the application's configuration, which specifies the list of sensors and their properties.
- **Error Handling**: Errors from sensor reads should be logged, but should not stop the polling loop from continuing with other sensors.

## Implementation Steps

1.  Create the `backend/src/control/sensor_poller.rs` file.
2.  Define the `start_sensor_polling_loop` function.
3.  Inside the function, retrieve the list of sensors from the application configuration.
4.  Spawn a `tokio` task that contains the main `loop`.
5.  Inside the loop, iterate over the sensors. For each one, spawn another `tokio` task to handle its individual polling logic. This allows sensors to be polled in parallel.
6.  The individual sensor task will loop indefinitely, calling the sensor's `read_temperature_celsius` method, creating a `SensorReading`, pushing it to the `DataStore`, and then sleeping for its configured interval.
7.  Add logging to report successful readings and any errors encountered.
8.  Integrate the `start_sensor_polling_loop` function into the application's main startup sequence.

## Definition of Done

- The sensor polling loop is implemented and runs correctly in the background.
- Sensor data is continuously and correctly added to the `DataStore`.
- The implementation is resilient to individual sensor failures.
- The polling behavior is configurable and tested.

## Dependencies

- Task 1.2.2: Implement mock sensors/fans for development
- Task 1.3.1: Implement In-Memory Data Store with Retention Policy

## Blocked By

None

## Related Tasks

- Task 1.3.6: Data aggregation and storage
- Task 2.2: Server-Sent Events (SSE)
