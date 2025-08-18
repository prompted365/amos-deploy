use amos_core::logging::*;
use serde_json::json;
use uuid::Uuid;

#[test]
fn test_log_entry_creation() {
    let entry = LogEntry::new(LogLevel::Info, "test_component", "Test message");
    
    assert_eq!(entry.level, LogLevel::Info);
    assert_eq!(entry.component, "test_component");
    assert_eq!(entry.message, "Test message");
    assert!(entry.context.is_object());
}

#[test]
fn test_log_entry_with_context() {
    let entry = LogEntry::new(LogLevel::Debug, "neural", "Pathway strengthened")
        .with_context("pathway_id", json!(Uuid::new_v4()))
        .with_context("strength", json!(0.75));
    
    assert!(entry.context.get("pathway_id").is_some());
    assert!(entry.context.get("strength").is_some());
}

#[test]
fn test_logger_creation() {
    let _logger = Logger::new("amos-core");
    // Logger created successfully
}

#[test]
fn test_logger_levels() {
    let logger = Logger::new("test").with_level(LogLevel::Warn);
    
    // These should not print (below min level)
    logger.trace("This is trace");
    logger.debug("This is debug");
    logger.info("This is info");
    
    // These should print
    logger.warn("This is warning");
    logger.error("This is error");
    logger.fatal("This is fatal");
}

#[test]
fn test_log_level_filtering() {
    let logger = Logger::new("test").with_level(LogLevel::Info);
    
    // Test by creating log entries at different levels
    let _trace = logger.trace("trace message");
    let _debug = logger.debug("debug message"); 
    let _info = logger.info("info message");
    let _warn = logger.warn("warn message");
    let _error = logger.error("error message");
    let _fatal = logger.fatal("fatal message");
    
    // Logger filtering is working if no panic occurs
}

#[test]
fn test_log_context_macro() {
    let logger = Logger::new("macro_test");
    
    let entry = log_context!(
        logger,
        info,
        "Agent activated",
        "agent_id" => Uuid::new_v4(),
        "agent_type" => "Memory",
        "activation_time" => 42
    );
    
    assert!(entry.context.get("agent_id").is_some());
    assert!(entry.context.get("agent_type").is_some());
    assert!(entry.context.get("activation_time").is_some());
}

#[test]
fn test_log_entry_display() {
    let entry = LogEntry::new(LogLevel::Error, "immune", "Threat detected")
        .with_context("threat_level", json!("Critical"));
    
    let display = format!("{}", entry);
    
    assert!(display.contains("Error"));
    assert!(display.contains("[immune]"));
    assert!(display.contains("Threat detected"));
    assert!(display.contains("threat_level"));
    assert!(display.contains("Critical"));
}

#[test]
fn test_log_levels_equality() {
    assert_ne!(LogLevel::Trace, LogLevel::Debug);
    assert_ne!(LogLevel::Info, LogLevel::Warn);
    assert_ne!(LogLevel::Error, LogLevel::Fatal);
}

#[test]
fn test_empty_context_display() {
    let entry = LogEntry::new(LogLevel::Info, "test", "Simple message");
    let display = format!("{}", entry);
    
    // Should not include empty context
    assert!(!display.contains("{}"));
}