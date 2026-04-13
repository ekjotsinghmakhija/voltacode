# ============================================================================
# Copyright (c) 2026 Ekjot Singh
# Contact: ekjotmakhija@gmail.com
# GitHub: https://github.com/ekjotsinghmakhija
#
# Project: Voltacode
# Description: High-performance intelligent coding agent and terminal UI.
# ============================================================================

import re

class ContextScrubber:
    def __init__(self):
        # Regex patterns for common secrets and PII to prevent LLM leakage
        self.patterns = {
            "ipv4": r"\b(?:\d{1,3}\.){3}\d{1,3}\b",
            "aws_key": r"(?i)AKIA[0-9A-Z]{16}",
            "generic_api_key": r"(?i)(?:key|token|secret)[\s=:\"\']+[0-9a-zA-Z\-_]{16,}"
        }
        print("[ANALYZER] PII & Secret Scrubber armed.")

    def sanitize_payload(self, text: str) -> str:
        """
        Redacts sensitive infrastructure strings before sending context to the LLM.
        """
        sanitized = text
        for secret_type, pattern in self.patterns.items():
            sanitized = re.sub(pattern, f"[REDACTED_{secret_type.upper()}]", sanitized)

        return sanitized

if __name__ == "__main__":
    scrubber = ContextScrubber()
    test_str = "Connecting to DB at 192.168.1.5 with token=sk-abcdef1234567890qwertyuiop"
    safe_str = scrubber.sanitize_payload(test_str)
    print(f"[TEST] Original: {test_str}")
    print(f"[TEST] Sanitized: {safe_str}")
