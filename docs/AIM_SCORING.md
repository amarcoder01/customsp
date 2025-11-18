# AIM (Aggregated Internet Measurement) Scoring System

## 🎯 Overview

The **AIM Scoring System** translates complex network metrics into simple, actionable quality scores that answer the question: **"Is my internet good enough for [specific activity]?"**

Instead of showing users raw numbers like "450 Mbps download, 18ms latency" and expecting them to interpret it, we provide context-specific assessments:

- 🎮 **Gaming**: "Perfect for competitive gaming - esports ready!" (95/100)
- 📺 **Streaming**: "4K streaming on multiple devices" (98/100)
- 💼 **Video Conferencing**: "HD video calls work smoothly" (82/100)
- 🌐 **General Browsing**: "Lightning-fast page loads" (96/100)

---

## 💡 Why AIM Matters

### The Problem
Users don't understand what "450 Mbps" means for their actual activities:
- "Can I play Fortnite without lag?"
- "Will my Zoom calls work during family Netflix time?"
- "Is this fast enough for 4K streaming?"

### Our Solution
**Use-case specific scoring** that considers what matters most for each activity:

| Activity | Most Important Metrics |
|----------|----------------------|
| Gaming | Loaded latency (50%), Jitter (25%), Packet loss (15%) |
| Streaming | Download speed (40%), Loaded latency (30%), Jitter (20%) |
| Video Calls | Upload speed (30%), Upload loaded latency (30%), Jitter (25%) |
| Browsing | Download speed (40%), Idle latency (40%), Stability (20%) |

---

## 🏗️ Architecture

### Scoring Components

```rust
pub struct AIMScores {
    pub gaming: UseCaseScore,              // For online gaming
    pub streaming: UseCaseScore,           // For video streaming
    pub video_conferencing: UseCaseScore,  // For Zoom/Teams/etc
    pub general_browsing: UseCaseScore,    // For web browsing
    pub overall_score: f64,                // Weighted average
    pub overall_grade: QualityGrade,       // Overall assessment
}

pub struct UseCaseScore {
    pub score: f64,                // 0-100 numerical score
    pub grade: QualityGrade,       // Excellent/Good/Fair/Poor/VeryPoor
    pub assessment: String,        // Human-readable verdict
    pub explanation: String,       // Why this score?
    pub emoji: String,             // Visual indicator
    pub capabilities: Vec<String>, // What works well
    pub recommendations: Vec<String>, // How to improve
}
```

### Quality Grades

| Grade | Score Range | Meaning |
|-------|-------------|---------|
| **Excellent** | 90-100 | Best possible - no issues |
| **Good** | 75-89 | Works well - minor issues |
| **Fair** | 60-74 | Acceptable - some problems |
| **Poor** | 40-59 | Problematic - many issues |
| **Very Poor** | 0-39 | Not suitable for this use |

---

## 🎮 Gaming Score (Latency-Focused)

### What We Measure
Gaming is **latency-sensitive**. A 1000 Mbps connection with 200ms latency performs worse than a 50 Mbps connection with 20ms latency for gaming.

### Scoring Weights
```
Loaded Latency: 50 points (CRITICAL)
Jitter:         25 points (consistency)
Packet Loss:    15 points (rubber-banding)
Download Speed: 10 points (least important)
```

### Scoring Thresholds

**Loaded Latency (50 points)**
```
< 20ms  → 50 points ⭐ Perfect for esports
< 50ms  → 45 points ✅ Excellent for gaming
< 80ms  → 35 points ✓ Good for most games
< 100ms → 25 points ⚠️ Playable, not ideal
< 150ms → 15 points 🔴 Noticeable lag
> 150ms → 5 points  ❌ Frustrating experience
```

**Jitter (25 points)**
```
< 5ms  → 25 points (consistent - no spikes)
< 15ms → 20 points (minor variation)
< 30ms → 15 points (occasional spikes)
> 30ms → 5 points (frequent lag spikes)
```

### Example Assessments

```
Score 95: "Perfect for competitive gaming - esports ready!"
  ✅ Latency 12ms - instant response
  ✅ Jitter 3ms - no lag spikes
  ✅ Perfect for competitive play

Score 65: "Playable but not ideal - casual gaming okay"
  ⚠️ Latency 85ms - noticeable in fast-paced games
  💡 Reduce bufferbloat to improve latency

Score 35: "Poor gaming experience - lag will be noticeable"
  🔴 Very high latency 180ms - gaming will be frustrating
  💡 Enable SQM/QoS on router
  💡 Contact ISP about high latency
```

---

## 📺 Streaming Score (Bandwidth-Focused)

### What We Measure
Streaming needs **sufficient download speed** and **stable connection** to prevent buffering.

### Scoring Weights
```
Download Speed:        40 points (CRITICAL)
Download Loaded Latency: 30 points (buffering prevention)
Jitter:               20 points (stream stability)
Packet Loss:          10 points (frame drops)
```

### Bandwidth Requirements

```
100+ Mbps → 8K streaming on multiple devices
50+ Mbps  → 4K streaming on 2-3 devices
25+ Mbps  → 4K on 1 device, HD on multiple
15+ Mbps  → Reliable HD (1080p) streaming
10+ Mbps  → HD on 1 device
5+ Mbps   → SD/HD works, avoid 4K
< 5 Mbps  → Too low for HD streaming
```

### Example Assessments

```
Score 98: "Perfect for 4K/8K streaming on multiple devices"
  ✅ Speed 450 Mbps - 8K streaming with headroom
  ✅ Latency 18ms - instant start, no buffering
  ✅ 4K 60fps streaming on 3+ devices

Score 70: "HD streaming works, 4K may buffer occasionally"
  ✓ Speed 15 Mbps - HD streaming reliable
  ⚠️ 4K may buffer occasionally
  💡 Consider faster plan for 4K

Score 40: "Streaming will be problematic"
  🔴 Speed 4.5 Mbps too low for HD
  💡 Upgrade plan for better streaming
```

---

## 💼 Video Conferencing Score (Upload-Focused)

### What We Measure
Video calls are **upload-sensitive** and need **low upload latency**. This is often the worst metric on consumer connections.

### Scoring Weights
```
Upload Speed:           30 points (CRITICAL)
Upload Loaded Latency:  30 points (frozen video!)
Jitter:                25 points (choppy audio/video)
Download Speed:        15 points (receiving video)
```

### Upload Requirements

```
20+ Mbps → 4K video with screen sharing
10+ Mbps → HD video with screen sharing
5+ Mbps  → HD video calls work well
3+ Mbps  → HD (may struggle with screen share)
1.5+ Mbps → SD video works
< 1.5 Mbps → Audio-only recommended
```

### Upload Latency Impact

```
< 30ms  → Smooth real-time conversation ⭐
< 80ms  → Good - minor delay ✅
< 150ms → Awkward pauses ⚠️
< 250ms → Video will freeze frequently 🔴
> 250ms → Unusable for video ❌
```

### Why Upload Matters More

**Common Issue**: Connection has 400 Mbps down / 20 Mbps up
- Download test: ⭐ Excellent
- Upload loaded latency: 🔴 180ms (bufferbloat!)
- **Result**: Video calls freeze when uploading

**Our Score**: Video Conferencing 65/100 (Fair)
- ⚠️ High upload latency 180ms
- 💡 Enable SQM to reduce bufferbloat

### Example Assessments

```
Score 92: "Perfect for 4K video calls and screen sharing"
  ✅ Upload 45 Mbps - 4K video with sharing
  ✅ Upload latency 22ms - smooth conversation
  ✅ Jitter 4ms - stable quality

Score 68: "Video calls work, occasional quality drops"
  ✓ Upload 5 Mbps - HD video works
  ⚠️ Latency 95ms - may cause awkward pauses
  💡 Enable SQM to reduce bufferbloat

Score 35: "Not suitable for video conferencing"
  🔴 Upload latency 280ms - video will freeze
  🔴 Upload 1.2 Mbps too low for video
  💡 Use audio-only or upgrade plan
```

---

## 🌐 General Browsing Score (Balanced)

### What We Measure
Browsing needs a **balance** of speed and responsiveness for snappy page loads.

### Scoring Weights
```
Download Speed: 40 points (page loads)
Idle Latency:   40 points (responsiveness)
Jitter:        10 points (stability)
Packet Loss:   10 points (reliability)
```

### Scoring Thresholds

**Download Speed (40 points)**
```
100+ Mbps → Lightning-fast, instant downloads
50+ Mbps  → Very fast browsing
25+ Mbps  → Fast page loads
10+ Mbps  → Good browsing
5+ Mbps   → Adequate for basic browsing
< 5 Mbps  → Slow
```

**Idle Latency (40 points)**
```
< 20ms  → Instant page response
< 50ms  → Fast response
< 100ms → Acceptable
< 200ms → Pages feel sluggish
> 200ms → Slow response times
```

### Example Assessments

```
Score 96: "Outstanding browsing - instant and smooth"
  ✅ Speed 450 Mbps - lightning-fast downloads
  ✅ Latency 12ms - instant page response
  ✅ Pages load instantaneously

Score 72: "Adequate browsing - some delays"
  ✓ Speed 18 Mbps - acceptable
  ⚠️ Latency 75ms - minor delays
  💡 Pages may feel slightly sluggish
```

---

## 🚀 Usage Examples

### Basic Usage

```rust
use speedtest_pro_backend::services::aim_scoring::AIMCalculator;

// After running speed test and loaded latency test
let test_result = /* ... */;
let loaded_latency = /* ... */;

// Calculate all scores
let aim_scores = AIMCalculator::calculate_all_scores(
    &test_result,
    &loaded_latency
);

// Display summary
println!("{}", aim_scores.summary());

// Get detailed report for gaming
println!("{}", aim_scores.detailed_report("gaming"));
```

### API Response Example

```json
{
  "gaming": {
    "score": 95,
    "grade": "Excellent",
    "emoji": "🎮⭐",
    "assessment": "Perfect for competitive gaming - esports ready!",
    "explanation": "Gaming requires low latency (12ms) and stable connection. Your connection is excellent for gaming.",
    "capabilities": [
      "Perfect for competitive gaming (esports-level)",
      "Consistent performance - no lag spikes"
    ],
    "recommendations": []
  },
  "streaming": {
    "score": 98,
    "grade": "Excellent",
    "emoji": "📺",
    "assessment": "Perfect for 4K/8K streaming on multiple devices",
    "explanation": "Streaming quality depends on download speed (450.0 Mbps) and stability. Your speed is excellent for streaming.",
    "capabilities": [
      "8K streaming on multiple devices",
      "4K 60fps streaming with headroom"
    ],
    "recommendations": []
  },
  "video_conferencing": {
    "score": 88,
    "grade": "Good",
    "emoji": "💼",
    "assessment": "HD video conferencing works smoothly",
    "explanation": "Video calls need good upload (45.0 Mbps) and low upload latency (22ms). Your connection is great for video calls.",
    "capabilities": [
      "HD video calls with screen sharing",
      "Smooth real-time conversation"
    ],
    "recommendations": []
  },
  "general_browsing": {
    "score": 96,
    "grade": "Excellent",
    "emoji": "🌐",
    "assessment": "Outstanding browsing experience - instant and smooth",
    "explanation": "Browsing quality combines speed (450.0 Mbps) and responsiveness (12ms latency).",
    "capabilities": [
      "Lightning-fast page loads",
      "Instant large downloads",
      "Instant page response"
    ],
    "recommendations": []
  },
  "overall_score": 94.25,
  "overall_grade": "Excellent"
}
```

---

## 🎨 Frontend Display Recommendations

### Summary Card
```
╔════════════════════════════════════╗
║   YOUR CONNECTION QUALITY          ║
╠════════════════════════════════════╣
║  Overall: 94/100 ⭐ Excellent      ║
╠════════════════════════════════════╣
║  🎮 Gaming:      95/100 ⭐         ║
║  📺 Streaming:   98/100 ⭐         ║
║  💼 Video Calls: 88/100 ✅         ║
║  🌐 Browsing:    96/100 ⭐         ║
╚════════════════════════════════════╝
```

### Detailed Card for Gaming
```
╔════════════════════════════════════════════╗
║  🎮 GAMING QUALITY                         ║
╠════════════════════════════════════════════╣
║  Score: 95/100 ⭐ Excellent                ║
║                                            ║
║  Perfect for competitive gaming!           ║
║  Your connection is esports-ready.         ║
║                                            ║
║  ✅ What Works:                            ║
║  • Perfect for competitive play            ║
║  • No lag spikes detected                  ║
║  • Instant response times                  ║
║                                            ║
║  📊 Key Metrics:                           ║
║  Latency: 12ms ⭐                          ║
║  Jitter:  3ms ⭐                           ║
║  Speed:   450 Mbps ✅                      ║
╚════════════════════════════════════════════╝
```

### Visual Progress Bars
```html
Gaming:      [████████████████████] 95%
Streaming:   [█████████████████████] 98%
Video Calls: [█████████████████░░░] 88%
Browsing:    [████████████████████░] 96%
```

---

## 🏆 Competitive Advantages

| Feature | SpeedTestPro | Ookla | Fast.com | Cloudflare |
|---------|--------------|-------|----------|------------|
| **Use-Case Scoring** | ✅ 4 scores | ❌ | ❌ | ✅ 3 scores |
| **Gaming-Specific** | ✅ Latency-focused | ❌ | ❌ | ❌ |
| **Upload Quality** | ✅ Video call score | ❌ | ❌ | ❌ |
| **Recommendations** | ✅ Specific fixes | ❌ | ❌ | ❌ |
| **Capabilities List** | ✅ What works | ❌ | ❌ | ❌ |

### Unique Features
1. **Gaming Score**: Only test that weights latency over speed for gaming
2. **Upload Latency**: Reveals why video calls freeze even with "fast" internet
3. **Actionable Recommendations**: Tells users HOW to improve
4. **Capabilities**: Shows what activities work well

---

## 📊 Research & Validation

### Based on Industry Standards

**Gaming Latency Requirements**
- Competitive gaming: <20ms (Source: Nvidia, Riot Games)
- Casual gaming: <50ms acceptable
- >100ms: Noticeable lag

**Streaming Bandwidth Requirements**
- Netflix 4K: 25 Mbps
- YouTube 4K 60fps: 50 Mbps
- Multiple 4K streams: 100+ Mbps

**Video Conferencing Requirements**
- Zoom HD: 3.8 Mbps up/down
- Teams 1080p: 4 Mbps up/down
- Screen sharing adds: +2-3 Mbps

### Cloudflare AIM Inspiration
Our implementation is inspired by Cloudflare's AIM system but adds:
- Gaming-specific scoring (they don't have this)
- Separate video conferencing score
- Detailed recommendations
- Capabilities listing

---

## 🔮 Future Enhancements

### Phase 2
- [ ] Packet loss integration (WebRTC-based)
- [ ] Historical trending (score changes over time)
- [ ] ISP comparison (vs others in area)

### Phase 3
- [ ] Time-of-day analysis (when is quality best/worst?)
- [ ] Device-specific scores (WiFi vs Ethernet)
- [ ] Activity detection (auto-detect what user is doing)

### Phase 4
- [ ] AI predictions (will quality degrade soon?)
- [ ] Automatic SQM configuration recommendations
- [ ] Custom scoring weights (user preferences)

---

## ✅ Implementation Checklist

- [x] Core AIM calculator
- [x] Gaming score algorithm
- [x] Streaming score algorithm
- [x] Video conferencing score algorithm
- [x] General browsing score algorithm
- [x] Quality grade system
- [x] Recommendations engine
- [x] Capabilities detection
- [x] Example test program
- [x] Comprehensive documentation
- [ ] WebSocket integration
- [ ] REST API endpoints
- [ ] Frontend visualization
- [ ] Database schema updates

---

**Status**: ✅ IMPLEMENTED - Ready for integration  
**Priority**: 🔥 HIGH - Key differentiator  
**Impact**: 🎯 HIGH - Makes metrics actionable
