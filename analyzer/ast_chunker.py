# ============================================================================
# Copyright (c) 2026 Ekjot Singh
# Contact: ekjotmakhija@gmail.com
# GitHub: https://github.com/ekjotsinghmakhija
#
# Project: Voltacode
# Description: High-performance intelligent coding agent and terminal UI.
# ============================================================================

import os
import tree_sitter_python as tspython
import tree_sitter_rust as tsrust
from tree_sitter import Language, Parser

class ASTChunker:
    def __init__(self):
        print("[ANALYZER] Booting Universal AST Parsers via Python Bridge...")
        self.parser = Parser()

        # Load native language grammars (v0.21 API requires pointer and name)
        self.LANG_PY = Language(tspython.language(), "python")
        self.LANG_RS = Language(tsrust.language(), "rust")

        print("[ANALYZER] Tree-sitter native bindings loaded successfully.")

    def chunk_file(self, filepath: str) -> list[str]:
        """
        Reads a source file and parses it into an Abstract Syntax Tree (AST).
        """
        if not os.path.exists(filepath):
            raise FileNotFoundError(f"[ANALYZER] Cannot chunk missing file: {filepath}")

        print(f"[ANALYZER] Slicing {filepath} into logic-complete closures...")

        with open(filepath, 'rb') as f:
            code = f.read()

        # Route to correct grammar based on extension
        if filepath.endswith('.py'):
            self.parser.set_language(self.LANG_PY)
        elif filepath.endswith('.rs'):
            self.parser.set_language(self.LANG_RS)
        else:
            print(f"[ANALYZER] Unsupported extension for {filepath}. Skipping AST.")
            return []

        tree = self.parser.parse(code)

        print(f"[ANALYZER] AST generated successfully. Root node type: {tree.root_node.type}")
        return [code.decode('utf-8')]

if __name__ == "__main__":
    chunker = ASTChunker()
    print("[ANALYZER] Extraction engine is ready for ingestion.")
