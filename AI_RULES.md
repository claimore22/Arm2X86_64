# AI_RULES — hard invariants (supervisor enforces)

## Completion invariant (all must hold before NEXT)

1. `IN_PROGRESS: <instr>` recorded in `AI_PROGRESS.md` before work starts.
2. Implementation exists under `semantic_ir/src/` or documented extension proposal.
3. `validate_instruction` passes for the new instruction.
4. Focused test file `semantic_ir/tests/<name>.rs` exists and passes.
5. Full `cargo test -p semantic_ir` passes.
6. `COMPLETED: <instr>` appended to `AI_PROGRESS.md` with heartbeat timestamp.
7. `git diff --name-only` contains only allowed paths:
   - `semantic_ir/src/**`
   - `semantic_ir/tests/**`
   - `AI_*.md`, `status.txt`, `supervisor.json`
8. Forbidden: `unsafe`, `panic!`, `unwrap()` in `semantic_ir/src/` (supervisor greps).

## Worker constraints

- Exactly one instruction at a time, in `BASE_INTRUCTS.MD` order unless
  `AI_PLAN.md` explicitly reorders.
- Never claim COMPLETE without 1–7 above.
- Never wait for bare "continue" — after COMPLETED, immediately start next
  `IN_PROGRESS`, or write `CHECKPOINT` + `NEED_CONTINUE` if context is low.
- On context pressure: write checkpoint block (current, source path,
  impl state, test state, next) and terminate cleanly.

## Supervisor constraints

- Never mark COMPLETE on worker word alone. Re-verify 2–5 deterministically.
- Structured actions only: CONTINUE | REPAIR | NEW_SESSION | RETRY | STOP.
- Ollama (default `qwen3.5:4b`) is Layer-2 fallback for ambiguous states only.
