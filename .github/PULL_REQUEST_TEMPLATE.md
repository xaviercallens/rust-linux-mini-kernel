## What and why

Closes #

## Evidence (required)

<!-- Paste the exact commands you ran and their output. A claim without output will not be reviewed. -->

```text
$ python3 scripts/pr_guard.py --base origin/main

$ <the oracle command from the issue>

```

## Checklist

- [ ] Only the files named in the issue are changed
- [ ] No theorem statement changed or deleted; no new `sorry` / `axiom` / `admit` / `native_decide` / `#[allow]` / `#[ignore]` / `todo!` / `unimplemented!`
- [ ] Every new or touched `unsafe` block has a specific `// SAFETY:` comment, or is listed below as possible UB
- [ ] No new numbers in docs without a script that produces them

## Possible UB / could not do honestly

<!-- List unsafe blocks you could not justify, or parts of the task you could not complete without weakening something. This is a valid outcome. -->

## AI assistance

- Agent / tool: <!-- e.g. Claude Code, Google Jules, none -->
- Model: <!-- e.g. claude-haiku-4-5, gemini-... -->
- What the agent did vs. what you checked yourself:
