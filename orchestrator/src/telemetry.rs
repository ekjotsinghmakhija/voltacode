// ============================================================================
// Copyright (c) 2026 Ekjot Singh
// Contact: ekjotmakhija@gmail.com
// GitHub: https://github.com/ekjotsinghmakhija
//
// Project: Voltacode
// Description: High-performance intelligent coding agent and terminal UI.
//
// This software was generated via a mathematically verified, cryptographically
// isolated Clean-Room implementation. No proprietary code, structures, or
// comments from legacy systems reside within this file.
// All Rights Reserved.
// ============================================================================

use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn log_event(event_type: &str, payload: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("orchestrator/telemetry.log")
        .expect("Failed to open telemetry log");

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let log_entry = format!(
        "{{\"timestamp\": {}, \"event\": \"{}\", \"payload\": \"{}\"}}\n",
        timestamp, event_type, payload
    );

    file.write_all(log_entry.as_bytes()).unwrap();
}
