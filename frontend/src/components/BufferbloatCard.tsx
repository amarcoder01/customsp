import { motion } from 'framer-motion';
import { AlertTriangle, CheckCircle, XCircle, Activity } from 'lucide-react';
import type { LoadedLatencyResult, BufferbloatGrade } from '../types';

interface BufferbloatCardProps {
  data: LoadedLatencyResult;
}

export function BufferbloatCard({ data }: BufferbloatCardProps) {
  const gradeInfo = getGradeInfo(data.bufferbloat_grade);

  return (
    <motion.div
      initial={{ y: 20, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      transition={{ delay: 0.3 }}
      className="bg-dark-800/50 backdrop-blur-lg rounded-2xl p-6 border border-dark-700"
    >
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-3">
          <Activity className="w-6 h-6 text-primary-400" />
          <h3 className="text-xl font-bold text-white">Loaded Latency</h3>
        </div>
        <div className={`px-4 py-2 rounded-full font-bold ${gradeInfo.bgColor} ${gradeInfo.textColor}`}>
          {data.bufferbloat_grade}
        </div>
      </div>

      <div className="space-y-4">
        {/* Idle Latency */}
        <LatencyRow
          label="Idle"
          value={data.idle_avg_ms}
          rpm={data.idle_rpm}
          icon={<CheckCircle className="w-5 h-5 text-green-400" />}
        />

        {/* Download Loaded Latency */}
        <LatencyRow
          label="Under Load (Download)"
          value={data.download_avg_ms}
          rpm={data.download_rpm}
          increase={data.bufferbloat_download_ratio}
          icon={getLatencyIcon(data.bufferbloat_download_ratio)}
        />

        {/* Upload Loaded Latency */}
        <LatencyRow
          label="Under Load (Upload)"
          value={data.upload_avg_ms}
          rpm={data.upload_rpm}
          increase={data.bufferbloat_upload_ratio}
          icon={getLatencyIcon(data.bufferbloat_upload_ratio)}
        />
      </div>

      {/* Explanation */}
      <div className="mt-6 p-4 bg-dark-900/50 rounded-lg">
        <p className="text-sm text-gray-300">
          {gradeInfo.explanation}
        </p>
      </div>
    </motion.div>
  );
}

interface LatencyRowProps {
  label: string;
  value: number;
  rpm: number;
  increase?: number;
  icon: React.ReactNode;
}

function LatencyRow({ label, value, rpm, increase, icon }: LatencyRowProps) {
  return (
    <div className="flex items-center justify-between p-3 bg-dark-900/30 rounded-lg">
      <div className="flex items-center gap-3">
        {icon}
        <div>
          <div className="text-sm text-gray-400">{label}</div>
          <div className="text-lg font-semibold text-white">
            {value.toFixed(1)} ms
            {increase && (
              <span className={`ml-2 text-sm ${increase > 3 ? 'text-red-400' : 'text-yellow-400'}`}>
                (+{((increase - 1) * 100).toFixed(0)}%)
              </span>
            )}
          </div>
        </div>
      </div>
      <div className="text-right">
        <div className="text-sm text-gray-400">RPM</div>
        <div className="text-lg font-semibold text-primary-400">
          {rpm.toFixed(0)}
        </div>
      </div>
    </div>
  );
}

function getLatencyIcon(ratio: number) {
  if (ratio < 2) return <CheckCircle className="w-5 h-5 text-green-400" />;
  if (ratio < 5) return <AlertTriangle className="w-5 h-5 text-yellow-400" />;
  return <XCircle className="w-5 h-5 text-red-400" />;
}

function getGradeInfo(grade: BufferbloatGrade) {
  const gradeMap: Record<BufferbloatGrade, { bgColor: string; textColor: string; explanation: string }> = {
    'A+': {
      bgColor: 'bg-green-500/20',
      textColor: 'text-green-400',
      explanation: '🎉 Perfect! Your connection has minimal bufferbloat. Ideal for gaming and video calls.',
    },
    'A': {
      bgColor: 'bg-green-500/20',
      textColor: 'text-green-400',
      explanation: '✅ Excellent! Very low bufferbloat. Great for all real-time applications.',
    },
    'B': {
      bgColor: 'bg-blue-500/20',
      textColor: 'text-blue-400',
      explanation: '👍 Good! Acceptable bufferbloat for most activities.',
    },
    'C': {
      bgColor: 'bg-yellow-500/20',
      textColor: 'text-yellow-400',
      explanation: '⚠️ Moderate bufferbloat detected. May cause lag in gaming and video calls. Enable SQM/QoS.',
    },
    'D': {
      bgColor: 'bg-orange-500/20',
      textColor: 'text-orange-400',
      explanation: '🔴 Significant bufferbloat. Gaming and video calls will be problematic. Enable SQM immediately.',
    },
    'F': {
      bgColor: 'bg-red-500/20',
      textColor: 'text-red-400',
      explanation: '❌ Severe bufferbloat! Your connection becomes very laggy under load. Contact ISP or enable SQM.',
    },
  };
  return gradeMap[grade];
}
