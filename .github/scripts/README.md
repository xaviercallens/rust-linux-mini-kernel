# Workflow helper scripts

- `gh.sh`, `edit-issue-labels.sh`: vendored unmodified from
  [anthropics/claude-code-action](https://github.com/anthropics/claude-code-action/tree/main/scripts)
  at commit `756cc22e1966` (MIT License). `gh.sh` allows only read-only `gh` calls;
  `edit-issue-labels.sh` binds the issue number to the triggering event and only
  applies labels that already exist.
- `triage-labels.sh`: RunuX wrapper that additionally refuses maintainer-only
  labels (`agent:*`, `security*`, `needs-human-review`, `spec-change`,
  `priority:critical`), so a prompt-injected triage run cannot escalate.
