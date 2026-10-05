# AI_PLAN — canonical corpus order

Source of truth: `BASE_INTRUCTS.MD` (507 lines), worked top-to-bottom.
Alias rows (e.g. `ASR (immediate): an alias of SBFM`) are implemented via
their canonical target; record alias decision in `AI_DECISIONS.md`.

Pipeline per instruction (matches `semantic_ir/README.md`):

    Stanford pseudocode (`arm64_md_clean/`)
      -> semantic_ir translation (no x86-64 details)
      -> validate_instruction
      -> focused test
      -> full cargo test

Bootstrap set first (already partially done): ADD, SUBS, CSEL, LDR, B.
Then resume in file order from first `[ ]` without coverage.

Supervisor drives `opencode run --continue` / `opencode run -s <id>`
with prompt from `WORKER_PROMPT.md`. New session on context exhaustion:
worker checkpoint -> supervisor starts fresh session with checkpoint text.
