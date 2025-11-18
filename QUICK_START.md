# 🚀 SpeedTestPro Quick Start Guide

Get up and running in 5 minutes!

---

## ✅ **Prerequisites**

- Rust 1.70+ ([Install](https://rustup.rs/))
- Node.js 18+ (for frontend, coming soon)
- OpenAI API Key (optional, for AI insights)

---

## 📦 **Installation**

### 1. Clone Repository

```bash
git clone <repo-url>
cd "advamce speed test site"
```

### 2. Configure Backend

```bash
cd backend

# Copy environment file
cp .env.example .env

# Edit .env and add your OpenAI API key (optional)
# OPENAI_API_KEY=sk-...
```

### 3. Install Dependencies

```bash
# Rust dependencies install automatically
cargo build
```

---

## 🧪 **Test All Features**

Run the comprehensive integration test:

```bash
cargo run --example test_integration
```

This will test:
- ✅ Loaded latency testing
- ✅ Bufferbloat detection
- ✅ AIM use-case scoring
- ✅ Binary protocol
- ✅ AI insights (if configured)

**Individual Feature Tests**:

```bash
# Test loaded latency
cargo run --example test_loaded_latency

# Test AIM scoring
cargo run --example test_aim_scoring

# Test AI insights (requires OpenAI API key)
cargo run --example test_ai_insights

# Test binary protocol
cargo run --example test_binary_protocol
```

---

## 🚀 **Start the Server**

```bash
cargo run
```

Server starts on: `http://localhost:8080`

---

## 🌐 **Try the API**

### Health Check

```bash
curl http://localhost:8080/api/health
```

**Response**:
```json
{
  "status": "ok",
  "version": "1.0.0",
  "uptime_seconds": 10,
  "active_tests": 0
}
```

### Start Enhanced Test

```bash
curl -X POST http://localhost:8080/api/test/enhanced/start \
  -H "Content-Type: application/json" \
  -d '{
    "include_ai_insights": true,
    "use_binary_protocol": true,
    "duration_ms": 10000
  }'
```

**Response**:
```json
{
  "test_id": "550e8400-e29b-41d4-a716-446655440000",
  "server_id": "mumbai-01",
  "websocket_url": "ws://localhost:8080/ws/enhanced/550e8400-e29b-41d4-a716-446655440000"
}
```

### WebSocket Test (JavaScript)

```html
<!DOCTYPE html>
<html>
<head>
    <title>SpeedTest Pro</title>
    <script src="https://cdn.jsdelivr.net/npm/@msgpack/msgpack@2.8.0/dist.es5+umd/msgpack.min.js"></script>
</head>
<body>
    <h1>SpeedTest Pro</h1>
    <button onclick="startTest()">Start Test</button>
    <pre id="output"></pre>
    
    <script>
        async function startTest() {
            // Start test
            const response = await fetch('http://localhost:8080/api/test/enhanced/start', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ include_ai_insights: true })
            });
            
            const { test_id, websocket_url } = await response.json();
            console.log('Test started:', test_id);
            
            // Connect WebSocket
            const ws = new WebSocket(websocket_url);
            ws.binaryType = 'arraybuffer';
            
            ws.onmessage = (event) => {
                if (event.data instanceof ArrayBuffer) {
                    // Binary MessagePack
                    const message = MessagePack.decode(new Uint8Array(event.data));
                    document.getElementById('output').textContent = JSON.stringify(message, null, 2);
                } else {
                    // JSON fallback
                    const message = JSON.parse(event.data);
                    document.getElementById('output').textContent = JSON.stringify(message, null, 2);
                }
            };
            
            ws.onerror = (error) => {
                console.error('WebSocket error:', error);
            };
        }
    </script>
</body>
</html>
```

---

## 📚 **Next Steps**

### Explore the Documentation

- [API Documentation](./API_DOCUMENTATION.md) - Complete API reference
- [Loaded Latency Guide](./docs/LOADED_LATENCY.md) - How bufferbloat detection works
- [AIM Scoring Guide](./docs/AIM_SCORING.md) - Use-case scoring explained
- [AI Insights Guide](./docs/AI_INSIGHTS.md) - GPT-4 integration
- [Binary Protocol Guide](./docs/BINARY_PROTOCOL.md) - MessagePack efficiency

### Run Individual Tests

```bash
# Example programs in backend/examples/
cargo run --example test_loaded_latency
cargo run --example test_aim_scoring
cargo run --example test_ai_insights
cargo run --example test_binary_protocol
cargo run --example test_integration
```

### Enable AI Insights

1. Get OpenAI API key from [platform.openai.com](https://platform.openai.com)
2. Add to `.env` file:
   ```
   OPENAI_API_KEY=sk-...
   OPENAI_MODEL=gpt-4o-mini
   ```
3. Test:
   ```bash
   cargo run --example test_ai_insights
   ```

---

## 🔧 **Troubleshooting**

### Port Already in Use

```bash
# Change port in .env
BIND_PORT=8081
```

### OpenAI API Errors

```bash
# Check API key
echo $OPENAI_API_KEY

# Test with curl
curl https://api.openai.com/v1/models \
  -H "Authorization: Bearer $OPENAI_API_KEY"
```

### Dependencies Not Building

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build
```

---

## 🎯 **What to Test**

### 1. Basic Speed Test
```bash
curl -X POST http://localhost:8080/api/test/start
```

### 2. Enhanced Test (All Features)
```bash
curl -X POST http://localhost:8080/api/test/enhanced/start \
  -H "Content-Type: application/json" \
  -d '{"include_ai_insights": true}'
```

### 3. Get Servers
```bash
curl http://localhost:8080/api/servers
```

### 4. Test History
```bash
curl http://localhost:8080/api/test/history
```

---

## 🚀 **Development Workflow**

### 1. Make Changes

Edit files in `backend/src/`

### 2. Run Tests

```bash
cargo test
```

### 3. Run Examples

```bash
cargo run --example test_integration
```

### 4. Start Server

```bash
cargo run
```

### 5. Test with cURL

```bash
curl http://localhost:8080/api/health
```

---

## 📊 **Performance Tips**

### Production Build

```bash
cargo build --release
./target/release/speedtest-pro-backend
```

### Resource Limits

In `.env`:
```bash
MAX_CONCURRENT_TESTS=50
MAX_MEMORY_MB=512
CPU_LIMIT_PERCENT=80
```

### AI Insights Cost

- Per analysis: ~$0.0003 (less than a penny!)
- 1,000 tests: ~$9/month
- Only enable when requested by users

---

## 🎨 **Frontend (Coming Soon)**

The React frontend is in development. For now, use:
- API endpoints directly
- WebSocket test HTML (see above)
- Postman/Insomnia collections

---

## 💡 **Tips**

1. **Start simple**: Test basic API first
2. **Enable features gradually**: Add AI insights last
3. **Monitor logs**: `RUST_LOG=debug cargo run`
4. **Use binary protocol**: 30-50% more efficient
5. **Cache AI insights**: They're expensive

---

## 📞 **Getting Help**

- Check [API Documentation](./API_DOCUMENTATION.md)
- Read feature guides in `docs/`
- Run example programs
- Check server logs

---

## ✅ **Checklist**

- [ ] Rust installed
- [ ] Repository cloned
- [ ] `.env` configured
- [ ] Dependencies built
- [ ] Tests passing
- [ ] Server running
- [ ] API responding
- [ ] OpenAI configured (optional)
- [ ] Features tested

---

**You're ready to go!** 🎉

Start the server with `cargo run` and test with `curl http://localhost:8080/api/health`
