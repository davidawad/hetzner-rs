#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"
failed=0

is_private_config() {
  case "$1" in
    .env|.env.*|.ssh|.aws|.kube|.netrc|.npmrc|.pypirc|.git-credentials|*.pem|*.key|id_rsa|id_ed25519|credentials|credentials.*|secrets|secrets.*|*.local)
      return 0 ;;
    *) return 1 ;;
  esac
}

while IFS= read -r -d '' file; do
  rel="${file#./}"
  case "$rel" in
    .beads|.beads/*|.claude|.claude/*|.codex|.codex/*)
      printf 'tooling state: %s\n' "$rel"
      failed=1 ;;
  esac
  base="${rel##*/}"
  if is_private_config "$base"; then
    printf 'private config: %s\n' "$rel"
    failed=1
  fi
done < <(find . -path './.git' -prune -o -print0)

if rg -n --hidden --glob '!.git/**' --glob '!scripts/check-release-tree.sh' \
  '(/Users/|/home/|phos''phorus|car''bon)' .; then
  printf 'release tree contains a local path or machine name\n' >&2
  failed=1
fi

if (( failed )); then
  exit 1
fi
printf 'release tree hygiene check passed\n'
