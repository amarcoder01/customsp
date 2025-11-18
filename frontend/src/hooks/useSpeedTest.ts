import { useState, useCallback, useRef } from 'react';
import * as msgpack from '@msgpack/msgpack';
import type { EnhancedTestResult, TestProgress, StartTestResponse } from '../types';

const API_BASE = 'http://localhost:8080/api';

export function useSpeedTest() {
  const [isRunning, setIsRunning] = useState(false);
  const [progress, setProgress] = useState<TestProgress | null>(null);
  const [result, setResult] = useState<EnhancedTestResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const wsRef = useRef<WebSocket | null>(null);

  const startTest = useCallback(async (includeAI: boolean = false) => {
    setIsRunning(true);
    setProgress(null);
    setResult(null);
    setError(null);

    try {
      // Start enhanced test
      const response = await fetch(`${API_BASE}/test/enhanced/start`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          include_ai_insights: includeAI,
          use_binary_protocol: true,
          duration_ms: 10000,
        }),
      });

      if (!response.ok) {
        throw new Error('Failed to start test');
      }

      const data: StartTestResponse = await response.json();
      
      // Connect to WebSocket
      const ws = new WebSocket(data.websocket_url);
      ws.binaryType = 'arraybuffer';
      wsRef.current = ws;

      ws.onopen = () => {
        console.log('WebSocket connected');
        setProgress({
          type: 'progress',
          stage: 'Initializing',
          progress_pct: 0,
          message: 'Connecting...',
        });
      };

      ws.onmessage = (event) => {
        try {
          let message: any;

          if (event.data instanceof ArrayBuffer) {
            // Binary MessagePack protocol
            message = msgpack.decode(new Uint8Array(event.data));
          } else {
            // JSON fallback
            message = JSON.parse(event.data);
          }

          if (message.type === 'progress' || message.Progress) {
            // Handle progress update
            const progressData = message.Progress || message;
            setProgress({
              type: 'progress',
              stage: progressData.stage,
              progress_pct: progressData.progress_pct,
              current_speed_mbps: progressData.current_speed_mbps,
              current_latency_ms: progressData.current_latency_ms,
              message: getStageMessage(progressData.stage, progressData.progress_pct),
            });
          } else if (message.type === 'complete' || message.Results || message.basic) {
            // Handle final results
            const enhancedResult = message.Results?.results || message;
            setResult(enhancedResult);
            setProgress({
              type: 'complete',
              progress_pct: 100,
              message: 'Test complete!',
            });
            setIsRunning(false);
            ws.close();
          } else if (message.error) {
            throw new Error(message.error);
          }
        } catch (err) {
          console.error('Message handling error:', err);
        }
      };

      ws.onerror = (err) => {
        console.error('WebSocket error:', err);
        setError('Connection error');
        setIsRunning(false);
      };

      ws.onclose = () => {
        console.log('WebSocket closed');
        if (isRunning && !result) {
          setError('Connection closed unexpectedly');
        }
        setIsRunning(false);
      };

    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
      setIsRunning(false);
    }
  }, [isRunning, result]);

  const stopTest = useCallback(() => {
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }
    setIsRunning(false);
  }, []);

  return {
    isRunning,
    progress,
    result,
    error,
    startTest,
    stopTest,
  };
}

function getStageMessage(stage: string, progress: number): string {
  const messages: Record<string, string> = {
    Initializing: 'Preparing test...',
    IdleLatency: 'Measuring baseline latency...',
    Download: `Testing download speed... ${progress}%`,
    Upload: `Testing upload speed... ${progress}%`,
    Finalizing: 'Calculating results...',
    Complete: 'Test complete!',
  };
  return messages[stage] || 'Testing...';
}
