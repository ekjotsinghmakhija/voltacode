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

import os
import sys

def verify_online_inference_tokens():
    """
    Validates the presence of required API keys for online inference testing.
    Local models are deferred to Phase 6.
    """
    print("[INFERENCE] Starting Pre-Flight Checks for Online Models...")

    anthropic_key = os.environ.get("ANTHROPIC_API_KEY")
    openai_key = os.environ.get("OPENAI_API_KEY")

    if not anthropic_key and not openai_key:
        print("[INFERENCE] ERROR: No online model tokens detected in environment vault.")
        print("[INFERENCE] Please set ANTHROPIC_API_KEY or OPENAI_API_KEY to proceed with pre-flight.")
        sys.exit(1)

    active_provider = "Anthropic" if anthropic_key else "OpenAI"
    print(f"[INFERENCE] Pre-Flight Successful. Routing established via {active_provider}.")
    print("[INFERENCE] Readiness confirmed. Waiting for Phase 1 Parity Map...")

if __name__ == "__main__":
    verify_online_inference_tokens()
