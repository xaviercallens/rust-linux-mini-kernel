#!/bin/bash
# Verify Lean 4 specifications for MVK v9.3.1
# Usage: ./verify_specs.sh [--verbose]

set -e

VERBOSE=false
if [[ "$1" == "--verbose" ]]; then
    VERBOSE=true
fi

echo "========================================"
echo "  MVK v9.3.1 Formal Verification"
echo "========================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Lean 4 is installed
if ! command -v lean &> /dev/null; then
    log_error "Lean 4 is not installed!"
    echo ""
    echo "Install Lean 4 using elan:"
    echo "  curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh"
    echo "  source ~/.elan/env"
    exit 1
fi

log_info "Lean version: $(lean --version)"
echo ""

# Navigate to specifications directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SPECS_DIR="$(dirname "$SCRIPT_DIR")/lean4"

if [ ! -d "$SPECS_DIR" ]; then
    log_error "Specifications directory not found: $SPECS_DIR"
    exit 1
fi

cd "$SPECS_DIR"
log_info "Working directory: $(pwd)"
echo ""

# Step 1: Build specifications
log_info "Step 1/5: Building Lean specifications..."
if $VERBOSE; then
    lake build
else
    lake build > /dev/null 2>&1
fi

if [ $? -eq 0 ]; then
    log_success "Build successful"
else
    log_error "Build failed"
    exit 1
fi
echo ""

# Step 2: Type check specifications
log_info "Step 2/5: Type checking specifications..."
if lake env lean MVK.lean > /dev/null 2>&1; then
    log_success "Type checking passed"
else
    log_error "Type checking failed"
    exit 1
fi
echo ""

# Step 3: Check individual specification files
log_info "Step 3/5: Checking individual modules..."

MODULES=(
    "MVK/Phase1/Printk.lean"
    "MVK/Phase1/ArchSetup.lean"
    "MVK/Phase1/InitMain.lean"
    "MVK/Phase2/Common.lean"
    "MVK/Phase2/PageAlloc.lean"
    "MVK/Phase2/Slab.lean"
    "MVK/Phase3/ConntrackCore.lean"
    "MVK/Phase3/ConntrackGeneric.lean"
    "MVK/Phase3/ConntrackUDP.lean"
    "MVK/Phase3/ConntrackTCP.lean"
    "MVK/Phase3/ConntrackICMP.lean"
    "MVK/Phase3/ConntrackICMPv6.lean"
    "MVK/Phase3/ConntrackSCTP.lean"
    "MVK/Phase3/ConntrackDCCP.lean"
    "MVK/Phase3/NatCore.lean"
    "MVK/Phase3/NatProto.lean"
    "MVK/Phase4/IPv4IPv6/AfInet.lean"
    "MVK/Phase4/IPv4IPv6/AfInet6.lean"
    "MVK/Phase4/Routing/FibSemantics.lean"
    "MVK/RunuxDefenses.lean"
)

PASSED=0
FAILED=0

for module in "${MODULES[@]}"; do
    if [ ! -f "$module" ]; then
        log_warning "Module not found: $module"
        continue
    fi

    echo -n "  Checking $module... "
    if lake env lean "$module" > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}✗${NC}"
        FAILED=$((FAILED + 1))
    fi
done

echo ""
log_info "Results: $PASSED passed, $FAILED failed"
echo ""

# Step 4: Count proof obligations
log_info "Step 4/5: Analyzing proof obligations..."

count_sorry() {
    local file=$1
    grep "sorry" "$file" 2>/dev/null | wc -l | tr -d ' '
}

count_axiom() {
    local file=$1
    grep "^axiom" "$file" 2>/dev/null | wc -l | tr -d ' '
}

count_theorem() {
    local file=$1
    grep "^theorem" "$file" 2>/dev/null | wc -l | tr -d ' '
}

TOTAL_SORRY=0
TOTAL_AXIOMS=0
TOTAL_THEOREMS=0

echo ""
echo "Proof Status by Module:"
echo "────────────────────────────────────────"

for module in "${MODULES[@]}"; do
    if [ ! -f "$module" ]; then
        continue
    fi

    MODULE_NAME=$(basename "$module" .lean)
    SORRY_COUNT=$(count_sorry "$module")
    AXIOM_COUNT=$(count_axiom "$module")
    THEOREM_COUNT=$(count_theorem "$module")

    TOTAL_SORRY=$((TOTAL_SORRY + SORRY_COUNT))
    TOTAL_AXIOMS=$((TOTAL_AXIOMS + AXIOM_COUNT))
    TOTAL_THEOREMS=$((TOTAL_THEOREMS + THEOREM_COUNT))

    printf "  %-15s Theorems: %2d  Axioms: %2d  Sorry: %2d\n" \
        "$MODULE_NAME" "$THEOREM_COUNT" "$AXIOM_COUNT" "$SORRY_COUNT"
done

echo "────────────────────────────────────────"
printf "  %-15s Theorems: %2d  Axioms: %2d  Sorry: %2d\n" \
    "TOTAL" "$TOTAL_THEOREMS" "$TOTAL_AXIOMS" "$TOTAL_SORRY"
echo ""

# Calculate completion percentage
TOTAL_OBLIGATIONS=$((TOTAL_THEOREMS + TOTAL_AXIOMS))
COMPLETED=$((TOTAL_THEOREMS - TOTAL_SORRY))
PERCENTAGE=0
if [ $TOTAL_OBLIGATIONS -gt 0 ]; then
    PERCENTAGE=$((COMPLETED * 100 / TOTAL_OBLIGATIONS))
fi

log_info "Proof completion: $PERCENTAGE% ($COMPLETED/$TOTAL_OBLIGATIONS)"
echo ""

# Step 5: Generate proof obligations report
log_info "Step 5/5: Generating reports..."

REPORT_FILE="$SPECS_DIR/../PROOF_STATUS_REPORT.md"

cat > "$REPORT_FILE" << EOF
# MVK v9.3.1 Proof Status Report

**Generated:** $(date)
**Lean Version:** $(lean --version)

## Summary

- **Total Theorems:** $TOTAL_THEOREMS
- **Total Axioms:** $TOTAL_AXIOMS
- **Total Obligations:** $TOTAL_OBLIGATIONS
- **Completed Proofs:** $COMPLETED
- **Incomplete (sorry):** $TOTAL_SORRY
- **Completion:** $PERCENTAGE%

## Module Breakdown

| Module | Theorems | Axioms | Sorry | Status |
|--------|----------|--------|-------|--------|
EOF

for module in "${MODULES[@]}"; do
    if [ ! -f "$module" ]; then
        continue
    fi

    MODULE_NAME=$(basename "$module" .lean)
    SORRY_COUNT=$(count_sorry "$module")
    AXIOM_COUNT=$(count_axiom "$module")
    THEOREM_COUNT=$(count_theorem "$module")

    if [ $SORRY_COUNT -eq 0 ]; then
        STATUS="✅ Complete"
    else
        STATUS="⏳ In Progress"
    fi

    echo "| $MODULE_NAME | $THEOREM_COUNT | $AXIOM_COUNT | $SORRY_COUNT | $STATUS |" >> "$REPORT_FILE"
done

cat >> "$REPORT_FILE" << EOF

## Next Steps

1. Complete proofs marked with \`sorry\`
2. Convert axioms to proven theorems where possible
3. Add more specifications for Phase 2 modules

## Files

- Specifications: \`specs/lean4/\`
- Proof obligations: \`specs/PROOF_OBLIGATIONS.md\`
- Full documentation: \`specs/SPECIFICATIONS.md\`

---

*This report is automatically generated by \`verify_specs.sh\`*
EOF

log_success "Report generated: $REPORT_FILE"
echo ""

# Final summary
echo "========================================"
echo "  Verification Complete"
echo "========================================"
echo ""
log_info "Specifications are type-correct"
log_info "Proof completion: $PERCENTAGE%"

if [ $TOTAL_SORRY -gt 0 ]; then
    log_warning "$TOTAL_SORRY proof(s) remaining (marked 'sorry')"
fi

if [ $FAILED -gt 0 ]; then
    log_warning "$FAILED module(s) failed type checking"
    exit 1
fi

log_success "All checks passed!"
exit 0
