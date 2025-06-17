# RPI Smoker Implementation Plan

## High-Level Architecture

### Technology Stack

- **Backend**: Rust with Axum web framework
- **Frontend**: Vue 3 + TypeScript + Vite
- **Communication**: REST API with Server-Sent Events (SSE) for real-time updates
- **Hardware Interface**: Rust GPIO libraries (rppal for Raspberry Pi)
- **Configuration**: JSON files with serde for serialization
- **Build System**: Cargo workspaces for multi-crate organization

### API Design Choice

**Recommendation: REST API + Server-Sent Events (SSE)**

**Rationale:**

- **SSE** provides unidirectional real-time updates (perfect for sensor data streaming)
- **REST** is simpler than GraphQL for this use case with clear CRUD operations
- **Lower complexity** than WebSockets since we don't need bidirectional real-time communication
- **Better browser support** and automatic reconnection handling
- **Easier to implement** push notifications through SSE

**Alternative considered:** GraphQL subscriptions would add unnecessary complexity for this straightforward use case.

## Project Structure

```
rpi-smoker/
├── backend/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── config/
│   │   │   ├── mod.rs
│   │   │   ├── models.rs
│   │   │   └── loader.rs
│   │   ├── hardware/
│   │   │   ├── mod.rs
│   │   │   ├── sensors.rs
│   │   │   ├── fans.rs
│   │   │   └── mock.rs          # For x86/dev testing
│   │   ├── api/
│   │   │   ├── mod.rs
│   │   │   ├── routes.rs
│   │   │   ├── sse.rs
│   │   │   └── handlers.rs
│   │   ├── control/
│   │   │   ├── mod.rs
│   │   │   ├── temperature.rs
│   │   │   ├── fan_controller.rs
│   │   │   └── alarm_system.rs
│   │   └── utils/
│   │       ├── mod.rs
│   │       ├── formula_eval.rs  # For parsing user formulas
│   │       └── data_store.rs    # In-memory storage
│   └── tests/
├── frontend/
│   ├── package.json
│   ├── vite.config.ts
│   ├── src/
│   │   ├── main.ts
│   │   ├── App.vue
│   │   ├── components/
│   │   │   ├── TemperatureGraph.vue
│   │   │   ├── FanSpeedGraph.vue
│   │   │   ├── ConfigurationForm.vue
│   │   │   ├── AlarmSettings.vue
│   │   │   └── NotificationManager.vue
│   │   ├── stores/
│   │   │   ├── config.ts
│   │   │   ├── sensors.ts
│   │   │   └── sse.ts
│   │   ├── types/
│   │   │   └── api.ts
│   │   └── utils/
│   │       ├── api.ts
│   │       └── notifications.ts
│   ├── public/
│   │   ├── manifest.json        # PWA manifest
│   │   └── sw.js                # Service worker
│   └── dist/
├── shared/
│   └── types.rs                 # Shared type definitions
├── config/
│   └── default.json             # Default configuration
├── Cargo.toml                   # Workspace manifest
└── README.md
```

## Implementation Phases

### Phase 1: Core Backend Infrastructure (Week 1-2)

#### 1.1 Project Setup

- [ ] Create Cargo workspace with backend crate
- [ ] Set up Axum web server with basic routing
- [x] Implement configuration loading from JSON
- [ ] Create basic data models and validation

#### 1.2 Hardware Abstraction Layer

- [ ] Design trait-based hardware interface
- [ ] Implement mock sensors/fans for development
- [ ] Add conditional compilation for Pi vs dev environments
- [ ] Basic GPIO PWM control structure

#### 1.3 Core Data Flow

- [ ] In-memory data store for temperature readings with retention policy
- [ ] Circular buffer implementation with 7-hour time limit and 1GB memory limit
- [ ] Automatic cleanup mechanism for oldest data when limits are exceeded
- [ ] Memory usage monitoring and reporting
- [ ] Basic sensor polling loop
- [ ] Data aggregation and storage

### Phase 2: API and Real-time Communication (Week 2-3)

#### 2.1 REST API Endpoints

```
GET  /api/config                    # Get current configuration
PUT  /api/config                    # Update configuration
GET  /api/sensors                   # Get current sensor readings
GET  /api/fans                      # Get current fan states
POST /api/data/clear                # Clear temperature history
GET  /api/data/stats                # Get memory usage and retention stats
GET  /api/alarms                    # Get alarm configurations
PUT  /api/alarms                    # Update alarm configurations
```

#### 2.2 Server-Sent Events

- [ ] SSE endpoint: `/api/events`
- [ ] Event types: `temperature-update`, `fan-update`, `alarm-triggered`
- [ ] Client subscription management
- [ ] Automatic reconnection handling

#### 2.3 Data Models

```rust
// Core data structures
#[derive(Serialize, Deserialize)]
pub struct SensorReading {
    pub sensor_id: String,
    pub timestamp: DateTime<Utc>,
    pub voltage: f64,
    pub temperature_celsius: f64,
}

#[derive(Serialize, Deserialize)]
pub struct FanState {
    pub fan_id: String,
    pub duty_cycle: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct AlarmEvent {
    pub alarm_id: String,
    pub triggered: bool,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct DataStats {
    pub total_readings: usize,
    pub memory_usage_bytes: usize,
    pub memory_limit_bytes: usize,
    pub oldest_reading: Option<DateTime<Utc>>,
    pub newest_reading: Option<DateTime<Utc>>,
    pub session_duration_seconds: u64,
    pub cleanup_events: u64,
}
```

### Phase 3: Hardware Integration (Week 3-4)

#### 3.1 Temperature Sensors (ADS1115)

- [ ] ADS1115 I2C communication
- [ ] Configurable gain settings
- [ ] Voltage-to-temperature conversion formulas
- [ ] Error handling and sensor validation

#### 3.2 Fan Control (PWM)

- [ ] Hardware PWM implementation
- [ ] Duty cycle formula evaluation
- [ ] Safety limits and validation
- [ ] Fan state monitoring

#### 3.3 Formula Engine

- [ ] Safe mathematical expression parser
- [ ] Support for basic arithmetic and comparison operators
- [ ] Variable substitution (temp1, temp2, etc.)
- [ ] Runtime evaluation with error handling

**Recommendation for Formula Engine:** Use the `evalexpr` crate for safe expression evaluation with custom contexts.

### Phase 4: Frontend Development (Week 4-6)

#### 4.1 Vue 3 Application Setup

- [ ] Vite build configuration
- [ ] TypeScript integration
- [ ] Pinia store setup for state management
- [ ] Router configuration

#### 4.2 Core Components

- [ ] Real-time temperature graphs (Chart.js or D3.js)
- [ ] Fan speed visualization
- [ ] Configuration forms with validation
- [ ] Alarm management interface

#### 4.3 PWA Implementation

- [ ] Service worker for offline capability
- [ ] Web app manifest
- [ ] Push notification support
- [ ] Install prompt handling

#### 4.4 Real-time Updates

- [ ] SSE client implementation
- [ ] Automatic reconnection logic
- [ ] Reactive data updates in Vue stores
- [ ] Subscription management based on active components

### Phase 5: Advanced Features (Week 6-8)

#### 5.1 Alarm System

- [ ] Formula-based alarm conditions
- [ ] Push notification delivery
- [ ] Optional action triggers (fan control)
- [ ] Alarm history and acknowledgment

#### 5.2 Enhanced Control Features

- [ ] Conditional duty cycle formulas
- [ ] Multiple formula support with precedence
- [ ] Safety interlocks and limits
- [ ] Manual override capabilities

#### 5.3 Data Management

- [ ] Data export functionality
- [ ] Session management
- [ ] Configuration backup/restore
- [ ] Performance optimization

### Phase 6: Testing and Deployment (Week 8-9)

#### 6.1 Testing Strategy

- [ ] Unit tests for formula engine
- [ ] Hardware mock integration tests
- [ ] API endpoint tests
- [ ] Frontend component tests
- [ ] End-to-end testing on Raspberry Pi

#### 6.2 Deployment

- [ ] Cross-compilation for ARM
- [ ] Systemd service configuration
- [ ] Auto-start configuration
- [ ] Update mechanism

## Technical Decisions and Recommendations

### Configuration Format

```json
{
  "hardware": {
    "i2c_bus": 1,
    "ads1115_address": "0x48"
  },
  "temp_sensors": {
    "temp1": {
      "ads1115_channel": 0,
      "gain": 1,
      "poll_interval_ms": 1000,
      "conversion_formula": "voltage * 100.0"
    },
    "temp2": {
      "ads1115_channel": 1,
      "gain": 1,
      "poll_interval_ms": 1000,
      "conversion_formula": "voltage * 100.0"
    }
  },
  "fans": {
    "fan1": {
      "pwm_pin": 18,
      "frequency_hz": 1000,
      "duty_cycle_formulas": [
        {
          "when": "temp1 >= 100.0 && temp1 <= 200.0",
          "value": "(temp1 - 100.0) / 100.0"
        },
        {
          "when": "temp1 > 200.0",
          "value": "1.0"
        },
        {
          "value": "0.0"
        }
      ]
    }
  },
  "alarms": {
    "overtemp": {
      "condition": "temp1 > 250.0 || temp2 > 250.0",
      "message": "Temperature too high!",
      "actions": ["turn_off_fan1"]
    }
  }
}
```

### Key Libraries

**Backend:**

- `axum` - Modern async web framework
- `tokio` - Async runtime
- `serde` - Serialization
- `rppal` - Raspberry Pi GPIO (with feature flags)
- `evalexpr` - Safe expression evaluation
- `ads1x1x` - ADS1115 driver
- `clap` - CLI argument parsing

**Frontend:**

- `vue` - Framework
- `@vueuse/core` - Utility library
- `chart.js` - Graphing
- `pinia` - State management
- `vite-pwa` - PWA plugin

### Development Environment

**Cross-platform Development:**

- Use feature flags to enable/disable hardware dependencies
- Mock implementations for development on x86/ARM64
- Docker containers for consistent builds
- CI/CD pipeline for automated testing

### Security Considerations

Since this runs on a local network without authentication:

- Input validation on all configuration inputs
- Safe formula evaluation (no system calls)
- Rate limiting on API endpoints
- CORS configuration for local development
- CSP headers for the frontend

### Performance Considerations

- Efficient in-memory data structures with circular buffers
- Optimized SSE event batching
- Lazy loading for historical data graphs
- Hardware polling optimization

#### Data Retention Policy

**Memory Management:**

- **Maximum session duration**: 7 hours
- **Maximum memory usage**: 1 GB for temperature data storage
- **Cleanup strategy**: Drop oldest data when limits are exceeded
- **Safety buffer**: Maintain 20% buffer below limits to prevent frequent cleanup cycles

**Implementation Details:**

- Use circular buffer data structure for temperature readings
- Implement memory usage monitoring with periodic cleanup
- Estimate memory usage: ~1KB per sensor reading (assuming 4 sensors @ 1 reading/sec = ~100MB for 7 hours)
- Cleanup triggers:
  - Time-based: Remove data older than 7 hours
  - Memory-based: Remove oldest 25% of data when approaching 800MB usage
  - Manual: API endpoint to clear all historical data

**Data Structure:**

```rust
pub struct DataStore {
    readings: VecDeque<SensorReading>,
    max_age: Duration,           // 7 hours
    max_memory_bytes: usize,     // 1 GB
    cleanup_threshold: f64,      // 0.8 (80% of max memory)
    cleanup_percentage: f64,     // 0.25 (remove 25% when cleaning)
}
```

**Post-MVP Migration:**

- When "Data Persistence" feature is implemented, transition to:
  - Keep last 1 hour in memory for real-time operations
  - Swap older data to disk/database storage
  - Implement background data archival process
  - Maintain fast access patterns for recent data

## Risk Mitigation

### Hardware Failures

- Sensor disconnection detection
- Fan failure monitoring
- Graceful degradation modes
- Hardware abstraction for easy testing

### Software Reliability

- Comprehensive error handling
- Automatic service restart
- Configuration validation
- Safe formula evaluation

### User Experience

- Clear error messages
- Responsive design for mobile devices
- Offline capability through PWA
- Intuitive configuration interface

## Future Enhancements (Post-MVP)

In order of importance & implementability:

1. **Enhanced Notifications**

   - Email/SMS integration
   - Custom notification rules
   - Integration with home automation (e.g. [Home Assistant](https://www.home-assistant.io))

2. **Data Persistence**

   - Optional database storage
   - Historical session data
   - Recipe/cook profiles

3. **Multi-Device Support**

   - Multiple sensor types
   - Additional output devices
   - Modular hardware configuration

4. **Advanced Analytics**
   - Temperature trend analysis
   - Predictive algorithms
   - Cook time estimation

This implementation plan provides a solid foundation for building a robust, feature-complete smoker control system while maintaining flexibility for future enhancements.
