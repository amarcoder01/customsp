# SpeedTestPro API Documentation

**Version**: 2.0  
**Base URL**: `http://localhost:8080`  
**WebSocket URL**: `ws://localhost:8080`

---

## 🎯 Overview

SpeedTestPro provides two API tiers:

1. **Basic API** - Standard speed test (compatible with legacy clients)
2. **Enhanced API** - Full feature set with loaded latency, AIM scores, and AI insights

---

## 📡 Basic API Endpoints

### 1. Health Check

**GET** `/api/health`

Check server status.

**Response**:
```json
{
  "status": "ok",
  "version": "1.0.0",
  "uptime_seconds": 3600,
  "active_tests": 5
}
```

---

### 2. Get Servers

**GET** `/api/servers`

Get list of available test servers.

**Response**:
```json
{
  "servers": [
    {
      "id": "mumbai-01",
      "name": "Mumbai, India",
      "location": {
        "lat": 19.0760,
        "lon": 72.8777
      },
      "available": true,
      "load": 0.45
    }
  ]
}
```

---

### 3. Start Basic Test

**POST** `/api/test/start`

Start a basic speed test.

**Request Body**:
```json
{
  "duration_ms": 10000,
  "protocol": "TCP"
}
```

**Response**:
```json
{
  "test_id": "550e8400-e29b-41d4-a716-446655440000",
  "server_id": "mumbai-01",
  "websocket_url": "ws://localhost:8080/ws/test/550e8400-e29b-41d4-a716-446655440000"
}
```

---

### 4. Get Test Result

**GET** `/api/test/{test_id}`

Retrieve test results.

**Response**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "server_id": "mumbai-01",
  "timestamp": "2025-11-18T12:00:00Z",
  "download_mbps": 300.5,
  "upload_mbps": 50.3,
  "latency_ms": 15.2,
  "jitter_ms": 5.1,
  "protocol": "TCP",
  "client_ip": "1.2.3.4",
  "test_duration_ms": 10000
}
```

---

### 5. Get Test History

**GET** `/api/test/history`

Get recent test history (last 20 tests).

**Response**:
```json
{
  "tests": [
    { /* TestResult object */ }
  ]
}
```

---

## 🚀 Enhanced API Endpoints

### 1. Start Enhanced Test

**POST** `/api/test/enhanced/start`

Start enhanced test with all features.

**Request Body**:
```json
{
  "include_ai_insights": true,
  "use_binary_protocol": true,
  "duration_ms": 10000
}
```

**Response**:
```json
{
  "test_id": "enhanced-550e8400-e29b-41d4-a716-446655440000",
  "server_id": "mumbai-01",
  "websocket_url": "ws://localhost:8080/ws/enhanced/enhanced-550e8400-e29b-41d4-a716-446655440000"
}
```

---

### 2. Get Enhanced Result

**GET** `/api/test/enhanced/{test_id}?include_ai=true`

Get full enhanced test result with all features.

**Response**:
```json
{
  "id": "enhanced-550e8400-e29b-41d4-a716-446655440000",
  "server_id": "mumbai-01",
  "timestamp": "2025-11-18T12:00:00Z",
  "download_mbps": 300.5,
  "upload_mbps": 50.3,
  "latency_ms": 15.2,
  "jitter_ms": 5.1,
  "protocol": "TCP",
  "client_ip": "1.2.3.4",
  "test_duration_ms": 10000,
  
  "loaded_latency": {
    "idle_avg_ms": 15.2,
    "idle_rpm": 3947,
    "download_avg_ms": 95.0,
    "download_rpm": 632,
    "upload_avg_ms": 180.0,
    "upload_rpm": 333,
    "bufferbloat_grade": "C",
    "bufferbloat_download_ratio": 5.33,
    "bufferbloat_upload_ratio": 11.0
  },
  
  "aim_scores": {
    "gaming": {
      "score": 62,
      "grade": "Fair",
      "assessment": "Playable but not ideal - casual gaming okay",
      "capabilities": ["Playable for casual games"],
      "recommendations": ["Reduce bufferbloat to improve latency"]
    },
    "streaming": {
      "score": 88,
      "grade": "Good",
      "assessment": "Great for 4K streaming and HD on multiple devices"
    },
    "video_conferencing": {
      "score": 58,
      "grade": "Fair",
      "assessment": "Video calls work, occasional quality drops"
    },
    "general_browsing": {
      "score": 85,
      "grade": "Good",
      "assessment": "Great browsing - fast page loads"
    },
    "overall_score": 73,
    "overall_grade": "Fair"
  },
  
  "ai_insights": {
    "summary": "Your connection has excellent download speeds (300 Mbps) but suffers from significant bufferbloat...",
    "detailed_analysis": "The 1100% latency increase during uploads indicates severe bufferbloat...",
    "recommendations": [
      {
        "priority": "Critical",
        "title": "Enable Smart Queue Management (SQM)",
        "description": "Configure QoS in your router settings...",
        "expected_improvement": "Reduce upload latency from 180ms to <50ms",
        "difficulty": "Medium"
      }
    ],
    "predictions": [
      "Video calls will freeze when anyone uploads large files",
      "Gaming will experience lag spikes during high upload activity"
    ],
    "simple_explanation": "Think of your internet like a highway..."
  }
}
```

---

## 🌐 WebSocket API

### Basic WebSocket

**WS** `/ws/test/{test_id}`

Real-time test progress (JSON protocol).

**Messages Received** (Server → Client):
```json
{
  "type": "progress",
  "stage": "download",
  "progress": 0.75,
  "current_speed_mbps": 450.5,
  "current_latency_ms": 15.2,
  "message": "Testing download speed..."
}
```

```json
{
  "type": "complete",
  "result": { /* TestResult object */ }
}
```

---

### Enhanced WebSocket (Binary Protocol)

**WS** `/ws/enhanced/{test_id}`

Real-time test with binary MessagePack protocol (30-50% more efficient).

**Messages Received** (Server → Client):

Binary MessagePack format. Use `@msgpack/msgpack` to decode:

```typescript
import * as msgpack from '@msgpack/msgpack';

ws.binaryType = 'arraybuffer';
ws.onmessage = (event) => {
  const message = msgpack.decode(new Uint8Array(event.data));
  
  switch (message.type) {
    case 'Progress':
      // Handle progress update
      break;
    case 'Results':
      // Handle final results
      break;
  }
};
```

---

## 💡 Usage Examples

### JavaScript/TypeScript

```typescript
// Start enhanced test
const response = await fetch('http://localhost:8080/api/test/enhanced/start', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    include_ai_insights: true,
    use_binary_protocol: true
  })
});

const { test_id, websocket_url } = await response.json();

// Connect to WebSocket
const ws = new WebSocket(websocket_url);
ws.binaryType = 'arraybuffer';

ws.onmessage = (event) => {
  if (event.data instanceof ArrayBuffer) {
    // Binary protocol
    const message = msgpack.decode(new Uint8Array(event.data));
    console.log('Binary message:', message);
  } else {
    // JSON fallback
    const message = JSON.parse(event.data);
    console.log('JSON message:', message);
  }
};

// Get final result
const result = await fetch(`http://localhost:8080/api/test/enhanced/${test_id}?include_ai=true`);
const enhancedResult = await result.json();
```

---

### Python

```python
import requests
import websocket
import msgpack

# Start test
response = requests.post('http://localhost:8080/api/test/enhanced/start', json={
    'include_ai_insights': True,
    'use_binary_protocol': True
})

data = response.json()
test_id = data['test_id']
ws_url = data['websocket_url']

# WebSocket connection
def on_message(ws, message):
    # Decode MessagePack
    msg = msgpack.unpackb(message)
    print(f"Message: {msg}")

ws = websocket.WebSocketApp(ws_url, on_message=on_message)
ws.run_forever()
```

---

### cURL

```bash
# Start test
curl -X POST http://localhost:8080/api/test/enhanced/start \
  -H "Content-Type: application/json" \
  -d '{"include_ai_insights": true}'

# Get result
curl http://localhost:8080/api/test/enhanced/550e8400?include_ai=true
```

---

## 📊 Rate Limits

- Basic tests: 10 per minute per IP
- Enhanced tests: 5 per minute per IP (AI insights are expensive)
- WebSocket connections: 5 concurrent per IP

---

## 🔒 Authentication (Future)

Currently public. Future versions will support:
- API keys for higher rate limits
- OAuth for user accounts
- Premium features (more AI insights, longer history)

---

## 🐛 Error Responses

All errors return JSON:

```json
{
  "error": "Error message here",
  "code": "ERROR_CODE",
  "details": "Additional details if available"
}
```

**Common Error Codes**:
- `INVALID_REQUEST` - Malformed request
- `SERVER_OVERLOADED` - Too many concurrent tests
- `TEST_NOT_FOUND` - Invalid test ID
- `AI_UNAVAILABLE` - AI insights temporarily unavailable

---

## 📚 SDKs & Libraries

### Official (Coming Soon)
- JavaScript/TypeScript SDK
- Python SDK
- React component library

### Community
- Contributions welcome!

---

## 🎯 Best Practices

1. **Use Enhanced API** for full feature set
2. **Enable binary protocol** for mobile apps
3. **Request AI insights sparingly** (they're expensive)
4. **Cache results** when possible
5. **Handle WebSocket reconnection** gracefully

---

## 📞 Support

- Documentation: `/docs`
- GitHub Issues: [github.com/speedtestpro/issues]
- Email: support@speedtestpro.com

---

**Last Updated**: November 18, 2025  
**API Version**: 2.0
