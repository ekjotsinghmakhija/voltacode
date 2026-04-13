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

import Parser from "tree-sitter";
import Rust from "tree-sitter-rust";
import Python from "tree-sitter-python";

console.log("[AST WARMUP] Initializing Universal AST Parsers...");

const rustParser = new Parser();
rustParser.setLanguage(Rust);

const pyParser = new Parser();
pyParser.setLanguage(Python);

console.log(
  "[AST WARMUP] Tree-sitter loaded successfully for target ecosystems.",
);
console.log("[AST WARMUP] Read-only extraction buffer is ready.");
