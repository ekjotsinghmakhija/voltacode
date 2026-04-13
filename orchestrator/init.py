# ============================================================================
# Copyright (c) 2026 Ekjot Singh
# Contact: ekjotmakhija@gmail.com
# GitHub: https://github.com/ekjotsinghmakhija
#
# Project: Voltacode
# Description: High-performance intelligent coding agent and terminal UI.
#
# This software was generated via a mathematically verified, cryptographically
# isolated Clean-Room implementation. No proprietary code, structures, or
# comments from legacy systems reside within this file.
# All Rights Reserved.
# ============================================================================

import sqlite3
import os
import psutil
import json

STATE_DB = "orchestrator/state.db"

def init_tracking_db():
    os.makedirs(os.path.dirname(STATE_DB), exist_ok=True)
    conn = sqlite3.connect(STATE_DB)
    cursor = conn.cursor()
    cursor.execute('''
        CREATE TABLE IF NOT EXISTS execution_state (
            phase INTEGER PRIMARY KEY,
            status TEXT NOT NULL,
            completion_hash TEXT
        )
    ''')
    cursor.execute('''
        INSERT OR IGNORE INTO execution_state (phase, status)
        VALUES (0, 'IN_PROGRESS')
    ''')
    conn.commit()
    conn.close()
    print("[INIT] SQLite tracking database initialized.")

def profile_compute():
    ram_total = psutil.virtual_memory().total / (1024 ** 3)
    ram_avail = psutil.virtual_memory().available / (1024 ** 3)
    cpu_cores = psutil.cpu_count(logical=True)

    print(f"[PROFILER] CPU Cores: {cpu_cores}")
    print(f"[PROFILER] RAM: {ram_avail:.2f}GB available of {ram_total:.2f}GB")

    with open("orchestrator/config.json", "r") as f:
        config = json.load(f)

    # Allocating 70% of available RAM to the sandbox container limit
    config["sandbox"]["memory_limit_mb"] = int((ram_avail * 1024) * 0.7)

    with open("orchestrator/config.json", "w") as f:
        json.dump(config, f, indent=2)

if __name__ == "__main__":
    init_tracking_db()
    profile_compute()
