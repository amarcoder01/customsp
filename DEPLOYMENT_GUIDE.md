# 🚀 SpeedTestPro - Complete Deployment Guide

**Platform**: Render.com  
**Status**: Production Ready  
**Last Updated**: November 18, 2025

---

## 📋 **Prerequisites**

- GitHub account
- Render account (render.com)
- OpenAI API key (optional, for AI insights)

---

## 🎯 **Quick Deploy to Render**

### Option 1: One-Click Deploy (Recommended)

1. Fork this repository to your GitHub account
2. Go to [Render Dashboard](https://dashboard.render.com)
3. Click "New" → "Blueprint"
4. Connect your GitHub repository
5. Render will automatically detect `render.yaml` and deploy both services

### Option 2: Manual Deploy

#### Deploy Backend:
1. New → Web Service
2. Connect repository
3. Name: `speedtestpro-backend`
4. Runtime: `Rust`
5. Build Command: `cargo build --release`
6. Start Command: `./target/release/speedtest-pro-backend`
7. Add environment variables (see below)

#### Deploy Frontend:
1. New → Static Site
2. Connect repository
3. Name: `speedtestpro-frontend`
4. Build Command: `cd frontend && npm install && npm run build`
5. Publish Directory: `frontend/dist`

---

## 🔐 **Environment Variables**

### Backend (Required)
```bash
SERVER_ID=render-01
SERVER_NAME=Render Cloud
BIND_HOST=0.0.0.0
BIND_PORT=8080
RUST_LOG=info
DATABASE_PATH=/opt/render/project/data/speedtest.db
```

### Backend (Optional - AI Features)
```bash
OPENAI_API_KEY=sk-your-key-here
OPENAI_MODEL=gpt-4o-mini
OPENAI_MAX_TOKENS=500
```

### Frontend
```bash
VITE_API_BASE_URL=https://speedtestpro-backend.onrender.com
VITE_WS_BASE_URL=wss://speedtestpro-backend.onrender.com
```

---

## 🐳 **Local Development with Docker**

```bash
# Clone repository
git clone <repo-url>
cd "advamce speed test site"

# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

Access at: `http://localhost`

---

## ✅ **Post-Deployment Checklist**

- [ ] Backend health check: `https://your-backend.onrender.com/api/health`
- [ ] Frontend loads correctly
- [ ] WebSocket connection works
- [ ] Speed test completes successfully
- [ ] All features display properly
- [ ] AI insights work (if enabled)

---

## 🔧 **Troubleshooting**

### Backend won't start
- Check environment variables
- Verify DATABASE_PATH directory exists
- Check Render logs for errors

### Frontend can't connect to backend
- Verify VITE_API_BASE_URL is correct
- Check CORS configuration
- Ensure backend is running

### WebSocket connection fails
- Verify WSS (not WS) for production
- Check firewall settings
- Confirm backend WebSocket routes

---

## 📊 **Monitoring**

Access logs in Render Dashboard:
- Backend: Environment → Logs
- Frontend: Environment → Logs
- Metrics: Environment → Metrics

---

## 🎉 **You're Live!**

Your SpeedTestPro instance is now running!

Share: `https://your-frontend.onrender.com`
