// Example usage of the data models
use rpi_smoker::models::*;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RPI Smoker Data Models Example ===\n");

    // Create some sensor readings
    let temp1 = SensorReading::new("temp_probe_1".to_string(), 3.3, 225.5, Some(2048))?;

    let temp2 = SensorReading::new("temp_probe_2".to_string(), 3.1, 180.0, Some(1900))?;

    println!("Sensor Readings:");
    println!("  {temp1}");
    println!("  {temp2}");
    println!();

    // Create fan states
    let intake_fan = FanState::new("intake_fan".to_string(), 0.75, 25000, false)?;

    let exhaust_fan = FanState::off("exhaust_fan".to_string())?;

    println!("Fan States:");
    println!("  {intake_fan}");
    println!("  {exhaust_fan}");
    println!();

    // Create an alarm for high temperature
    let mut alarm = AlarmEvent::triggered("high_temp_alarm".to_string())?;

    if temp1.is_overheating(220.0) {
        alarm.trigger();
        println!("Alarm Triggered:");
        println!("  {alarm}");
        println!("  Severity: {}", alarm.severity_level());
        println!("  Needs attention: {}", alarm.needs_attention());
        println!();
    }

    // Create API responses
    let mut sensors = HashMap::new();
    sensors.insert("temp_probe_1".to_string(), temp1);
    sensors.insert("temp_probe_2".to_string(), temp2);

    let sensors_response = SensorsResponse::new(sensors);

    let mut fans = HashMap::new();
    fans.insert("intake_fan".to_string(), intake_fan);
    fans.insert("exhaust_fan".to_string(), exhaust_fan);

    let fans_response = FansResponse::new(fans);

    let mut alarms = HashMap::new();
    alarms.insert("high_temp_alarm".to_string(), alarm);

    let alarms_response = AlarmsResponse::new(alarms);

    println!("API Response Summary:");
    println!(
        "  Sensors: {} (avg temp: {:.1}°C)",
        sensors_response.sensor_count(),
        sensors_response.average_temperature().unwrap_or(0.0)
    );
    println!(
        "  Fans: {} ({} running)",
        fans_response.fan_count(),
        fans_response.running_fan_count()
    );
    println!(
        "  Alarms: {} ({} active, {} need attention)",
        alarms_response.total_alarm_count(),
        alarms_response.active_count,
        alarms_response.attention_count()
    );
    println!();

    // Create system statistics
    let mut stats = DataStats::new(1024 * 1024); // 1MB limit
    stats.total_readings = 1000;
    stats.memory_usage_bytes = 512 * 1024; // 512KB used

    println!("System Statistics:");
    println!("  Total readings: {}", stats.total_readings);
    println!(
        "  Memory usage: {:.1}% ({} / {} bytes)",
        stats.memory_usage_percent(),
        stats.memory_usage_bytes,
        stats.memory_limit_bytes
    );
    println!("  Memory usage high: {}", stats.is_memory_usage_high());
    println!();

    // Demonstrate JSON serialization
    let api_response = ApiResponse::success(sensors_response);
    let json = serde_json::to_string_pretty(&api_response)?;

    println!("JSON Serialization Example:");
    println!("{json}");

    Ok(())
}
