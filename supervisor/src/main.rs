//! Local supervisor / watchdog for the Nemotron-3-Ultra-550B worker.
//!
//! Layer 1 (deterministic) handles ~90% of states. Layer 2 (Ollama,
//! default `qwen3.5:4b`) is consulted only when the state is ambiguous.
//! std-only: no external crates so this builds even when offline.
//!
//! Usage (from repo root):
//!   cargo run -p supervisor -- --once                 # single tick, dry-run
//!   cargo run -p supervisor -- --once --execute       # single tick, spawn opencode
//!   cargo run -p supervisor -- --loop                 # poll loop, dry-run
//!   cargo run -p supervisor -- --coverage             # print COMPLETED vs corpus
//!   cargo run -p supervisor -- --check                # verify completion invariant

use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// ---------------------------------------------------------------- config ---

#[derive(Debug, Clone)]
struct Config {
    repo_root: PathBuf,
    progress_path: PathBuf,
    audit_path: PathBuf,
    status_path: PathBuf,
    test_crate: String,
    poll_secs: u64,
    heartbeat_timeout_secs: u64,
    heartbeat_fresh_secs: u64,
    ollama_host: String,
    ollama_port: u16,
    ollama_model: String,
    ollama_timeout_secs: u64,
    opencode_model: String,
    opencode_path: String,
    max_repairs: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            repo_root: PathBuf::from("."),
            progress_path: PathBuf::from("AI_PROGRESS.md"),
            audit_path: PathBuf::from("AI_AUDIT.md"),
            status_path: PathBuf::from("status.txt"),
            test_crate: "semantic_ir".to_string(),
            poll_secs: 30,
            heartbeat_timeout_secs: 600,
            heartbeat_fresh_secs: 300,
            ollama_host: "127.0.0.1".to_string(),
            ollama_port: 11434,
            ollama_model: "qwen3.5:4b".to_string(),
            ollama_timeout_secs: 60,
            opencode_model: String::new(),
            opencode_path: r"C:\Users\claimoar\AppData\Local\hermes\node\node_modules\opencode-ai\bin\opencode.exe".to_string(),
            max_repairs: 3,
        }
    }
}

/// Minimal JSON extractor (string / u64 / u16 / bool) — avoids serde.
fn jstr(json: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\"");
    let i = json.find(&pat)?;
    let after = &json[i + pat.len()..];
    let colon = after.find(':')?;
    let v = after[colon + 1..].trim_start();
    if v.starts_with('"') {
        let end = v[1..].find('"')?;
        Some(v[1..1 + end].to_string())
    } else {
        None
    }
}

fn jnum<T: std::str::FromStr>(json: &str, key: &str) -> Option<T> {
    let pat = format!("\"{key}\"");
    let i = json.find(&pat)?;
    let after = &json[i + pat.len()..];
    let colon = after.find(':')?;
    let v = after[colon + 1..].trim_start();
    let end = v
        .find(|c: char| c == ',' || c == '}' || c == '\n' || c == ' ')
        .unwrap_or(v.len());
    v[..end].trim_matches('"').parse().ok()
}

fn jbool(json: &str, key: &str) -> Option<bool> {
    jnum::<String>(json, key).and_then(|s| s.parse().ok())
}

impl Config {
    fn load(repo_root: &Path, path: &Path) -> Self {
        let mut cfg = Config::default();
        cfg.repo_root = repo_root.to_path_buf();
        let Ok(text) = fs::read_to_string(path) else {
            return cfg;
        };
        if let Some(v) = jstr(&text, "progress_path") {
            cfg.progress_path = PathBuf::from(v);
        }
        if let Some(v) = jstr(&text, "audit_path") {
            cfg.audit_path = PathBuf::from(v);
        }
        if let Some(v) = jstr(&text, "status_path") {
            cfg.status_path = PathBuf::from(v);
        }
        if let Some(v) = jstr(&text, "test_crate") {
            cfg.test_crate = v;
        }
        if let Some(v) = jnum(&text, "poll_secs") {
            cfg.poll_secs = v;
        }
        if let Some(v) = jnum(&text, "heartbeat_timeout_secs") {
            cfg.heartbeat_timeout_secs = v;
        }
        if let Some(v) = jnum(&text, "heartbeat_fresh_secs") {
            cfg.heartbeat_fresh_secs = v;
        }
        if let Some(v) = jstr(&text, "ollama_host") {
            cfg.ollama_host = v;
        }
        if let Some(v) = jnum(&text, "ollama_port") {
            cfg.ollama_port = v;
        }
        if let Some(v) = jstr(&text, "ollama_model") {
            cfg.ollama_model = v;
        }
        if let Some(v) = jnum(&text, "ollama_timeout_secs") {
            cfg.ollama_timeout_secs = v;
        }
        if let Some(v) = jstr(&text, "opencode_model") {
            cfg.opencode_model = v;
        }
        if let Some(v) = jstr(&text, "opencode_path") {
            cfg.opencode_path = v;
        }
        if let Some(v) = jnum(&text, "max_repairs_per_instruction") {
            cfg.max_repairs = v;
        }
        let _ = jbool(&text, "auto_continue"); // reserved; CLI --execute wins
        cfg
    }

    fn abs(&self, p: &Path) -> PathBuf {
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            self.repo_root.join(p)
        }
    }
}

// --------------------------------------------------------------- progress ---

#[derive(Debug, Default)]
struct Progress {
    in_progress: String,
    completed: Vec<String>,
    need_continue: bool,
    has_checkpoint: bool,
    heartbeat_present: bool,
}

fn parse_progress(text: &str) -> Progress {
    let mut p = Progress::default();
    let mut in_comment = false;
    for line in text.lines() {
        let t = line.trim();
        // Skip HTML comment blocks (examples live inside them).
        if t.contains("<!--") {
            in_comment = true;
        }
        if in_comment {
            if t.contains("-->") {
                in_comment = false;
            }
            continue;
        }
        if t.starts_with("-->") {
            continue;
        }
        if let Some(v) = t.strip_prefix("IN_PROGRESS:") {
            p.in_progress = v.trim().to_string();
        } else if let Some(v) = t.strip_prefix("COMPLETED:") {
            // COMPLETED: <name> | ... — keep name part only
            let name = v.split('|').next().unwrap_or("").trim().to_string();
            if !name.is_empty() {
                p.completed.push(name);
            }
        } else if t.starts_with("NEED_CONTINUE") {
            p.need_continue = true;
        } else if t.starts_with("CHECKPOINT") {
            p.has_checkpoint = true;
        } else if t.starts_with("HEARTBEAT:") {
            p.heartbeat_present = true;
        }
    }
    if p.in_progress.is_empty() {
        p.in_progress = "none".to_string();
    }
    p
}

// ----------------------------------------------------------------- actions ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Idle, // worker active — do nothing
    Continue,
    Repair,
    NewSession,
    Retry,
    Stop,
}

impl Action {
    fn as_str(self) -> &'static str {
        match self {
            Action::Idle => "IDLE",
            Action::Continue => "CONTINUE",
            Action::Repair => "REPAIR",
            Action::NewSession => "NEW_SESSION",
            Action::Retry => "RETRY",
            Action::Stop => "STOP",
        }
    }

    fn from_token(tok: &str) -> Option<Self> {
        let u = tok.trim().to_uppercase();
        // Handle NEW_SESSION variants first (underscore/space splits naive word scan).
        if u.contains("NEW_SESSION") || u.contains("NEWSESSION") || u.contains("NEW SESSION") {
            return Some(Action::NewSession);
        }
        for word in u.split(|c: char| !c.is_ascii_alphabetic()) {
            match word {
                "CONTINUE" => return Some(Action::Continue),
                "REPAIR" => return Some(Action::Repair),
                "NEWSESSION" | "NEW_SESSION" => return Some(Action::NewSession),
                "RETRY" => return Some(Action::Retry),
                "STOP" => return Some(Action::Stop),
                "IDLE" => return Some(Action::Idle),
                _ => {}
            }
        }
        None
    }
}

#[derive(Debug)]
struct Snapshot {
    progress: Progress,
    progress_age_secs: u64,
    progress_missing: bool,
    changed_files: Vec<String>,
    forbidden_hit: Vec<String>,
    tests_pass: Option<bool>,
    tests_output_tail: String,
    repairs_used: u32,
}

/// Deterministic Layer-1 decision. `needs_llm=true` means ambiguous → Layer 2.
fn decide(snap: &Snapshot, cfg: &Config) -> (Action, String, bool) {
    // 0. No progress file at all → fresh start.
    if snap.progress_missing {
        return (
            Action::Continue,
            "AI_PROGRESS.md missing; start worker with WORKER_PROMPT.md".to_string(),
            false,
        );
    }
    // 1. Context-exhaustion handshake always wins.
    if snap.progress.need_continue || snap.has_checkpoint_for_current() {
        return (
            Action::NewSession,
            "worker wrote CHECKPOINT/NEED_CONTINUE; start fresh opencode session".to_string(),
            false,
        );
    }
    // 2. Forbidden paths or patterns → stop/repair, never continue.
    if let Some(bad) = snap.changed_files.iter().find(|f| {
        !(f.starts_with("semantic_ir/src/")
            || f.starts_with("semantic_ir/tests/")
            || f.starts_with("AI_")
            || *f == "status.txt"
            || *f == "supervisor.json"
            || f.starts_with("supervisor/"))
    }) {
        return (
            Action::Stop,
            format!("git diff touches forbidden path: {bad}"),
            false,
        );
    }
    if !snap.forbidden_hit.is_empty() {
        return (
            Action::Repair,
            format!("forbidden pattern in src: {}", snap.forbidden_hit[0]),
            false,
        );
    }
    // 3. Tests failing → repair (bounded).
    if snap.tests_pass == Some(false) {
        if snap.repairs_used >= cfg.max_repairs {
            return (
                Action::Stop,
                format!("tests failing after {} repairs; needs human", snap.repairs_used),
                false,
            );
        }
        return (
            Action::Repair,
            "cargo test failing; ask worker to fix".to_string(),
            false,
        );
    }
    let idle_work = !snap.progress.in_progress.eq_ignore_ascii_case("none");
    // 4. Heartbeat stale while work claimed in-progress → worker stopped.
    if idle_work && snap.progress_age_secs > cfg.heartbeat_timeout_secs {
        return (
            Action::Retry,
            format!(
                "heartbeat stale ({}s > {}s) with IN_PROGRESS={}; re-drive worker",
                snap.progress_age_secs,
                cfg.heartbeat_timeout_secs,
                snap.progress.in_progress
            ),
            false,
        );
    }
    // 5. Worker active → do nothing.
    if idle_work && snap.progress_age_secs <= cfg.heartbeat_fresh_secs {
        return (
            Action::Idle,
            format!(
                "worker active (progress mtime {}s ago, IN_PROGRESS={})",
                snap.progress_age_secs, snap.progress.in_progress
            ),
            false,
        );
    }
    // 6. Between fresh and timeout with no other signal → ambiguous: ask LLM.
    if idle_work {
        return (
            Action::Continue,
            format!(
                "ambiguous: IN_PROGRESS={} but heartbeat age {}s (fresh<={}s, stale>{}s)",
                snap.progress.in_progress,
                snap.progress_age_secs,
                cfg.heartbeat_fresh_secs,
                cfg.heartbeat_timeout_secs
            ),
            true,
        );
    }
    // 7. No work in progress and tests green → continue to next instruction.
    (Action::Continue, "no IN_PROGRESS; advance to next corpus item".to_string(), false)
}

impl Snapshot {
    fn has_checkpoint_for_current(&self) -> bool {
        // CHECKPOINT without a matching COMPLETED for the in-progress item.
        if !(self.progress.has_checkpoint || self.progress.need_continue) {
            return false;
        }
        let cur = self.progress.in_progress.to_lowercase();
        if cur == "none" {
            return true;
        }
        !self
            .progress
            .completed
            .iter()
            .any(|c| c.to_lowercase() == cur)
    }
}

// ------------------------------------------------------------ shell helpers ---

fn run_cmd(dir: &Path, prog: &str, args: &[&str]) -> (Option<i32>, String) {
    #[cfg(windows)]
    {
        let mut cmd_args = vec!["/c", prog];
        cmd_args.extend(args);
        let out = Command::new("cmd").args(&cmd_args).current_dir(dir).output();
        match out {
            Err(e) => (None, format!("spawn cmd /c {prog} failed: {e}")),
            Ok(o) => {
                let mut s = String::from_utf8_lossy(&o.stdout).into_owned();
                let e = String::from_utf8_lossy(&o.stderr);
                if !e.is_empty() {
                    s.push_str("\n[stderr]\n");
                    s.push_str(&e);
                }
                if s.len() > 4000 {
                    s = format!("...[truncated]...\n{}", &s[s.len() - 4000..]);
                }
                (o.status.code(), s)
            }
        }
    }
    #[cfg(not(windows))]
    {
        let out = Command::new(prog).args(args).current_dir(dir).output();
        match out {
            Err(e) => (None, format!("spawn {prog} failed: {e}")),
            Ok(o) => {
                let mut s = String::from_utf8_lossy(&o.stdout).into_owned();
                let e = String::from_utf8_lossy(&o.stderr);
                if !e.is_empty() {
                    s.push_str("\n[stderr]\n");
                    s.push_str(&e);
                }
                if s.len() > 4000 {
                    s = format!("...[truncated]...\n{}", &s[s.len() - 4000..]);
                }
                (o.status.code(), s)
            }
        }
    }
}

fn git_changed_files(repo: &Path) -> Vec<String> {
    let (code, out) = run_cmd(repo, "git", &["diff", "--name-only"]);
    if code != Some(0) {
        return Vec::new();
    }
    out.lines()
        .map(|l| l.trim().trim_start_matches("[stdout]").trim().to_string())
        .filter(|l| !l.is_empty() && !l.starts_with("[stderr]"))
        .collect()
}

fn file_age_secs(p: &Path) -> Option<u64> {
    let m = fs::metadata(p).ok()?.modified().ok()?;
    SystemTime::now().duration_since(m).ok().map(|d| d.as_secs())
}

fn forbidden_scan(repo: &Path) -> Vec<String> {
    let src = repo.join("semantic_ir").join("src");
    let Ok(entries) = fs::read_dir(&src) else {
        return Vec::new();
    };
    let mut hits = Vec::new();
    for e in entries.flatten() {
        let path = e.path();
        if path.extension().map(|x| x != "rs").unwrap_or(true) {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        for (i, line) in text.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("//") {
                continue;
            }
            if t.contains("unsafe_code") {
                continue; // e.g. #![forbid(unsafe_code)] — not an `unsafe` block
            }
            // Match `unsafe` as a standalone token, not a substring.
            let has_unsafe = t
                .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .any(|tok| tok == "unsafe");
            if has_unsafe || t.contains("panic!") || t.contains("unwrap()") {
                hits.push(format!("src/{name}:{}: {}", i + 1, t.trim()));
                if hits.len() >= 5 {
                    return hits;
                }
            }
        }
    }
    hits
}

/// Directory to run `cargo test` in: the standalone crate dir when it exists
/// (repo root has no Cargo.toml, so `cargo test -p` from root fails).
fn crate_dir(repo: &Path, krate: &str) -> PathBuf {
    let c = repo.join(krate);
    if c.join("Cargo.toml").is_file() {
        c
    } else {
        repo.to_path_buf()
    }
}

fn cargo_test(repo: &Path, krate: &str) -> (Option<bool>, String) {
    // The crate may be a standalone dir (repo/<crate>/Cargo.toml) rather than
    // a workspace member — `cargo test -p` fails without a workspace root, so
    // run inside the crate dir when it exists.
    let dir = crate_dir(repo, krate);
    let (code, out) = run_cmd(&dir, "cargo", &["test"]);
    match code {
        Some(0) => (Some(true), out),
        Some(_) => (Some(false), out),
        None => (None, out),
    }
}

fn epoch_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// ------------------------------------------------------------------ ollama ---

fn json_escape(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o
}

/// Minimal Ollama `/api/generate` client over raw TCP (no deps).
/// Returns the constrained action token, if one was produced.
fn query_ollama(
    host: &str,
    port: u16,
    model: &str,
    state_summary: &str,
    timeout_secs: u64,
) -> Option<Action> {
    let prompt = format!(
        "You are a watchdog supervisor for an ARM64 translation worker.\n\
         Reply with EXACTLY ONE token, nothing else: CONTINUE, REPAIR, NEW_SESSION, RETRY, IDLE, or STOP.\n\
         Rules: CONTINUE only if worker is done and tests pass. REPAIR if tests fail or worker claims done but evidence disagrees. \
         NEW_SESSION if checkpoint/context-exhaustion is mentioned. RETRY if worker is idle/stale. IDLE if worker is actively progressing. \
         STOP if forbidden files or repeated failure.\n\
         State:\n{state_summary}\nToken:"
    );
    let body = format!(
        "{{\"model\":\"{}\",\"prompt\":\"{}\",\"stream\":false}}",
        json_escape(model),
        json_escape(&prompt)
    );
    let req = format!(
        "POST /api/generate HTTP/1.0\r\nHost: {host}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    let addr = format!("{host}:{port}");
    let mut stream = TcpStream::connect(addr).ok()?;
    stream
        .set_read_timeout(Some(Duration::from_secs(timeout_secs.max(5))))
        .ok()?;
    stream
        .set_write_timeout(Some(Duration::from_secs(10)))
        .ok()?;
    stream.write_all(req.as_bytes()).ok()?;
    let mut resp = String::new();
    stream.read_to_string(&mut resp).ok()?;
    // Body starts after blank line; response JSON contains "response":"...".
    let body_start = resp.find("\r\n\r\n").map(|i| i + 4).unwrap_or(0);
    let body = &resp[body_start..];
    // Find "response":"..." value with minimal unescaping.
    let key = "\"response\"";
    let ki = body.find(key)?;
    let after = &body[ki + key.len()..];
    let colon = after.find(':')?;
    let mut v = after[colon + 1..].trim_start();
    if v.starts_with('"') {
        v = &v[1..];
        let mut out = String::new();
        let mut esc = false;
        for c in v.chars() {
            if esc {
                match c {
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    c => out.push(c),
                }
                esc = false;
            } else if c == '\\' {
                esc = true;
            } else if c == '"' {
                break;
            } else {
                out.push(c);
            }
        }
        return Action::from_token(&out);
    }
    Action::from_token(v)
}

// ------------------------------------------------------------------ driver ---

fn action_prompt(repo: &Path, action: Action, snap: &Snapshot) -> String {
    // WORKER_PROMPT.md lives at the repo root. Prefer repo/WORKER_PROMPT.md
    // (correct when cwd is the repo root); fall back to parent of repo for
    // legacy cwd=supervisor/ invocations.
    let direct = repo.join("WORKER_PROMPT.md");
    let worker_prompt_path = if direct.is_file() {
        direct
    } else {
        repo.parent().unwrap_or(repo).join("WORKER_PROMPT.md")
    };
    let base = std::fs::read_to_string(&worker_prompt_path).unwrap_or_default();
    let head = base.lines().take(8).collect::<Vec<_>>().join("\n");
    match action {
        Action::Continue | Action::Retry => format!(
            "{head}\n\nSupervisor action: CONTINUE/RETRY. State: IN_PROGRESS={} completed={} tests={:?}. Read AI_*.md and resume the loop now.",
            snap.progress.in_progress,
            snap.progress.completed.len(),
            snap.tests_pass
        ),
        Action::Repair => format!(
            "{head}\n\nSupervisor action: REPAIR. IN_PROGRESS={} tests={:?} forbidden={:?}. Tail:\n{}. Fix, re-run cargo test, update AI_PROGRESS.md.",
            snap.progress.in_progress,
            snap.tests_pass,
            snap.forbidden_hit,
            snap.tests_output_tail
        ),
        Action::NewSession => format!(
            "{head}\n\nSupervisor action: NEW_SESSION from checkpoint. Read CHECKPOINT in AI_PROGRESS.md and resume from the recorded next instruction.",
        ),
        Action::Stop | Action::Idle => String::new(),
    }
}

fn spawn_opencode(repo: &Path, cfg: &Config, action: Action, snap: &Snapshot, cont: bool) {
    let prompt = action_prompt(repo, action, snap);
    if prompt.is_empty() {
        println!("supervisor: {} — no opencode invocation needed", action.as_str());
        return;
    }
    let mut args: Vec<String> = vec!["run".to_string()];
    if cont {
        args.push("--continue".to_string());
    }
    if !cfg.opencode_model.is_empty() {
        args.push("-m".to_string());
        args.push(cfg.opencode_model.clone());
    }
    // The prompt is a positional argument, not a --prompt flag
    args.push(prompt);
    let opencode_cmd = &cfg.opencode_path;
    println!("supervisor: spawning: {} {}", opencode_cmd, args.join(" "));
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let (code, out) = run_cmd(repo, opencode_cmd, &arg_refs);
    println!("supervisor: opencode exit={code:?}\n{out}");
}

fn write_status(cfg: &Config, action: Action, reason: &str, next: &str) {
    let line = format!(
        "{} | epoch:{} | {} | next={}\n",
        action.as_str(),
        epoch_now(),
        reason.replace('\n', " "),
        next
    );
    let p = cfg.abs(&cfg.status_path);
    // Best-effort atomic-ish write.
    let tmp = p.with_extension("tmp");
    if fs::write(&tmp, &line).is_ok() {
        let _ = fs::rename(&tmp, &p);
    } else {
        let _ = fs::write(&p, &line);
    }
}

fn append_audit(cfg: &Config, action: Action, reason: &str) {
    let line = format!(
        "epoch:{} {} {}\n",
        epoch_now(),
        action.as_str(),
        reason.replace('\n', " ")
    );
    let p = cfg.abs(&cfg.audit_path);
    let mut existing = fs::read_to_string(&p).unwrap_or_default();
    existing.push_str(&line);
    let _ = fs::write(&p, existing);
}

// ------------------------------------------------------------------- ticks ---

fn collect_snapshot(cfg: &Config, run_tests: bool) -> Snapshot {
    let ppath = cfg.abs(&cfg.progress_path);
    let text = fs::read_to_string(&ppath);
    let progress_missing = text.is_err();
    let progress = text.map(|t| parse_progress(&t)).unwrap_or_default();
    let age = file_age_secs(&ppath).unwrap_or(u64::MAX / 2);
    let changed = git_changed_files(&cfg.repo_root);
    let forbidden = forbidden_scan(&cfg.repo_root);
    let (tests_pass, tail) = if run_tests {
        cargo_test(&cfg.repo_root, &cfg.test_crate)
    } else {
        (None, "skipped".to_string())
    };
    // repairs_used: count REPAIR lines in audit for current instruction (best effort).
    let repairs_used = fs::read_to_string(cfg.abs(&cfg.audit_path))
        .map(|a| {
            a.lines()
                .filter(|l| {
                    l.contains("REPAIR") && l.contains(&progress.in_progress)
                        || l.contains("REPAIR") && progress.in_progress == "none"
                })
                .count() as u32
        })
        .unwrap_or(0);
    Snapshot {
        progress,
        progress_age_secs: if age == u64::MAX / 2 { 999_999_999 } else { age },
        progress_missing,
        changed_files: changed,
        forbidden_hit: forbidden,
        tests_pass,
        tests_output_tail: tail,
        repairs_used,
    }
}

fn state_summary(snap: &Snapshot) -> String {
    format!(
        "instruction={} completed={} tests={:?} changed={:?} forbidden={:?} age_s={} need_continue={} checkpoint={}",
        snap.progress.in_progress,
        snap.progress.completed.len(),
        snap.tests_pass,
        snap.changed_files,
        snap.forbidden_hit,
        snap.progress_age_secs,
        snap.progress.need_continue,
        snap.progress.has_checkpoint
    )
}

fn tick(cfg: &Config, execute: bool, use_llm: bool, run_tests: bool) -> Action {
    let snap = collect_snapshot(cfg, run_tests);
    let (mut action, mut reason, needs_llm) = decide(&snap, cfg);
    if needs_llm && use_llm {
        let summary = state_summary(&snap);
        match query_ollama(
            &cfg.ollama_host,
            cfg.ollama_port,
            &cfg.ollama_model,
            &summary,
            cfg.ollama_timeout_secs,
        ) {
            Some(llm_action) => {
                reason = format!("ollama({}) -> {} | was: {reason}", cfg.ollama_model, llm_action.as_str());
                action = llm_action;
            }
            None => {
                reason = format!("ollama unreachable; deterministic fallback: {reason}");
                action = Action::Retry; // safe: re-drive worker once
            }
        }
    } else if needs_llm {
        reason = format!("{reason} [llm disabled; treating as RETRY]");
        action = Action::Retry;
    }
    let next = if snap.progress.in_progress.eq_ignore_ascii_case("none") {
        snap
            .progress
            .completed
            .last()
            .cloned()
            .unwrap_or_else(|| "first-corpus-item".to_string())
    } else {
        snap.progress.in_progress.clone()
    };
    println!("supervisor: {} — {}", action.as_str(), reason);
    println!("supervisor: state: {}", state_summary(&snap));
    write_status(cfg, action, &reason, &next);
    append_audit(cfg, action, &reason);
    if execute {
        match action {
            Action::Idle | Action::Stop => {
                if action == Action::Stop {
                    eprintln!("supervisor: STOP — human intervention required");
                }
            }
            Action::NewSession => spawn_opencode(&cfg.repo_root, cfg, action, &snap, false),
            _ => spawn_opencode(&cfg.repo_root, cfg, action, &snap, true),
        }
    } else {
        println!("supervisor: dry-run (pass --execute to spawn opencode)");
    }
    action
}

fn coverage(cfg: &Config) {
    let base = fs::read_to_string(cfg.repo_root.join("BASE_INTRUCTS.MD")).unwrap_or_default();
    let total = base
        .lines()
        .filter(|l| !l.trim().is_empty())
        .count();
    let prog = fs::read_to_string(cfg.abs(&cfg.progress_path)).unwrap_or_default();
    let p = parse_progress(&prog);
    println!(
        "coverage: {}/{} COMPLETED in ledger ({} total lines in BASE_INTRUCTS.MD)",
        p.completed.len(),
        total,
        total
    );
    for c in &p.completed {
        println!("  COMPLETED: {c}");
    }
    println!("IN_PROGRESS: {}", p.in_progress);
}

fn check_invariant(cfg: &Config) -> i32 {
    // --check verifies the hard gate for the current IN_PROGRESS item.
    let snap = collect_snapshot(cfg, true);
    let cur = snap.progress.in_progress.clone();
    if cur.eq_ignore_ascii_case("none") {
        println!("check: no IN_PROGRESS — nothing to gate");
        return 0;
    }
    let mut fail = 0;
    if snap.tests_pass != Some(true) {
        println!("check FAIL: cargo test not green: {:?}", snap.tests_pass);
        fail = 1;
    }
    if !snap.forbidden_hit.is_empty() {
        println!("check FAIL: forbidden patterns: {:?}", snap.forbidden_hit);
        fail = 1;
    }
    let bad: Vec<_> = snap
        .changed_files
        .iter()
        .filter(|f| {
            !(f.starts_with("semantic_ir/src/")
                || f.starts_with("semantic_ir/tests/")
                || f.starts_with("AI_")
                || *f == "status.txt"
                || *f == "supervisor.json"
                || f.starts_with("supervisor/"))
        })
        .collect();
    if !bad.is_empty() {
        println!("check FAIL: forbidden git paths: {bad:?}");
        fail = 1;
    }
    if fail == 0 {
        println!("check PASS: {cur} satisfies completion gate (tests green, diff sane)");
    }
    println!("tail:\n{}", snap.tests_output_tail);
    fail
}

fn usage() -> ! {
    eprintln!(
        "supervisor — deterministic watchdog + Ollama fallback\n\
         \n\
         --once [--execute] [--no-tests] [--no-llm]   single tick (default)\n\
         --loop [--execute]                           poll every supervisor.json poll_secs\n\
         --coverage                                   COMPLETED vs BASE_INTRUCTS.MD\n\
         --check                                      verify completion gate for IN_PROGRESS\n\
         --config <path>                              default: supervisor.json\n\
         --no-llm disables Ollama fallback (deterministic RETRY on ambiguous)\n\
         --no-tests skips cargo test (fast state probe; gate not verified)\n\
         --execute actually spawns `opencode run`; without it: dry-run"
    );
    std::process::exit(2);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut config_path = PathBuf::from("supervisor.json");
    let mut mode = "once";
    let mut execute = false;
    let mut use_llm = true;
    let mut run_tests = true;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--once" => mode = "once",
            "--loop" => mode = "loop",
            "--coverage" => mode = "coverage",
            "--check" => mode = "check",
            "--execute" => execute = true,
            "--no-llm" => use_llm = false,
            "--no-tests" => run_tests = false,
            "--config" => {
                i += 1;
                if i >= args.len() {
                    usage();
                }
                config_path = PathBuf::from(&args[i]);
            }
            "-h" | "--help" => usage(),
            _ => usage(),
        }
        i += 1;
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let cfg = Config::load(&cwd, &cwd.join(&config_path));
    match mode {
        "coverage" => coverage(&cfg),
        "check" => std::process::exit(check_invariant(&cfg)),
        "loop" => loop {
            tick(&cfg, execute, use_llm, run_tests);
            std::thread::sleep(Duration::from_secs(cfg.poll_secs.max(5)));
        },
        _ => {
            tick(&cfg, execute, use_llm, run_tests);
        }
    }
}

// ------------------------------------------------------------------- tests ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_parses_ledger() {
        let p = parse_progress(
            "HEARTBEAT: x\nIN_PROGRESS: ADCS\nCOMPLETED: SUBS (shifted) | tests/x.rs | t\nNEED_CONTINUE\n",
        );
        assert_eq!(p.in_progress, "ADCS");
        assert_eq!(p.completed, vec!["SUBS (shifted)".to_string()]);
        assert!(p.need_continue);
    }

    #[test]
    fn checkpoint_triggers_new_session() {
        let snap = Snapshot {
            progress: Progress {
                in_progress: "ADCS".to_string(),
                completed: vec!["SUBS".to_string()],
                need_continue: false,
                has_checkpoint: true,
                heartbeat_present: true,
            },
            progress_age_secs: 10,
            progress_missing: false,
            changed_files: vec![],
            forbidden_hit: vec![],
            tests_pass: Some(true),
            tests_output_tail: String::new(),
            repairs_used: 0,
        };
        let cfg = Config::default();
        let (a, _, llm) = decide(&snap, &cfg);
        assert_eq!(a, Action::NewSession);
        assert!(!llm);
    }

    #[test]
    fn failing_tests_trigger_repair_then_stop() {
        let mut snap = Snapshot {
            progress: Progress {
                in_progress: "ADCS".to_string(),
                completed: vec![],
                need_continue: false,
                has_checkpoint: false,
                heartbeat_present: true,
            },
            progress_age_secs: 10,
            progress_missing: false,
            changed_files: vec![],
            forbidden_hit: vec![],
            tests_pass: Some(false),
            tests_output_tail: String::new(),
            repairs_used: 0,
        };
        let cfg = Config::default();
        assert_eq!(decide(&snap, &cfg).0, Action::Repair);
        snap.repairs_used = 99;
        assert_eq!(decide(&snap, &cfg).0, Action::Stop);
    }

    #[test]
    fn stale_heartbeat_triggers_retry() {
        let snap = Snapshot {
            progress: Progress {
                in_progress: "ADCS".to_string(),
                completed: vec![],
                need_continue: false,
                has_checkpoint: false,
                heartbeat_present: true,
            },
            progress_age_secs: 9999,
            progress_missing: false,
            changed_files: vec![],
            forbidden_hit: vec![],
            tests_pass: Some(true),
            tests_output_tail: String::new(),
            repairs_used: 0,
        };
        let cfg = Config::default();
        assert_eq!(decide(&snap, &cfg).0, Action::Retry);
    }

    #[test]
    fn forbidden_path_stops() {
        let snap = Snapshot {
            progress: Progress {
                in_progress: "ADCS".to_string(),
                completed: vec![],
                need_continue: false,
                has_checkpoint: false,
                heartbeat_present: true,
            },
            progress_age_secs: 10,
            progress_missing: false,
            changed_files: vec!["semantic_ir/Cargo.toml".to_string()],
            forbidden_hit: vec![],
            tests_pass: Some(true),
            tests_output_tail: String::new(),
            repairs_used: 0,
        };
        let cfg = Config::default();
        assert_eq!(decide(&snap, &cfg).0, Action::Stop);
    }

    #[test]
    fn progress_ignores_html_comment_examples() {
        let p = parse_progress(
            "<!--\nCOMPLETED: EXAMPLE | x | t\nIN_PROGRESS: EXAMPLE\nCHECKPOINT: x\n-->\nHEARTBEAT: x\nIN_PROGRESS: none\n",
        );
        assert_eq!(p.in_progress, "none");
        assert!(p.completed.is_empty());
        assert!(!p.has_checkpoint);
        assert!(p.heartbeat_present);
    }

    #[test]
    fn cargo_dir_prefers_standalone_crate_dir() {
        // Regression: repo root has no Cargo.toml, so gating must run inside
        // repo/semantic_ir. Pure path logic — does not invoke cargo.
        let tmp = std::env::temp_dir().join("supervisor_test_repo");
        let _ = std::fs::create_dir_all(tmp.join("semantic_ir"));
        let _ = std::fs::write(tmp.join("semantic_ir").join("Cargo.toml"), "[package]\n");
        assert_eq!(
            super::crate_dir(&tmp, "semantic_ir"),
            tmp.join("semantic_ir")
        );
        assert_eq!(
            super::crate_dir(&tmp, "missing_crate"),
            tmp
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn action_token_parsing_is_strict() {
        assert_eq!(Action::from_token("CONTINUE"), Some(Action::Continue));
        assert_eq!(Action::from_token("  repair the code"), Some(Action::Repair));
        assert_eq!(Action::from_token("new_session"), Some(Action::NewSession));
        assert_eq!(Action::from_token("hello world"), None);
    }
}
