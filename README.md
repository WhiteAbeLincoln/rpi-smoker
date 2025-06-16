# RPI Smoker

An application for controlling a blower fan for a smoker, using one or more attached temperature probe sensors.

## Project Goals / Specs

### Web Interface

- The user can access a web interface to configure and view the system
- All configuration can be performed through forms in the web interface
- The interface has graphs showing the current & historical calculated temperature for each configured temperature probe
- The interface has graphs showing the current & historical fan speeds
- The website should be a PWA that the user can install to their device
- The website should receive sensor data updates pushed from the server
- The website should receive push-notifications from the server
- The website should be able to unsubscribe and subscribe to sensor updates depending on elements on the current page
- The website does not need authentication - this is intended to be an application on a secure local network
- The website should allow clearing the temperature data, in case the server has remained running since the last session.

### Server

#### Configuration

All configuration should be stored in a human-readable JSON file

#### Temperature Reading

The server will read temperatures from one or more analog temperature sensors attached to an ADS1115 Analog-to-Digital Converter.

- The sensors will report values in volts. The user should be able to configure the conversion formula
  from volts to temperature in celsius.
- The user should be able to configure the gain for the sensor
- The user should be able to configure the poll rate for each sensor
- Temperature values are stored in-memory. There is no need to persist them to disk since we only need
  data for the single smoking session.

#### Alarms

The server should notify the web client when the temperatures pass a configured threshold

- The alarms can be configured as a formula in terms of the instantaneous temperature values (in celsius),
  e.g `(temp1 + temp2) / 2 >= 100.0`.
  - TODO: Support non-instantaneous measurements, like running averages, average temperature over a time threshold, etc.
- The alarm should present a push-notification to the web interface by default.
- The alarm should be able to optionally trigger an action, like "turn off fan X".

#### Fan Control

The server should control one or more attached fans through pulse-width modulation (PWM).

##### Hardware Configuration

##### Duty Cycles

- The user can configure the duty cycle for the fans using one or more formulas which evaulate to a floating-point value between 0 and 1.
  - The user can set a single formula which configures the duty cycle continuously over the entire temperature range.
  - Alternatively, the user can set multiple formulas which only take effect under specific conditions:

For example, this configuration (note, the format is not finalized) will set the duty cycle as a linear relation to
the temperature sensor `temp1`, when `temp1` is between 100.0 and 200.0 degrees celsius.

```json
{
  "temp_sensors": {
    "temp1": {}
  },
  "fans": {
    "fan1": {
      "dutyCycle": [
        {
          "when": "temp1 >= 100.0 and temp1 <= 200.0",
          "value": "(temp1-100) / 100"
        }
      ]
    }
  }
}
```

This configuration sets a constant 50% duty cycle.

```json
{
  "fans": {
    "fan1": { "dutyCycle": [{ "value": "0.5" }] }
  }
}
```

If multiple duty cycles are applicable, the last matching cycle in the list is used.

## Technical Details

- The application will run on a Raspberry Pi 3B+, but also needs to run on a regular x86 or arm PC for
  unit & integration testing.
  - When run under an arm or x86 PC, we should be able to use dummy temperature and fan sensors.
- The website will be written in Vue 3 and TypeScript.
- The server will be written in Rust.

## TODOs / Future Ideas
