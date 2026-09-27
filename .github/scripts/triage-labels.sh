#!/usr/bin/env bash
# Guarded wrapper around edit-issue-labels.sh for the automated triage bot.
# Refuses labels that are maintainer decisions: launching a coding agent
# (agent:*), security handling, and review gates. The triage bot can
# categorize; it cannot escalate.
set -euo pipefail
for a in "$@"; do
  case "$a" in
    agent:*|security*|needs-human-review|spec-change|priority:critical)
      echo "Error: label '$a' is maintainer-only" >&2; exit 1 ;;
  esac
done
exec "$(dirname "$0")/edit-issue-labels.sh" "$@"
