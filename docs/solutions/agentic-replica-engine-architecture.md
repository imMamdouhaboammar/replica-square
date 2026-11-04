# Compound Engineering Solution: Agentic Replica Engine Architecture

## Context & Challenge
The legacy `jakeschincariol/replica-skill` provided 11 passive markdown prompt instructions with toy Python scripts relying on standard library modules. While conceptually sound for manual prompting, it suffered from:
1. No automated agentic orchestration or state machine.
2. Fragile single-threaded PNG parsing with hand-rolled deflate logic.
3. Lack of cryptographic evidence anchoring (violating REA standards).
4. No Model Context Protocol (MCP) server for direct invocation by AI agents.
5. Inability to execute deterministically in CI or autonomous agent loops.

## Technical Decision: Why Rust over Go & TypeScript
When building an agentic engine for reverse engineering and software reconstruction:
- **Rust** provides:
  * Zero-cost abstractions and sub-millisecond execution for AST parsing, binary disassembly, and SIMD image diffing.
  * Native, memory-safe data structures for algebraic state machines (`enum` with variants), avoiding the type gymnastics and memory overhead of Go's `interface{}` / reflection.
  * Single statically linked binary (< 15MB) with zero runtime dependencies (no Python runtime, no Node/Bun runtime required for the engine core).
  * Native JSON-RPC 2.0 stdio stream handling for Model Context Protocol (MCP).
- **Go** was evaluated:
  * While Go excels at network microservices and Docker/Kubernetes tooling, it lacks high-performance native image manipulation (SIMD), deep AST reverse-engineering tooling (equivalent to `tree-sitter`/`syn`), and expressive algebraic types for deterministic state machines.

## Architecture Implemented
1. **ReconEngine**: Cryptographic SHA-256 evidence anchoring, surface mapping, and features CSV matrix generation.
2. **EntrepreneurEngine**: Bayesian grievance scoring with time-decay for customer reviews.
3. **ArchitectEngine**: Clean-room PostgreSQL schema synthesis with UUID primary keys, check constraints, foreign keys, and Row Level Security (RLS) policies.
4. **DesignEngine**: WCAG 2.2 Relative Luminance ($L$) mathematical evaluation with automated Tailwind CSS token generation.
5. **DiffEngine**: 80/20 weighted parity calculation (80% features + 20% visual layout via edge density grids).
6. **BrandEngine**: Deep clean-room recursive sweep across repository paths, identifiers, and hex colors.
7. **LaunchEngine**: Store listing metadata linter with anti-claim regexes and field length constraints.
8. **McpServer**: Native JSON-RPC 2.0 stdio server providing tool-calling capabilities to external agents.
