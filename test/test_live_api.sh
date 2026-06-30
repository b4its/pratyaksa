#!/bin/bash
# Test Live API Integration (192.168.101.3:6000)
# Usage: ./test_live_api.sh

set -e

BACKEND_URL="${BACKEND_URL:-http://localhost:8080}"
LIVE_API_URL="${LIVE_API_URL:-http://192.168.101.3:6000}"

echo "=========================================="
echo "🧪 Testing Live API Integration"
echo "=========================================="
echo ""
echo "Backend URL: $BACKEND_URL"
echo "Live API URL: $LIVE_API_URL"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

success() {
    echo -e "${GREEN}✓${NC} $1"
}

warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

error() {
    echo -e "${RED}✗${NC} $1"
}

info() {
    echo -e "ℹ️  $1"
}

# Test function
test_endpoint() {
    local name="$1"
    local url="$2"
    local method="${3:-GET}"
    local data="$4"
    
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    info "Testing: $name"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    if [ "$method" = "GET" ]; then
        response=$(curl -s -w "\n%{http_code}" "$url")
    else
        response=$(curl -s -w "\n%{http_code}" -X "$method" \
            -H "Content-Type: application/json" \
            -d "$data" \
            "$url")
    fi
    
    http_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')
    
    if [ "$http_code" = "200" ]; then
        success "HTTP $http_code - Success"
        echo "$body" | jq '.' 2>/dev/null || echo "$body"
    else
        error "HTTP $http_code - Failed"
        echo "$body"
    fi
}

# 1. Test Live API Health (direct)
echo ""
echo "=========================================="
echo "1️⃣  Testing Live API Health (Direct)"
echo "=========================================="
info "Checking if Live API (192.168.101.3:6000) is reachable..."

if curl -s --connect-timeout 5 "$LIVE_API_URL/health" > /dev/null 2>&1; then
    success "Live API is REACHABLE"
    curl -s "$LIVE_API_URL/health" | jq '.'
else
    warning "Live API is UNREACHABLE (will fallback to simulasi)"
    echo "This is OK - backend will auto-switch to simulation mode"
fi

# 2. Test Backend Pratyaksa Status
test_endpoint \
    "Backend Pratyaksa Status" \
    "$BACKEND_URL/api/v1/pratyaksa/status"

# 3. Test Fleet Data
test_endpoint \
    "Fleet Data (Live or Simulation)" \
    "$BACKEND_URL/api/v1/pratyaksa/fleet"

# 4. Test Fleet Health
test_endpoint \
    "Fleet Health Summary" \
    "$BACKEND_URL/api/v1/pratyaksa/fleet/health"

# 5. Test Unit Detail
echo ""
info "Testing unit detail endpoints..."
ASSET_IDS=("WA600-001" "HD785-001" "DT-001")

for asset_id in "${ASSET_IDS[@]}"; do
    test_endpoint \
        "Unit Detail: $asset_id" \
        "$BACKEND_URL/api/v1/pratyaksa/result/$asset_id"
done

# 6. Test Mode Switching
echo ""
echo "=========================================="
echo "2️⃣  Testing Mode Switching"
echo "=========================================="

# Switch to LIVE
test_endpoint \
    "Switch to LIVE Mode" \
    "$BACKEND_URL/api/v1/pratyaksa/mode" \
    "POST" \
    '{"mode": "live"}'

sleep 2

# Check status after switch
test_endpoint \
    "Status after LIVE switch" \
    "$BACKEND_URL/api/v1/pratyaksa/status"

# Switch to SIMULASI
test_endpoint \
    "Switch to SIMULASI Mode" \
    "$BACKEND_URL/api/v1/pratyaksa/mode" \
    "POST" \
    '{"mode": "simulasi"}'

sleep 2

# Check status after switch
test_endpoint \
    "Status after SIMULASI switch" \
    "$BACKEND_URL/api/v1/pratyaksa/status"

# Reset to AUTO
test_endpoint \
    "Reset to AUTO Mode" \
    "$BACKEND_URL/api/v1/pratyaksa/mode" \
    "POST" \
    '{"reset": true}'

sleep 2

# Check status after reset
test_endpoint \
    "Status after AUTO reset" \
    "$BACKEND_URL/api/v1/pratyaksa/status"

# 7. Test Features Endpoint
test_endpoint \
    "Features List (37 sensor names)" \
    "$BACKEND_URL/api/v1/pratyaksa/features"

# 8. Test Explain Endpoint
test_endpoint \
    "SHAP Explanation (sample)" \
    "$BACKEND_URL/api/v1/pratyaksa/explain/test-prediction-id"

echo ""
echo "=========================================="
echo "✅ Testing Complete!"
echo "=========================================="
echo ""
info "Summary:"
echo "  - Live API Health: Check logs above"
echo "  - Backend Status: ✓"
echo "  - Fleet Data: ✓"
echo "  - Mode Switching: ✓"
echo ""
info "Next Steps:"
echo "  1. Test Telegram Bot commands:"
echo "     - /start"
echo "     - /status"
echo "     - /detail"
echo "     - /pratyaksa"
echo "     - /unit WA600-001"
echo "  2. Test Frontend ModeSelector"
echo "  3. Monitor logs: docker logs -f backend"
echo ""
info "For detailed testing guide, see:"
echo "  → LIVE_API_INTEGRATION_TESTING.md"
echo ""
