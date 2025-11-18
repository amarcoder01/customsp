import { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import { History, Trash2, Download, Calendar } from 'lucide-react';
import type { TestResult } from '../types';

export function TestHistory() {
  const [history, setHistory] = useState<TestResult[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchHistory();
  }, []);

  const fetchHistory = async () => {
    try {
      const response = await fetch('http://localhost:8080/api/test/history');
      if (response.ok) {
        const data = await response.json();
        setHistory(data.tests || []);
      }
    } catch (error) {
      console.error('Failed to fetch history:', error);
    } finally {
      setLoading(false);
    }
  };

  const clearHistory = () => {
    if (confirm('Clear all test history?')) {
      setHistory([]);
      localStorage.removeItem('test_history');
    }
  };

  const exportHistory = () => {
    const dataStr = JSON.stringify(history, null, 2);
    const dataBlob = new Blob([dataStr], { type: 'application/json' });
    const url = URL.createObjectURL(dataBlob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `speedtest-history-${new Date().toISOString()}.json`;
    link.click();
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-primary-500" />
      </div>
    );
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      className="bg-dark-800/50 backdrop-blur-lg rounded-2xl p-6 border border-dark-700"
    >
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-3">
          <History className="w-6 h-6 text-primary-400" />
          <h3 className="text-xl font-bold text-white">Test History</h3>
          <span className="text-sm text-gray-500">({history.length} tests)</span>
        </div>
        <div className="flex gap-2">
          <button
            onClick={exportHistory}
            disabled={history.length === 0}
            className="px-4 py-2 bg-primary-500/20 text-primary-400 rounded-lg hover:bg-primary-500/30 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          >
            <Download className="w-4 h-4" />
            Export
          </button>
          <button
            onClick={clearHistory}
            disabled={history.length === 0}
            className="px-4 py-2 bg-red-500/20 text-red-400 rounded-lg hover:bg-red-500/30 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          >
            <Trash2 className="w-4 h-4" />
            Clear
          </button>
        </div>
      </div>

      {history.length === 0 ? (
        <div className="text-center py-12 text-gray-500">
          <History className="w-16 h-16 mx-auto mb-4 opacity-30" />
          <p>No test history yet</p>
          <p className="text-sm mt-2">Run a speed test to see results here</p>
        </div>
      ) : (
        <div className="space-y-3">
          {history.map((test, index) => (
            <motion.div
              key={test.id}
              initial={{ opacity: 0, x: -20 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ delay: index * 0.05 }}
              className="bg-dark-900/50 rounded-lg p-4 hover:bg-dark-900/70 transition-all"
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-4">
                  <div className="text-gray-500">
                    <Calendar className="w-5 h-5" />
                  </div>
                  <div>
                    <div className="text-sm text-gray-400">
                      {new Date(test.timestamp).toLocaleString()}
                    </div>
                    <div className="flex gap-4 mt-1 text-sm">
                      <span className="text-green-400">↓ {test.download_mbps.toFixed(1)} Mbps</span>
                      <span className="text-blue-400">↑ {test.upload_mbps.toFixed(1)} Mbps</span>
                      <span className="text-purple-400">⚡ {test.latency_ms.toFixed(1)} ms</span>
                    </div>
                  </div>
                </div>
                <div className="text-xs text-gray-500">
                  {test.server_id}
                </div>
              </div>
            </motion.div>
          ))}
        </div>
      )}
    </motion.div>
  );
}
