//! Observer implementations for the stub plugin
//!
//! This module demonstrates how to implement observers that can monitor
//! simulation events, component state changes, and system events.
//!
//! # API Stability Warning
//! These observers use **UNSTABLE** APIs that may change without notice.

use logisim_core::{
    SimulationObserver, ComponentObserver, SystemObserver,
    SimulationEvent, ObserverComponentEvent as ComponentEvent, // Use the aliased version
    ObserverId, ObserverResult, ComponentId, Timestamp,
};
use ::std::collections::HashMap;

/// A plugin event logger that logs all events it observes
/// 
/// This demonstrates how to create observers that can monitor multiple event types.
pub struct PluginEventLogger {
    id: ObserverId,
    name: String,
    event_counts: HashMap<String, u32>,
}

impl PluginEventLogger {
    /// Create a new plugin event logger
    pub fn new() -> Self {
        Self {
            id: ObserverId::new(0), // Will be assigned by manager
            name: "Plugin Event Logger".to_string(),
            event_counts: HashMap::new(),
        }
    }
    
    /// Get the number of events of a specific type that have been logged
    pub fn get_event_count(&self, event_type: &str) -> u32 {
        self.event_counts.get(event_type).copied().unwrap_or(0)
    }
    
    /// Get total number of events logged
    pub fn get_total_event_count(&self) -> u32 {
        self.event_counts.values().sum()
    }
    
    /// Reset event counters
    pub fn reset_counters(&mut self) {
        self.event_counts.clear();
    }
    
    /// Increment the counter for a specific event type
    fn increment_counter(&mut self, event_type: &str) {
        *self.event_counts.entry(event_type.to_string()).or_insert(0) += 1;
    }
}

impl SimulationObserver for PluginEventLogger {
    fn id(&self) -> ObserverId {
        self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn on_simulation_event(&mut self, event: &SimulationEvent) -> ObserverResult<()> {
        match event {
            SimulationEvent::Started { timestamp } => {
                log::info!("[PluginEventLogger] Simulation started at time {}", timestamp.0);
                self.increment_counter("simulation_started");
            }
            SimulationEvent::Stopped { timestamp } => {
                log::info!("[PluginEventLogger] Simulation stopped at time {}", timestamp.0);
                self.increment_counter("simulation_stopped");
            }
            SimulationEvent::Paused { timestamp } => {
                log::info!("[PluginEventLogger] Simulation paused at time {}", timestamp.0);
                self.increment_counter("simulation_paused");
            }
            SimulationEvent::Resumed { timestamp } => {
                log::info!("[PluginEventLogger] Simulation resumed at time {}", timestamp.0);
                self.increment_counter("simulation_resumed");
            }
            SimulationEvent::Reset => {
                log::info!("[PluginEventLogger] Simulation reset");
                self.increment_counter("simulation_reset");
            }
            SimulationEvent::StepCompleted { timestamp } => {
                log::debug!("[PluginEventLogger] Simulation step completed at time {}", timestamp.0);
                self.increment_counter("step_completed");
            }
            SimulationEvent::ClockTick { timestamp, signal } => {
                log::debug!("[PluginEventLogger] Clock tick at time {} with signal {:?}", timestamp.0, signal);
                self.increment_counter("clock_tick");
            }
        }
        
        Ok(())
    }
    
    fn interested_in_event(&self, event: &SimulationEvent) -> bool {
        // We're interested in all events except step completion (too noisy for logging)
        !matches!(event, SimulationEvent::StepCompleted { .. })
    }
}

impl ComponentObserver for PluginEventLogger {
    fn id(&self) -> ObserverId {
        self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn on_component_event(&mut self, event: &ComponentEvent) -> ObserverResult<()> {
        match event {
            ComponentEvent::Created { component_id, component_type } => {
                log::info!("[PluginEventLogger] Component {} created: {}", component_id, component_type);
                self.increment_counter("component_created");
            }
            ComponentEvent::Removed { component_id } => {
                log::info!("[PluginEventLogger] Component {} removed", component_id);
                self.increment_counter("component_removed");
            }
            ComponentEvent::StateChanged { component_id, timestamp } => {
                log::debug!("[PluginEventLogger] Component {} state changed at time {}", component_id, timestamp.0);
                self.increment_counter("component_state_changed");
            }
            ComponentEvent::InputChanged { component_id, pin_name, old_signal, new_signal, timestamp } => {
                log::debug!(
                    "[PluginEventLogger] Component {} input {} changed at time {}: {:?} -> {:?}",
                    component_id, pin_name, timestamp.0, old_signal, new_signal
                );
                self.increment_counter("component_input_changed");
            }
            ComponentEvent::OutputChanged { component_id, pin_name, old_signal, new_signal, timestamp } => {
                log::debug!(
                    "[PluginEventLogger] Component {} output {} changed at time {}: {:?} -> {:?}",
                    component_id, pin_name, timestamp.0, old_signal, new_signal
                );
                self.increment_counter("component_output_changed");
            }
            ComponentEvent::Reset { component_id } => {
                log::info!("[PluginEventLogger] Component {} reset", component_id);
                self.increment_counter("component_reset");
            }
        }
        
        Ok(())
    }
    
    fn interested_in_component(&self, _component_id: ComponentId) -> bool {
        // We're interested in all components
        true
    }
    
    fn interested_in_event(&self, event: &ComponentEvent) -> bool {
        // We're interested in all events except state changes (too noisy)
        !matches!(event, ComponentEvent::StateChanged { .. })
    }
}

impl SystemObserver for PluginEventLogger {
    fn id(&self) -> ObserverId {
        self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn on_system_init(&mut self) -> ObserverResult<()> {
        log::info!("[PluginEventLogger] System initializing");
        self.increment_counter("system_init");
        Ok(())
    }
    
    fn on_system_shutdown(&mut self) -> ObserverResult<()> {
        log::info!("[PluginEventLogger] System shutting down - logged {} total events", 
                  self.get_total_event_count());
        self.increment_counter("system_shutdown");
        Ok(())
    }
    
    fn on_plugin_loaded(&mut self, plugin_name: &str) -> ObserverResult<()> {
        log::info!("[PluginEventLogger] Plugin loaded: {}", plugin_name);
        self.increment_counter("plugin_loaded");
        Ok(())
    }
    
    fn on_plugin_unloaded(&mut self, plugin_name: &str) -> ObserverResult<()> {
        log::info!("[PluginEventLogger] Plugin unloaded: {}", plugin_name);
        self.increment_counter("plugin_unloaded");
        Ok(())
    }
}

/// A component state tracker that maintains statistics about component behavior
/// 
/// This demonstrates how to create observers that collect data for analysis.
pub struct ComponentStateTracker {
    id: ObserverId,
    name: String,
    component_stats: HashMap<ComponentId, ComponentStats>,
    activity_log: Vec<ActivityEntry>,
    max_log_entries: usize,
}

/// Statistics tracked for each component
#[derive(Debug, Clone)]
pub struct ComponentStats {
    pub component_id: ComponentId,
    pub component_type: String,
    pub state_changes: u32,
    pub input_changes: u32,
    pub output_changes: u32,
    pub reset_count: u32,
    pub first_seen: Timestamp,
    pub last_activity: Timestamp,
}

/// Activity log entry
#[derive(Debug, Clone)]
pub struct ActivityEntry {
    pub timestamp: Timestamp,
    pub component_id: ComponentId,
    pub activity_type: ActivityType,
    pub details: String,
}

/// Types of component activity
#[derive(Debug, Clone)]
pub enum ActivityType {
    Created,
    StateChanged,
    InputChanged,
    OutputChanged,
    Reset,
    Removed,
}

impl ComponentStateTracker {
    /// Create a new component state tracker
    pub fn new() -> Self {
        Self {
            id: ObserverId::new(0), // Will be assigned by manager
            name: "Component State Tracker".to_string(),
            component_stats: HashMap::new(),
            activity_log: Vec::new(),
            max_log_entries: 1000, // Limit log size
        }
    }
    
    /// Get statistics for a specific component
    pub fn get_component_stats(&self, component_id: ComponentId) -> Option<&ComponentStats> {
        self.component_stats.get(&component_id)
    }
    
    /// Get statistics for all components
    pub fn get_all_stats(&self) -> Vec<&ComponentStats> {
        self.component_stats.values().collect()
    }
    
    /// Get the activity log
    pub fn get_activity_log(&self) -> &[ActivityEntry] {
        &self.activity_log
    }
    
    /// Get the most active components (by total activity)
    pub fn get_most_active_components(&self, limit: usize) -> Vec<(ComponentId, u32)> {
        let mut activities: Vec<(ComponentId, u32)> = self.component_stats
            .iter()
            .map(|(id, stats)| {
                let total_activity = stats.state_changes + stats.input_changes + 
                                   stats.output_changes + stats.reset_count;
                (*id, total_activity)
            })
            .collect();
        
        activities.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by activity descending
        activities.truncate(limit);
        activities
    }
    
    /// Clear all statistics and logs
    pub fn clear_statistics(&mut self) {
        self.component_stats.clear();
        self.activity_log.clear();
        log::info!("[ComponentStateTracker] Statistics cleared");
    }
    
    /// Add an activity entry to the log
    fn log_activity(&mut self, component_id: ComponentId, activity_type: ActivityType, details: String, timestamp: Timestamp) {
        let entry = ActivityEntry {
            timestamp,
            component_id,
            activity_type,
            details,
        };
        
        self.activity_log.push(entry);
        
        // Limit log size
        if self.activity_log.len() > self.max_log_entries {
            self.activity_log.remove(0);
        }
    }
    
    /// Get or create component statistics
    fn get_or_create_stats(&mut self, component_id: ComponentId, component_type: &str, timestamp: Timestamp) -> &mut ComponentStats {
        self.component_stats.entry(component_id).or_insert_with(|| {
            ComponentStats {
                component_id,
                component_type: component_type.to_string(),
                state_changes: 0,
                input_changes: 0,
                output_changes: 0,
                reset_count: 0,
                first_seen: timestamp,
                last_activity: timestamp,
            }
        })
    }
}

impl ComponentObserver for ComponentStateTracker {
    fn id(&self) -> ObserverId {
        self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn on_component_event(&mut self, event: &ComponentEvent) -> ObserverResult<()> {
        match event {
            ComponentEvent::Created { component_id, component_type } => {
                let stats = ComponentStats {
                    component_id: *component_id,
                    component_type: component_type.clone(),
                    state_changes: 0,
                    input_changes: 0,
                    output_changes: 0,
                    reset_count: 0,
                    first_seen: Timestamp(0), // Creation time
                    last_activity: Timestamp(0),
                };
                
                self.component_stats.insert(*component_id, stats);
                self.log_activity(
                    *component_id, 
                    ActivityType::Created, 
                    format!("Component {} created", component_type),
                    Timestamp(0)
                );
                
                log::debug!("[ComponentStateTracker] Tracking new component {} ({})", component_id, component_type);
            }
            
            ComponentEvent::Removed { component_id } => {
                if let Some(stats) = self.component_stats.get(component_id) {
                    self.log_activity(
                        *component_id, 
                        ActivityType::Removed, 
                        format!("Component {} removed", stats.component_type),
                        Timestamp(0) // We don't have timestamp in this event
                    );
                }
                
                self.component_stats.remove(component_id);
                log::debug!("[ComponentStateTracker] Stopped tracking component {}", component_id);
            }
            
            ComponentEvent::StateChanged { component_id, timestamp } => {
                let current_count = if let Some(stats) = self.component_stats.get(component_id) {
                    stats.state_changes + 1
                } else {
                    1
                };
                
                if let Some(stats) = self.component_stats.get_mut(component_id) {
                    stats.state_changes = current_count;
                    stats.last_activity = *timestamp;
                }
                
                self.log_activity(
                    *component_id, 
                    ActivityType::StateChanged, 
                    format!("State change #{}", current_count),
                    *timestamp
                );
            }
            
            ComponentEvent::InputChanged { component_id, pin_name, old_signal, new_signal, timestamp } => {
                let current_count = if let Some(stats) = self.component_stats.get(component_id) {
                    stats.input_changes + 1
                } else {
                    1
                };
                
                if let Some(stats) = self.component_stats.get_mut(component_id) {
                    stats.input_changes = current_count;
                    stats.last_activity = *timestamp;
                }
                
                self.log_activity(
                    *component_id, 
                    ActivityType::InputChanged, 
                    format!("Input {} changed: {:?} -> {:?}", pin_name, old_signal, new_signal),
                    *timestamp
                );
            }
            
            ComponentEvent::OutputChanged { component_id, pin_name, old_signal, new_signal, timestamp } => {
                let current_count = if let Some(stats) = self.component_stats.get(component_id) {
                    stats.output_changes + 1
                } else {
                    1
                };
                
                if let Some(stats) = self.component_stats.get_mut(component_id) {
                    stats.output_changes = current_count;
                    stats.last_activity = *timestamp;
                }
                
                self.log_activity(
                    *component_id, 
                    ActivityType::OutputChanged, 
                    format!("Output {} changed: {:?} -> {:?}", pin_name, old_signal, new_signal),
                    *timestamp
                );
            }
            
            ComponentEvent::Reset { component_id } => {
                let current_count = if let Some(stats) = self.component_stats.get(component_id) {
                    stats.reset_count + 1
                } else {
                    1
                };
                
                if let Some(stats) = self.component_stats.get_mut(component_id) {
                    stats.reset_count = current_count;
                    stats.last_activity = Timestamp(0); // Reset events don't have timestamps
                }
                
                self.log_activity(
                    *component_id, 
                    ActivityType::Reset, 
                    format!("Reset #{}", current_count),
                    Timestamp(0)
                );
            }
        }
        
        Ok(())
    }
    
    fn interested_in_component(&self, _component_id: ComponentId) -> bool {
        // We're interested in tracking all components
        true
    }
    
    fn interested_in_event(&self, _event: &ComponentEvent) -> bool {
        // We're interested in all component events for tracking
        true
    }
}

/// A specialized observer that monitors only custom components from this plugin
pub struct CustomComponentMonitor {
    id: ObserverId,
    name: String,
    custom_xor_count: u32,
    custom_counter_count: u32,
    total_operations: u64,
}

impl CustomComponentMonitor {
    /// Create a new custom component monitor
    pub fn new() -> Self {
        Self {
            id: ObserverId::new(0), // Will be assigned by manager
            name: "Custom Component Monitor".to_string(),
            custom_xor_count: 0,
            custom_counter_count: 0,
            total_operations: 0,
        }
    }
    
    /// Get the number of custom XOR components being monitored
    pub fn get_custom_xor_count(&self) -> u32 {
        self.custom_xor_count
    }
    
    /// Get the number of custom counter components being monitored
    pub fn get_custom_counter_count(&self) -> u32 {
        self.custom_counter_count
    }
    
    /// Get the total number of operations performed by monitored components
    pub fn get_total_operations(&self) -> u64 {
        self.total_operations
    }
}

impl ComponentObserver for CustomComponentMonitor {
    fn id(&self) -> ObserverId {
        self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn on_component_event(&mut self, event: &ComponentEvent) -> ObserverResult<()> {
        match event {
            ComponentEvent::Created { component_id, component_type } => {
                match component_type.as_str() {
                    "CustomXOR" => {
                        self.custom_xor_count += 1;
                        log::info!("[CustomComponentMonitor] Custom XOR component {} created (total: {})", 
                                  component_id, self.custom_xor_count);
                    }
                    "CustomCounter" => {
                        self.custom_counter_count += 1;
                        log::info!("[CustomComponentMonitor] Custom Counter component {} created (total: {})", 
                                  component_id, self.custom_counter_count);
                    }
                    _ => {} // Not interested in other component types
                }
            }
            
            ComponentEvent::Removed { component_id } => {
                // Note: We can't determine the type from removal event, so we'd need to track it
                log::debug!("[CustomComponentMonitor] Component {} removed", component_id);
            }
            
            ComponentEvent::StateChanged { component_id, .. } => {
                // Count state changes as operations for our custom components
                self.total_operations += 1;
                log::debug!("[CustomComponentMonitor] Custom component {} performed operation #{}", 
                           component_id, self.total_operations);
            }
            
            _ => {} // Not interested in other events for this monitor
        }
        
        Ok(())
    }
    
    fn interested_in_component(&self, _component_id: ComponentId) -> bool {
        // We'll filter by component type in the event handler
        true
    }
    
    fn interested_in_event(&self, event: &ComponentEvent) -> bool {
        // We're only interested in creation, removal, and state changes
        matches!(
            event, 
            ComponentEvent::Created { .. } | 
            ComponentEvent::Removed { .. } | 
            ComponentEvent::StateChanged { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_event_logger_creation() {
        let logger = PluginEventLogger::new();
        assert_eq!(SystemObserver::name(&logger), "Plugin Event Logger");
        assert_eq!(logger.get_total_event_count(), 0);
    }

    #[test]
    fn test_plugin_event_logger_simulation_events() {
        let mut logger = PluginEventLogger::new();
        
        let start_event = SimulationEvent::Started { timestamp: Timestamp(10) };
        let result = logger.on_simulation_event(&start_event);
        assert!(result.is_ok());
        assert_eq!(logger.get_event_count("simulation_started"), 1);
        
        let stop_event = SimulationEvent::Stopped { timestamp: Timestamp(20) };
        let result = logger.on_simulation_event(&stop_event);
        assert!(result.is_ok());
        assert_eq!(logger.get_event_count("simulation_stopped"), 1);
        
        assert_eq!(logger.get_total_event_count(), 2);
    }

    #[test]
    fn test_component_state_tracker_creation() {
        let tracker = ComponentStateTracker::new();
        assert_eq!(ComponentObserver::name(&tracker), "Component State Tracker");
        assert_eq!(tracker.get_all_stats().len(), 0);
    }

    #[test]
    fn test_component_state_tracker_component_events() {
        let mut tracker = ComponentStateTracker::new();
        
        let create_event = ComponentEvent::Created {
            component_id: ComponentId::new(1),
            component_type: "TestComponent".to_string(),
        };
        
        let result = tracker.on_component_event(&create_event);
        assert!(result.is_ok());
        
        let stats = tracker.get_component_stats(ComponentId::new(1));
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().component_type, "TestComponent");
        
        let state_change_event = ComponentEvent::StateChanged {
            component_id: ComponentId::new(1),
            timestamp: Timestamp(5),
        };
        
        let result = tracker.on_component_event(&state_change_event);
        assert!(result.is_ok());
        
        let stats = tracker.get_component_stats(ComponentId::new(1)).unwrap();
        assert_eq!(stats.state_changes, 1);
        assert_eq!(stats.last_activity, Timestamp(5));
    }

    #[test]
    fn test_custom_component_monitor() {
        let mut monitor = CustomComponentMonitor::new();
        
        let xor_create_event = ComponentEvent::Created {
            component_id: ComponentId::new(1),
            component_type: "CustomXOR".to_string(),
        };
        
        let result = monitor.on_component_event(&xor_create_event);
        assert!(result.is_ok());
        assert_eq!(monitor.get_custom_xor_count(), 1);
        
        let counter_create_event = ComponentEvent::Created {
            component_id: ComponentId::new(2),
            component_type: "CustomCounter".to_string(),
        };
        
        let result = monitor.on_component_event(&counter_create_event);
        assert!(result.is_ok());
        assert_eq!(monitor.get_custom_counter_count(), 1);
        
        let state_change_event = ComponentEvent::StateChanged {
            component_id: ComponentId::new(1),
            timestamp: Timestamp(10),
        };
        
        let result = monitor.on_component_event(&state_change_event);
        assert!(result.is_ok());
        assert_eq!(monitor.get_total_operations(), 1);
    }
}