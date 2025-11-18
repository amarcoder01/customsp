import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Zap, Activity } from 'lucide-react';
import { useSpeedTest } from './hooks/useSpeedTest';
import { SpeedGauge } from './components/SpeedGauge';
import { BufferbloatCard } from './components/BufferbloatCard';
import { AIMScoreCard } from './components/AIMScoreCard';
import { AIInsightsPanel } from './components/AIInsightsPanel';
import { ProgressOverlay } from './components/ProgressOverlay';
import { DarkModeToggle } from './components/DarkModeToggle';
import { ShareResults } from './components/ShareResults';
import { ExportResults } from './components/ExportResults';
import { ServerSelector } from './components/ServerSelector';
import { TestHistory } from './components/TestHistory';

function App() {
  const { isRunning, progress, result, error, startTest } = useSpeedTest();
  const [includeAI, setIncludeAI] = useState(false);
  const [showHistory, setShowHistory] = useState(false);

  return (
    <div className="min-h-screen bg-gradient-to-br from-dark-950 via-dark-900 to-dark-950 text-white">
      <DarkModeToggle />
      
      {/* Animated Background */}
      <div className="fixed inset-0 overflow-hidden pointer-events-none">
        <div className="absolute -top-1/2 -left-1/2 w-full h-full bg-primary-500/5 rounded-full blur-3xl animate-pulse-slow" />
        <div className="absolute -bottom-1/2 -right-1/2 w-full h-full bg-blue-500/5 rounded-full blur-3xl animate-pulse-slow" />
      </div>

      {/* Main Container */}
      <div className="relative z-10 container mx-auto px-4 py-8">
        {/* Header */}
        <motion.header
          initial={{ y: -50, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          className="text-center mb-12"
        >
          <div className="flex items-center justify-center gap-3 mb-4">
            <Zap className="w-12 h-12 text-primary-400" />
            <h1 className="text-5xl font-bold">
              <span className="bg-gradient-to-r from-primary-400 to-blue-500 bg-clip-text text-transparent">
                SpeedTestPro
              </span>
            </h1>
          </div>
          <p className="text-gray-400 text-lg">
            The world's most advanced internet speed test
          </p>
          <div className="flex items-center justify-center gap-6 mt-4 text-sm text-gray-500">
            <span className="flex items-center gap-1">
              <Activity className="w-4 h-4" /> Loaded Latency
            </span>
            <span>•</span>
            <span>AIM Scoring</span>
            <span>•</span>
            <span>AI Insights</span>
          </div>
        </motion.header>

        {/* Server Selector */}
        {!result && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="max-w-md mx-auto mb-6"
          >
            <ServerSelector />
          </motion.div>
        )}

        {/* Test Button */}
        {!result && (
          <motion.div
            initial={{ scale: 0.9, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            className="flex flex-col items-center mb-12"
          >
            <button
              onClick={() => startTest(includeAI)}
              disabled={isRunning}
              className="group relative px-12 py-6 bg-gradient-to-r from-primary-500 to-blue-600 rounded-2xl font-bold text-xl transition-all hover:scale-105 hover:shadow-2xl hover:shadow-primary-500/50 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
            >
              <span className="relative z-10 flex items-center gap-3">
                {isRunning ? (
                  <>
                    <motion.div animate={{ rotate: 360 }} transition={{ duration: 1, repeat: Infinity, ease: 'linear' }}>
                      <Zap className="w-6 h-6" />
                    </motion.div>
                    Running Test...
                  </>
                ) : (
                  <>
                    <Zap className="w-6 h-6" />
                    Start Speed Test
                  </>
                )}
              </span>
              <div className="absolute inset-0 bg-gradient-to-r from-primary-600 to-blue-700 rounded-2xl blur-xl opacity-50 group-hover:opacity-75 transition-opacity" />
            </button>

            <label className="flex items-center gap-2 mt-6 cursor-pointer">
              <input
                type="checkbox"
                checked={includeAI}
                onChange={(e) => setIncludeAI(e.target.checked)}
                disabled={isRunning}
                className="w-5 h-5 rounded border-2 border-primary-500 bg-dark-800 checked:bg-primary-500 focus:ring-2 focus:ring-primary-500 disabled:opacity-50"
              />
              <span className="text-gray-300">
                Include AI Insights (GPT-4 analysis)
              </span>
            </label>
          </motion.div>
        )}

        {/* Error Display */}
        {error && (
          <motion.div
            initial={{ opacity: 0, y: -20 }}
            animate={{ opacity: 1, y: 0 }}
            className="max-w-2xl mx-auto mb-8 p-4 bg-red-500/10 border border-red-500/30 rounded-lg text-red-400"
          >
            ⚠️ {error}
          </motion.div>
        )}

        {/* Results */}
        {result && (
          <div className="space-y-8">
            {/* Speed Gauges */}
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8"
            >
              <div className="bg-dark-800/50 backdrop-blur-lg rounded-2xl p-6 border border-dark-700">
                <SpeedGauge
                  value={result.basic.download_mbps}
                  maxValue={1000}
                  label="Download"
                  unit="Mbps"
                  color="#10b981"
                />
              </div>
              <div className="bg-dark-800/50 backdrop-blur-lg rounded-2xl p-6 border border-dark-700">
                <SpeedGauge
                  value={result.basic.upload_mbps}
                  maxValue={100}
                  label="Upload"
                  unit="Mbps"
                  color="#3b82f6"
                />
              </div>
              <div className="bg-dark-800/50 backdrop-blur-lg rounded-2xl p-6 border border-dark-700">
                <SpeedGauge
                  value={result.basic.latency_ms}
                  maxValue={100}
                  label="Latency"
                  unit="ms"
                  color="#8b5cf6"
                />
              </div>
            </motion.div>

            {/* Loaded Latency */}
            {result.loaded_latency && (
              <BufferbloatCard data={result.loaded_latency} />
            )}

            {/* AIM Scores */}
            {result.aim_scores && (
              <AIMScoreCard scores={result.aim_scores} />
            )}

            {/* AI Insights */}
            {result.ai_insights && (
              <AIInsightsPanel insights={result.ai_insights} />
            )}

            {/* Share & Export */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
              <ShareResults result={result} />
              <ExportResults result={result} />
            </div>

            {/* New Test Button */}
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="flex justify-center gap-4"
            >
              <button
                onClick={() => {
                  window.location.reload();
                }}
                className="px-8 py-4 bg-gradient-to-r from-primary-500 to-blue-600 rounded-xl font-semibold hover:scale-105 transition-all shadow-lg hover:shadow-primary-500/50"
              >
                Run Another Test
              </button>
              <button
                onClick={() => setShowHistory(!showHistory)}
                className="px-8 py-4 bg-dark-800/50 backdrop-blur-lg rounded-xl font-semibold hover:bg-dark-700/50 transition-all border border-dark-700"
              >
                {showHistory ? 'Hide' : 'Show'} History
              </button>
            </motion.div>
          </div>
        )}

        {/* Test History */}
        {showHistory && (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: 20 }}
            className="mt-8"
          >
            <TestHistory />
          </motion.div>
        )}
      </div>

      {/* Progress Overlay */}
      <AnimatePresence>
        {isRunning && progress && (
          <ProgressOverlay progress={progress} />
        )}
      </AnimatePresence>
    </div>
  );
}

export default App;
