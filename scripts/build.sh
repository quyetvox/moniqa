#!/usr/bin/env bash
# =============================================================================
# MoniQA — Build CLI Script
# Usage: ./scripts/build.sh [--platform <target>] [--help]
#
# Platforms:
#   macos, macos-arm     → aarch64-apple-darwin  (Apple Silicon)   [default]
#   macos-intel          → x86_64-apple-darwin   (Intel Mac)
#   macos-universal      → universal-apple-darwin (ARM64 + Intel fat binary)
# =============================================================================

set -euo pipefail

# ─── Colors ──────────────────────────────────────────────────────────────────
RED=$'\033[0;31m'
GREEN=$'\033[0;32m'
YELLOW=$'\033[1;33m'
CYAN=$'\033[0;36m'
BOLD=$'\033[1m'
DIM=$'\033[2m'
RESET=$'\033[0m'

# ─── Helpers ─────────────────────────────────────────────────────────────────
info()    { printf "${CYAN}[INFO]${RESET}  %s\n" "$*"; }
success() { printf "${GREEN}[OK]${RESET}    %s\n" "$*"; }
warn()    { printf "${YELLOW}[WARN]${RESET}  %s\n" "$*"; }
error()   { printf "${RED}[ERROR]${RESET} %s\n" "$*" >&2; }
die()     { error "$*"; exit 1; }

# ─── Defaults ────────────────────────────────────────────────────────────────
DEFAULT_PLATFORM="macos"
PLATFORM=""
SIGN=false
VERBOSE=false

# ─── Usage ───────────────────────────────────────────────────────────────────
usage() {
  printf "\n"
  printf "${BOLD}MoniQA Build Script${RESET}\n"
  printf "${DIM}Rust + Tauri v2 + Svelte 5 — macOS System Monitor${RESET}\n"
  printf "\n"
  printf "${BOLD}USAGE${RESET}\n"
  printf "  ./scripts/build.sh [OPTIONS]\n"
  printf "\n"
  printf "${BOLD}OPTIONS${RESET}\n"
  printf "  ${CYAN}-p, --platform <name>${RESET}   Target platform (default: macos)\n"
  printf "  ${CYAN}-s, --sign${RESET}              Enable code signing (requires Apple Dev cert)\n"
  printf "  ${CYAN}-v, --verbose${RESET}           Show full Rust/Tauri build output\n"
  printf "  ${CYAN}-h, --help${RESET}              Show this help\n"
  printf "\n"
  printf "${BOLD}PLATFORMS${RESET}\n"
  printf "  ${GREEN}macos${RESET}, ${GREEN}macos-arm${RESET}\n"
  printf "      → aarch64-apple-darwin   ${DIM}(Apple Silicon — default)${RESET}\n"
  printf "\n"
  printf "  ${GREEN}macos-intel${RESET}\n"
  printf "      → x86_64-apple-darwin    ${DIM}(Intel Mac)${RESET}\n"
  printf "\n"
  printf "  ${GREEN}macos-universal${RESET}\n"
  printf "      → universal-apple-darwin ${DIM}(ARM64 + Intel, runs on both)${RESET}\n"
  printf "\n"
  printf "${BOLD}EXAMPLES${RESET}\n"
  printf "  ${DIM}# Build for Apple Silicon (default)${RESET}\n"
  printf "  ./scripts/build.sh\n"
  printf "  ./scripts/build.sh -p macos\n"
  printf "\n"
  printf "  ${DIM}# Build for Intel Mac${RESET}\n"
  printf "  ./scripts/build.sh -p macos-intel\n"
  printf "\n"
  printf "  ${DIM}# Build Universal Binary (runs on both)${RESET}\n"
  printf "  ./scripts/build.sh -p macos-universal\n"
  printf "\n"
  printf "  ${DIM}# Verbose mode — show full Rust compile log${RESET}\n"
  printf "  ./scripts/build.sh -p macos-intel -v\n"
  printf "\n"
  exit 0
}

# ─── Parse args ──────────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
  case "$1" in
    -p|--platform)
      PLATFORM="${2:-}"
      [[ -z "$PLATFORM" ]] && die "--platform requires a value"
      shift 2
      ;;
    -s|--sign)
      SIGN=true
      shift
      ;;
    -v|--verbose)
      VERBOSE=true
      shift
      ;;
    -h|--help)
      usage
      ;;
    *)
      die "Unknown option: $1. Use --help for usage."
      ;;
  esac
done

PLATFORM="${PLATFORM:-$DEFAULT_PLATFORM}"

# ─── Resolve platform → Rust target ──────────────────────────────────────────
case "${PLATFORM,,}" in
  macos|macos-arm|arm|arm64)
    RUST_TARGET="aarch64-apple-darwin"
    PLATFORM_LABEL="macOS — Apple Silicon (ARM64)"
    ;;
  macos-intel|intel)
    RUST_TARGET="x86_64-apple-darwin"
    PLATFORM_LABEL="macOS — Intel (x86_64)"
    ;;
  macos-universal|universal)
    RUST_TARGET="universal-apple-darwin"
    PLATFORM_LABEL="macOS — Universal Binary (ARM64 + Intel)"
    ;;
  *)
    error "Unknown platform: '${PLATFORM}'"
    printf "\n"
    printf "  Supported platforms:\n"
    printf "    macos, macos-arm      → Apple Silicon\n"
    printf "    macos-intel           → Intel Mac\n"
    printf "    macos-universal       → Universal Binary\n"
    printf "\n"
    printf "  Run: ./scripts/build.sh --help\n"
    exit 1
    ;;
esac

# ─── Project root ─────────────────────────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

# ─── Banner ───────────────────────────────────────────────────────────────────
printf "\n"
printf "${BOLD}╔══════════════════════════════════════════╗${RESET}\n"
printf "${BOLD}║         MoniQA — Build System            ║${RESET}\n"
printf "${BOLD}╚══════════════════════════════════════════╝${RESET}\n"
printf "\n"
printf "  ${DIM}Platform  :${RESET} ${GREEN}%s${RESET}\n" "$PLATFORM_LABEL"
printf "  ${DIM}Target    :${RESET} ${CYAN}%s${RESET}\n" "$RUST_TARGET"
printf "  ${DIM}Signing   :${RESET} %s\n" "$([ "$SIGN" = true ] && printf "${YELLOW}Enabled${RESET}" || printf "${DIM}Disabled${RESET}")"
printf "  ${DIM}Project   :${RESET} %s\n" "$PROJECT_ROOT"
printf "\n"

# ─── Pre-flight checks ────────────────────────────────────────────────────────
info "Running pre-flight checks..."

command -v rustup &>/dev/null || die "rustup not found — install from https://rustup.rs"
command -v cargo  &>/dev/null || die "cargo not found"
command -v npm    &>/dev/null || die "npm not found"
command -v node   &>/dev/null || die "node not found"

# Install Rust target if not present
if ! rustup target list --installed | grep -q "^${RUST_TARGET}$"; then
  info "Rust target '${RUST_TARGET}' not installed. Installing..."
  rustup target add "$RUST_TARGET" || die "Failed to install Rust target: $RUST_TARGET"
  success "Target installed: $RUST_TARGET"
else
  success "Rust target ready: $RUST_TARGET"
fi

# Intel cross-compile warning
if [[ "$RUST_TARGET" == "x86_64-apple-darwin" ]]; then
  warn "Cross-compiling to Intel x86_64. If build fails, run: xcode-select --install"
fi

# Node modules check
if [[ ! -d "node_modules" ]]; then
  info "node_modules not found. Running npm install..."
  npm install --silent || die "npm install failed"
  success "npm install complete"
fi

# ─── Frontend build ───────────────────────────────────────────────────────────
info "Building Svelte frontend..."
if [[ "$VERBOSE" == true ]]; then
  npm run build || die "Frontend build failed"
else
  npm run build &>/dev/null || die "Frontend build failed"
fi
success "Frontend build complete → build/"

# ─── Tauri release build ──────────────────────────────────────────────────────
info "Building Tauri release (this takes ~2–5 min)..."

BUILD_CMD=("npm" "run" "tauri" "build" "--" "--target" "$RUST_TARGET")

# Disable code signing if not requested
if [[ "$SIGN" == false ]]; then
  export APPLE_SIGNING_IDENTITY="-"
fi

printf "\n"
if [[ "$VERBOSE" == true ]]; then
  "${BUILD_CMD[@]}" || die "Tauri build failed"
else
  "${BUILD_CMD[@]}" 2>&1 \
    | grep --line-buffered -E "(Compiling|Finished|Bundling|Bundle at|error\[|^error)" \
    || die "Tauri build failed"
fi

# ─── Locate output bundles ────────────────────────────────────────────────────
printf "\n"
info "Locating output bundles..."

BUNDLE_DIR="$PROJECT_ROOT/src-tauri/target/${RUST_TARGET}/release/bundle"

APP_PATH=""
DMG_PATH=""

if [[ -d "$BUNDLE_DIR/macos" ]]; then
  APP_PATH=$(find "$BUNDLE_DIR/macos" -name "*.app" -maxdepth 1 | head -1)
fi
if [[ -d "$BUNDLE_DIR/dmg" ]]; then
  DMG_PATH=$(find "$BUNDLE_DIR/dmg" -name "*.dmg" -maxdepth 1 | head -1)
fi

# ─── Summary ─────────────────────────────────────────────────────────────────
printf "\n"
printf "${BOLD}╔══════════════════════════════════════════╗${RESET}\n"
printf "${BOLD}║         Build Complete ✓                 ║${RESET}\n"
printf "${BOLD}╚══════════════════════════════════════════╝${RESET}\n"
printf "\n"
printf "  ${DIM}Platform :${RESET} ${GREEN}%s${RESET}\n" "$PLATFORM_LABEL"
printf "  ${DIM}Target   :${RESET} ${CYAN}%s${RESET}\n" "$RUST_TARGET"
printf "\n"

if [[ -n "$APP_PATH" && -d "$APP_PATH" ]]; then
  APP_SIZE=$(du -sh "$APP_PATH" 2>/dev/null | cut -f1)
  printf "  ${GREEN}✓${RESET} App  : %s ${DIM}(%s)${RESET}\n" "$APP_PATH" "$APP_SIZE"
else
  warn "MoniQA.app not found at expected location."
fi

if [[ -n "$DMG_PATH" && -f "$DMG_PATH" ]]; then
  DMG_SIZE=$(du -sh "$DMG_PATH" 2>/dev/null | cut -f1)
  printf "  ${GREEN}✓${RESET} DMG  : %s ${DIM}(%s)${RESET}\n" "$DMG_PATH" "$DMG_SIZE"
else
  warn "DMG not found at expected location."
fi

printf "\n"
printf "  ${DIM}All bundles: %s${RESET}\n" "$BUNDLE_DIR"
printf "\n"
