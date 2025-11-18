import { motion } from 'framer-motion';
import { Loader2 } from 'lucide-react';
import type { TestProgress } from '../types';

interface ProgressOverlayProps {
  progress: TestProgress;
}

export function ProgressOverlay({ progress }: ProgressOverlayProps) {
  const percentage = progress.progress_pct || 0;

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50"
    >
      <motion.div
        initial={{ scale: 0.9, y: 20 }}
        animate={{ scale: 1, y: 0 }}
        className="bg-dark-900/90 rounded-2xl p-8 max-w-md w-full mx-4 border border-dark-700"
      >
        {/* Stage indicator */}
        <div className="text-center mb-6">
          <motion.div
            animate={{ rotate: 360 }}
            transition={{ duration: 2, repeat: Infinity, ease: 'linear' }}
            className="inline-block"
          >
            <Loader2 className="w-16 h-16 text-primary-400" />
          </motion.div>
          
          <h3 className="text-2xl font-bold text-white mt-4">
            {getStageTitle(progress.stage || 'Initializing')}
          </h3>
          <p className="text-gray-400 mt-2">
            {progress.message || 'Running test...'}
          </p>
        </div>

        {/* Progress bar */}
        <div className="relative">
          <div className="w-full bg-dark-700 rounded-full h-3 overflow-hidden">
            <motion.div
              initial={{ width: 0 }}
              animate={{ width: `${percentage}%` }}
              transition={{ duration: 0.3 }}
              className="h-full bg-gradient-to-r from-primary-500 to-blue-600 rounded-full"
            />
          </div>
          <div className="flex justify-between mt-2 text-sm text-gray-400">
            <span>{percentage}%</span>
            <span>Complete</span>
          </div>
        </div>

        {/* Current metrics */}
        {(progress.current_speed_mbps || progress.current_latency_ms) && (
          <div className="mt-6 grid grid-cols-2 gap-4">
            {progress.current_speed_mbps !== undefined && (
              <div className="text-center p-3 bg-dark-800/50 rounded-lg">
                <div className="text-2xl font-bold text-primary-400">
                  {progress.current_speed_mbps.toFixed(1)}
                </div>
                <div className="text-xs text-gray-500">Mbps</div>
              </div>
            )}
            {progress.current_latency_ms !== undefined && (
              <div className="text-center p-3 bg-dark-800/50 rounded-lg">
                <div className="text-2xl font-bold text-green-400">
                  {progress.current_latency_ms.toFixed(1)}
                </div>
                <div className="text-xs text-gray-500">ms</div>
              </div>
            )}
          </div>
        )}
      </motion.div>
    </motion.div>
  );
}

function getStageTitle(stage: string): string {
  const titles: Record<string, string> = {
    Initializing: 'Initializing Test',
    IdleLatency: 'Measuring Baseline',
    Download: 'Testing Download',
    Upload: 'Testing Upload',
    Finalizing: 'Analyzing Results',
    Complete: 'Test Complete',
  };
  return titles[stage] || 'Running Test';
}
