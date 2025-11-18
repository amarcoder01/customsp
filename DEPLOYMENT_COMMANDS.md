# 🚀 SpeedTestPro - Complete Deployment Commands

**Repository**: https://github.com/amarcoder01/customsp.git  
**Status**: ✅ Code Pushed Successfully  
**Date**: November 18, 2025

---

## 📦 **1. LOCAL DEVELOPMENT**

### Prerequisites Installation
```powershell
# Install Rust (if not installed)
# Visit: https://rustup.rs and run the installer

# Install Node.js 18+ (if not installed)
# Visit: https://nodejs.org

# Verify installations
rustc --version
cargo --version
node --version
npm --version
```

### Clone Repository
```powershell
git clone https://github.com/amarcoder01/customsp.git
cd customsp
```

### Backend Setup
```powershell
# Navigate to backend
cd backend

# Copy environment file
copy .env.example .env

# Edit .env and add your OpenAI API key (optional)
# OPENAI_API_KEY=sk-your-key-here

# Install dependencies and build
cargo build --release

# Run backend server
cargo run --release
```

Backend will run at: **http://localhost:8080**

### Frontend Setup (New Terminal)
```powershell
# Navigate to frontend
cd frontend

# Install dependencies
npm install

# Copy environment file
copy .env.example .env

# Start development server
npm run dev
```

Frontend will run at: **http://localhost:5173** or **http://localhost:5174**

### Test the Application
```powershell
# Check backend health
curl http://localhost:8080/api/health

# Or open in browser
start http://localhost:5173
```

---

## 🐳 **2. DOCKER DEPLOYMENT (Local)**

### Build and Run with Docker Compose
```powershell
# Build and start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Check status
docker-compose ps

# Stop services
docker-compose down

# Rebuild after changes
docker-compose up -d --build
```

Access at: **http://localhost**

### Individual Docker Commands
```powershell
# Build backend
docker build -t speedtestpro-backend ./backend

# Run backend
docker run -p 8080:8080 speedtestpro-backend

# Build frontend
docker build -t speedtestpro-frontend ./frontend

# Run frontend
docker run -p 80:80 speedtestpro-frontend
```

---

## ☁️ **3. RENDER.COM DEPLOYMENT**

### Method 1: Blueprint Deploy (Recommended - Easiest)
1. Go to: https://dashboard.render.com
2. Click **"New"** → **"Blueprint"**
3. Connect your GitHub repository: `amarcoder01/customsp`
4. Render will auto-detect `render.yaml` and deploy both services
5. Wait 5-10 minutes for deployment
6. Access your URLs provided by Render

### Method 2: Manual Deploy

#### Deploy Backend:
1. Go to https://dashboard.render.com
2. Click **"New"** → **"Web Service"**
3. Connect GitHub repository: `amarcoder01/customsp`
4. Configure:
   - **Name**: `speedtestpro-backend`
   - **Runtime**: `Rust`
   - **Build Command**: `cargo build --release`
   - **Start Command**: `./target/release/speedtest-pro-backend`
   - **Plan**: Free or Starter
5. Add Environment Variables:
   ```
   SERVER_ID=render-01
   SERVER_NAME=Render Cloud
   BIND_HOST=0.0.0.0
   BIND_PORT=8080
   RUST_LOG=info
   DATABASE_PATH=/opt/render/project/data/speedtest.db
   OPENAI_API_KEY=sk-your-key-here (optional)
   ```
6. Click **"Create Web Service"**

#### Deploy Frontend:
1. Click **"New"** → **"Static Site"**
2. Connect GitHub repository: `amarcoder01/customsp`
3. Configure:
   - **Name**: `speedtestpro-frontend`
   - **Build Command**: `cd frontend && npm install && npm run build`
   - **Publish Directory**: `frontend/dist`
4. Add Environment Variables:
   ```
   VITE_API_BASE_URL=https://speedtestpro-backend.onrender.com
   VITE_WS_BASE_URL=wss://speedtestpro-backend.onrender.com
   ```
5. Click **"Create Static Site"**

### Post-Deployment Verification
```powershell
# Test backend health
curl https://speedtestpro-backend.onrender.com/api/health

# Test in browser
start https://speedtestpro-frontend.onrender.com
```

---

## 🌐 **4. NETLIFY DEPLOYMENT (Frontend Only)**

```powershell
# Install Netlify CLI
npm install -g netlify-cli

# Login to Netlify
netlify login

# Build frontend
cd frontend
npm install
npm run build

# Deploy to Netlify
netlify deploy --prod --dir=dist

# Or use manual upload
# Go to https://app.netlify.com/drop
# Drag and drop the frontend/dist folder
```

---

## 🔧 **5. VERCEL DEPLOYMENT (Frontend Only)**

```powershell
# Install Vercel CLI
npm install -g vercel

# Login to Vercel
vercel login

# Deploy frontend
cd frontend
vercel --prod
```

Or use the web interface:
1. Go to https://vercel.com
2. Import your GitHub repo
3. Set root directory to `frontend`
4. Deploy

---

## 🖥️ **6. VPS/CLOUD SERVER DEPLOYMENT (Production)**

### For Ubuntu/Debian Server

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# Install Nginx
sudo apt install -y nginx

# Clone repository
git clone https://github.com/amarcoder01/customsp.git
cd customsp

# Build backend
cd backend
cargo build --release
cp .env.example .env
# Edit .env with your settings

# Create systemd service for backend
sudo nano /etc/systemd/system/speedtest-backend.service
```

**speedtest-backend.service** content:
```ini
[Unit]
Description=SpeedTest Pro Backend
After=network.target

[Service]
Type=simple
User=YOUR_USERNAME
WorkingDirectory=/home/YOUR_USERNAME/customsp/backend
ExecStart=/home/YOUR_USERNAME/customsp/backend/target/release/speedtest-pro-backend
Restart=always

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start backend service
sudo systemctl enable speedtest-backend
sudo systemctl start speedtest-backend
sudo systemctl status speedtest-backend

# Build frontend
cd ../frontend
npm install
npm run build

# Copy frontend to Nginx directory
sudo cp -r dist/* /var/www/html/

# Configure Nginx
sudo nano /etc/nginx/sites-available/speedtest
```

**Nginx configuration**:
```nginx
server {
    listen 80;
    server_name your-domain.com;
    
    root /var/www/html;
    index index.html;
    
    location / {
        try_files $uri $uri/ /index.html;
    }
    
    location /api {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
    
    location /ws {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "Upgrade";
        proxy_set_header Host $host;
    }
}
```

```bash
# Enable site and restart Nginx
sudo ln -s /etc/nginx/sites-available/speedtest /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx

# Setup SSL with Let's Encrypt (optional)
sudo apt install -y certbot python3-certbot-nginx
sudo certbot --nginx -d your-domain.com
```

---

## 🔍 **7. TESTING COMMANDS**

### Backend Tests
```powershell
cd backend

# Run all tests
cargo test

# Run specific test examples
cargo run --example test_integration
cargo run --example test_loaded_latency
cargo run --example test_aim_scoring
cargo run --example test_binary_protocol
cargo run --example test_ai_insights
```

### Frontend Tests
```powershell
cd frontend

# Run linter
npm run lint

# Build for production
npm run build

# Preview production build
npm run preview
```

### API Testing
```powershell
# Health check
curl http://localhost:8080/api/health

# Get servers
curl http://localhost:8080/api/servers

# Start test (returns test_id)
curl -X POST http://localhost:8080/api/test/start

# WebSocket connection (use browser console or wscat)
npm install -g wscat
wscat -c ws://localhost:8080/ws/test/YOUR_TEST_ID
```

---

## 📊 **8. MONITORING & LOGS**

### Local Development
```powershell
# Backend logs (if running as service)
cargo run --release

# Frontend logs
npm run dev

# Docker logs
docker-compose logs -f backend
docker-compose logs -f frontend
```

### Production (Render.com)
1. Go to Render Dashboard
2. Select your service
3. Click "Logs" tab
4. View real-time logs

### VPS Server
```bash
# Backend service logs
sudo journalctl -u speedtest-backend -f

# Nginx logs
sudo tail -f /var/log/nginx/access.log
sudo tail -f /var/log/nginx/error.log
```

---

## 🔄 **9. UPDATE/REDEPLOY COMMANDS**

### Local Update
```powershell
# Pull latest changes
git pull origin main

# Update backend
cd backend
cargo build --release
# Restart the server

# Update frontend
cd frontend
npm install
npm run build
```

### Render.com
- Automatic: Push to GitHub triggers auto-deploy
- Manual: Go to Dashboard → Select Service → "Manual Deploy" → "Deploy latest commit"

### VPS Server
```bash
# Pull latest changes
cd customsp
git pull origin main

# Rebuild backend
cd backend
cargo build --release
sudo systemctl restart speedtest-backend

# Rebuild frontend
cd ../frontend
npm install
npm run build
sudo cp -r dist/* /var/www/html/
```

---

## 🎯 **10. QUICK REFERENCE URLS**

After deployment, you'll have these endpoints:

### Local Development
- Frontend: http://localhost:5173
- Backend API: http://localhost:8080
- Health Check: http://localhost:8080/api/health

### Render.com (Example - your URLs will be different)
- Frontend: https://speedtestpro-frontend.onrender.com
- Backend: https://speedtestpro-backend.onrender.com
- API: https://speedtestpro-backend.onrender.com/api/health

### VPS Server
- Frontend: https://your-domain.com
- Backend: https://your-domain.com/api
- WebSocket: wss://your-domain.com/ws

---

## 🆘 **TROUBLESHOOTING**

### Backend won't compile
```powershell
# Clear cache and rebuild
cd backend
cargo clean
cargo build --release
```

### Frontend build fails
```powershell
cd frontend
# Clear node_modules and reinstall
rm -rf node_modules package-lock.json
npm install
npm run build
```

### Port already in use
```powershell
# Windows - Kill process on port 8080
netstat -ano | findstr :8080
taskkill /PID <PID> /F

# Or change port in backend/.env
BIND_PORT=8081
```

### Database errors
```powershell
cd backend
# Remove old database
rm data/speedtest.db*
# Restart server (will create new database)
cargo run
```

---

## ✅ **SUCCESS CHECKLIST**

- [ ] Code pushed to GitHub: ✅ https://github.com/amarcoder01/customsp.git
- [ ] Backend running locally
- [ ] Frontend running locally
- [ ] Speed test completes successfully
- [ ] Deployed to Render/Netlify/VPS
- [ ] Production URLs accessible
- [ ] WebSocket connections working
- [ ] AI insights enabled (if API key added)

---

## 🎉 **YOUR APP IS NOW DEPLOYED!**

**Repository**: https://github.com/amarcoder01/customsp.git

Choose your deployment method above and follow the commands step by step!

For support: Check the README.md and other documentation files in the repository.
