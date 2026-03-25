# BUILD MODE

You are in build mode. Your job is to implement ONE task from the plan, then exit.

## 0a. Read AGENTS.md

Read AGENTS.md to understand build/test/lint commands for this project.

## 0b. Read Implementation Plan

Read IMPLEMENTATION_PLAN.md. Find the first uncompleted task (marked with `- [ ]`).

## 0c. Study Relevant Specs

Read the specification file(s) related to your task to understand requirements.

## 0d. Study Reference Documents

Read the relevant reference documents for context:
- GALAGA_RESEARCH.md — complete game mechanics research
- TECH_STACK.md — architecture, ECS model, project structure
- ASSETS.md — asset plan, sprite sheets, audio

## 0e. Understand Existing Code

Read relevant existing code to understand patterns and conventions.

## 1. Implement the Task

Write code to complete the task:
- Follow the architecture defined in TECH_STACK.md
- Follow existing code patterns
- Keep changes focused on the single task

## 2. Validate

Run the validation command from AGENTS.md (`cargo build` at minimum).

If validation fails:
- Fix the issues
- Run validation again
- Repeat until passing

## 3. Update Plan and Exit

After validation passes:
1. Mark the task complete in IMPLEMENTATION_PLAN.md: `- [ ]` becomes `- [x]`
2. Exit cleanly

The loop will restart with fresh context for the next task.

---

## 99999. GUARDRAILS - READ CAREFULLY

- **DON'T skip validation** — always run `cargo build` before finishing
- **DON'T implement multiple tasks** — one task per iteration
- **DON'T modify unrelated code** — stay focused on the current task
- **DO follow existing code patterns** — consistency matters
- **DO follow the architecture in TECH_STACK.md** — module structure, ECS components, system groups
- **DO update IMPLEMENTATION_PLAN.md** before exiting
