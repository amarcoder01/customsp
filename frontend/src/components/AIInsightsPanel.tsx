import { motion } from 'framer-motion';
import { Brain, AlertCircle, CheckCircle, Zap, Lightbulb } from 'lucide-react';
import type { AIInsights } from '../types';

interface AIInsightsPanelProps {
  insights: AIInsights;
}

export function AIInsightsPanel({ insights }: AIInsightsPanelProps) {
  return (
    <motion.div
      initial={{ y: 20, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      transition={{ delay: 0.5 }}
      className="bg-gradient-to-br from-purple-900/30 to-blue-900/30 backdrop-blur-lg rounded-2xl p-6 border border-purple-500/30"
    >
      <div className="flex items-center gap-3 mb-6">
        <div className="p-2 bg-purple-500/20 rounded-lg">
          <Brain className="w-6 h-6 text-purple-400" />
        </div>
        <div>
          <h3 className="text-xl font-bold text-white">AI-Powered Insights</h3>
          <p className="text-sm text-gray-400">GPT-4 Analysis</p>
        </div>
      </div>

      {/* Summary */}
      <div className="mb-6">
        <div className="flex items-center gap-2 mb-2">
          <Zap className="w-4 h-4 text-yellow-400" />
          <h4 className="text-sm font-semibold text-gray-300">Quick Summary</h4>
        </div>
        <p className="text-gray-200 leading-relaxed">
          {insights.summary}
        </p>
      </div>

      {/* Recommendations */}
      <div className="mb-6">
        <h4 className="text-sm font-semibold text-gray-300 mb-3 flex items-center gap-2">
          <CheckCircle className="w-4 h-4 text-green-400" />
          Recommendations
        </h4>
        <div className="space-y-3">
          {insights.recommendations.map((rec, index) => (
            <motion.div
              key={index}
              initial={{ x: -20, opacity: 0 }}
              animate={{ x: 0, opacity: 1 }}
              transition={{ delay: 0.6 + index * 0.1 }}
              className="bg-dark-900/50 rounded-lg p-4"
            >
              <div className="flex items-start gap-3">
                <div className={`px-2 py-1 rounded text-xs font-semibold ${getPriorityStyle(rec.priority)}`}>
                  {rec.priority}
                </div>
                <div className="flex-1">
                  <h5 className="font-semibold text-white mb-1">{rec.title}</h5>
                  <p className="text-sm text-gray-400 mb-2">{rec.description}</p>
                  <div className="flex items-center gap-4 text-xs text-gray-500">
                    <span>💡 {rec.expected_improvement}</span>
                    <span>🔧 {rec.difficulty}</span>
                  </div>
                </div>
              </div>
            </motion.div>
          ))}
        </div>
      </div>

      {/* Predictions */}
      {insights.predictions.length > 0 && (
        <div className="mb-6">
          <h4 className="text-sm font-semibold text-gray-300 mb-3 flex items-center gap-2">
            <AlertCircle className="w-4 h-4 text-orange-400" />
            Predictions
          </h4>
          <ul className="space-y-2">
            {insights.predictions.map((prediction, index) => (
              <motion.li
                key={index}
                initial={{ x: -20, opacity: 0 }}
                animate={{ x: 0, opacity: 1 }}
                transition={{ delay: 0.8 + index * 0.1 }}
                className="flex items-start gap-2 text-sm text-gray-300"
              >
                <span className="text-orange-400 mt-1">⚠️</span>
                <span>{prediction}</span>
              </motion.li>
            ))}
          </ul>
        </div>
      )}

      {/* ELI5 Explanation */}
      <div className="bg-blue-900/20 rounded-lg p-4">
        <div className="flex items-center gap-2 mb-2">
          <Lightbulb className="w-4 h-4 text-blue-400" />
          <h4 className="text-sm font-semibold text-gray-300">Explain Like I'm 5</h4>
        </div>
        <p className="text-sm text-gray-300 leading-relaxed">
          {insights.simple_explanation}
        </p>
      </div>
    </motion.div>
  );
}

function getPriorityStyle(priority: string) {
  const styles: Record<string, string> = {
    Critical: 'bg-red-500/20 text-red-400',
    High: 'bg-orange-500/20 text-orange-400',
    Medium: 'bg-yellow-500/20 text-yellow-400',
    Low: 'bg-blue-500/20 text-blue-400',
  };
  return styles[priority] || 'bg-gray-500/20 text-gray-400';
}
