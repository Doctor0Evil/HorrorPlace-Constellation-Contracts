#!/usr/bin/env bash
set -euo pipefail

# Horror$Place Constellation Guardrail: Install Hooks
# Creates symlinks for all guardrail hooks in .git/hooks/.

HOOKS_DIR="$(git rev-parse --show-toplevel)/hooks"
GIT_HOOKS_DIR="$(git rev-parse --show-toplevel)/.git/hooks"

declare -a HOOKS=(
  "validate-schemas.sh"
  "lint-registry.sh"
  "lint-prism-envelope.sh"
  "lint-qpudatashard.sh"
)

echo "🔗 Installing Horror$Place Constellation guardrails..."

for hook in "${HOOKS[@]}"; do
  SRC="$HOOKS_DIR/$hook"
  DEST="$GIT_HOOKS_DIR/pre-commit-$hook"
  
  if [ -f "$SRC" ]; then
    ln -sf "$SRC" "$DEST"
    chmod +x "$DEST"
    echo "✅ Installed: $hook -> $DEST"
  else
    echo "❌ Missing: $SRC"
    exit 1
  fi
done

echo ""
echo "🛡️ All guardrails installed successfully."
echo "⚠️  Note: Git runs all pre-commit-* scripts in alphabetical order."
echo "📝 To uninstall, run: rm .git/hooks/pre-commit-*"
