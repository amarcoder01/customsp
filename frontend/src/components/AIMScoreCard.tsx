import { motion } from 'framer-motion';
import { Gamepad2, Video, Users, Globe } from 'lucide-react';
import type { AIMScores } from '../types';

interface AIMScoreCardProps {
  scores: AIMScores;
}

export function AIMScoreCard({ scores }: AIMScoreCardProps) {
  const useCases = [
    {
      icon: <Gamepad2 className="w-6 h-6" />,
      label: 'Gaming',
      score: scores.gaming,
      color: 'from-purple-500 to-pink-500',
    },
    {
      icon: <Video className="w-6 h-6" />,
      label: 'Streaming',
      score: scores.streaming,
      color: 'from-red-500 to-orange-500',
    },
    {
      icon: <Users className="w-6 h-6" />,
      label: 'Video Calls',
      score: scores.video_conferencing,
      color: 'from-blue-500 to-cyan-500',
    },
    {
      icon: <Globe className="w-6 h-6" />,
      label: 'Browsing',
      score: scores.general_browsing,
      color: 'from-green-500 to-emerald-500',
    },
  ];

  return (
    <motion.div
      initial={{ y: 20, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      transition={{ delay: 0.4 }}
      className="bg-dark-800/50 backdrop-blur-lg rounded-2xl p-6 border border-dark-700"
    >
      <h3 className="text-xl font-bold text-white mb-6">Use-Case Performance</h3>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
        {useCases.map((useCase, index) => (
          <motion.div
            key={useCase.label}
            initial={{ scale: 0.9, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            transition={{ delay: 0.5 + index * 0.1 }}
            className="bg-dark-900/50 rounded-xl p-4 hover:bg-dark-900/70 transition-all"
          >
            <div className="flex items-center justify-between mb-3">
              <div className="flex items-center gap-3">
                <div className={`p-2 rounded-lg bg-gradient-to-br ${useCase.color}`}>
                  {useCase.icon}
                </div>
                <div>
                  <div className="text-sm font-semibold text-gray-300">{useCase.label}</div>
                  <div className="text-xs text-gray-500">{useCase.score.grade}</div>
                </div>
              </div>
              <div className="text-2xl font-bold text-white">
                {useCase.score.score}
                <span className="text-sm text-gray-500">/100</span>
              </div>
            </div>

            {/* Progress bar */}
            <div className="w-full bg-dark-700 rounded-full h-2 overflow-hidden">
              <motion.div
                initial={{ width: 0 }}
                animate={{ width: `${useCase.score.score}%` }}
                transition={{ delay: 0.7 + index * 0.1, duration: 1 }}
                className={`h-full rounded-full bg-gradient-to-r ${useCase.color}`}
              />
            </div>

            <p className="mt-3 text-xs text-gray-400 line-clamp-2">
              {useCase.score.assessment}
            </p>
          </motion.div>
        ))}
      </div>

      {/* Overall Score */}
      <div className="relative overflow-hidden bg-gradient-to-br from-primary-500/20 to-blue-600/20 rounded-xl p-6 border border-primary-500/30">
        <div className="relative z-10">
          <div className="flex items-center justify-between">
            <div>
              <div className="text-sm text-gray-400 mb-1">Overall Score</div>
              <div className="text-4xl font-bold text-white">
                {scores.overall_score.toFixed(0)}
                <span className="text-xl text-gray-500">/100</span>
              </div>
              <div className="mt-2">
                <span className={`px-3 py-1 rounded-full text-sm font-semibold ${getGradeStyle(scores.overall_grade)}`}>
                  {scores.overall_grade}
                </span>
              </div>
            </div>
            
            {/* Circular progress */}
            <div className="relative w-24 h-24">
              <svg className="w-full h-full transform -rotate-90">
                <circle
                  cx="48"
                  cy="48"
                  r="40"
                  stroke="currentColor"
                  strokeWidth="8"
                  fill="none"
                  className="text-dark-700"
                />
                <motion.circle
                  cx="48"
                  cy="48"
                  r="40"
                  stroke="url(#gradient)"
                  strokeWidth="8"
                  fill="none"
                  strokeLinecap="round"
                  initial={{ strokeDasharray: 251, strokeDashoffset: 251 }}
                  animate={{ strokeDashoffset: 251 - (scores.overall_score / 100) * 251 }}
                  transition={{ delay: 0.8, duration: 1.5 }}
                  style={{ strokeDasharray: 251 }}
                />
                <defs>
                  <linearGradient id="gradient" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" stopColor="#0ea5e9" />
                    <stop offset="100%" stopColor="#3b82f6" />
                  </linearGradient>
                </defs>
              </svg>
            </div>
          </div>
        </div>
      </div>
    </motion.div>
  );
}

function getGradeStyle(grade: string) {
  const grades: Record<string, string> = {
    'Excellent': 'bg-green-500/20 text-green-400',
    'Good': 'bg-blue-500/20 text-blue-400',
    'Fair': 'bg-yellow-500/20 text-yellow-400',
    'Poor': 'bg-orange-500/20 text-orange-400',
    'Very Poor': 'bg-red-500/20 text-red-400',
  };
  return grades[grade] || 'bg-gray-500/20 text-gray-400';
}
