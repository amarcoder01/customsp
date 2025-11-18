// Type definitions for SpeedTestPro

export interface TestResult {
  id: string;
  server_id: string;
  timestamp: string;
  download_mbps: number;
  upload_mbps: number;
  latency_ms: number;
  jitter_ms: number;
  protocol: string;
  client_ip: string;
  test_duration_ms: number;
}

export interface LoadedLatencyResult {
  idle_avg_ms: number;
  idle_rpm: number;
  download_avg_ms: number;
  download_rpm: number;
  upload_avg_ms: number;
  upload_rpm: number;
  bufferbloat_grade: BufferbloatGrade;
  bufferbloat_download_ratio: number;
  bufferbloat_upload_ratio: number;
}

export type BufferbloatGrade = 'A+' | 'A' | 'B' | 'C' | 'D' | 'F';

export interface AIMScore {
  score: number;
  grade: string;
  assessment: string;
  capabilities?: string[];
  recommendations?: string[];
}

export interface AIMScores {
  gaming: AIMScore;
  streaming: AIMScore;
  video_conferencing: AIMScore;
  general_browsing: AIMScore;
  overall_score: number;
  overall_grade: string;
}

export interface AIRecommendation {
  priority: 'Critical' | 'High' | 'Medium' | 'Low';
  title: string;
  description: string;
  expected_improvement: string;
  difficulty: 'Easy' | 'Medium' | 'Hard';
}

export interface AIInsights {
  summary: string;
  detailed_analysis: string;
  recommendations: AIRecommendation[];
  predictions: string[];
  simple_explanation: string;
}

export interface EnhancedTestResult {
  basic: TestResult;
  loaded_latency: LoadedLatencyResult | null;
  aim_scores: AIMScores | null;
  ai_insights: AIInsights | null;
}

export interface TestProgress {
  type: 'progress' | 'complete' | 'error';
  stage?: 'Initializing' | 'IdleLatency' | 'Download' | 'Upload' | 'Finalizing' | 'Complete';
  progress_pct?: number;
  current_speed_mbps?: number;
  current_latency_ms?: number;
  message?: string;
  result?: EnhancedTestResult;
  error?: string;
}

export interface StartTestResponse {
  test_id: string;
  server_id: string;
  websocket_url: string;
}
