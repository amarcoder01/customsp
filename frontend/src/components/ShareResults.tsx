import { useState } from 'react';
import { motion } from 'framer-motion';
import { Share2, Copy, Check, Twitter, Facebook, Link2 } from 'lucide-react';
import type { EnhancedTestResult } from '../types';

interface ShareResultsProps {
  result: EnhancedTestResult;
}

export function ShareResults({ result }: ShareResultsProps) {
  const [copied, setCopied] = useState(false);

  const shareUrl = typeof window !== 'undefined' ? window.location.href : '';
  const shareText = `I just tested my internet speed on SpeedTestPro! 🚀\n\n` +
    `📥 Download: ${result.basic.download_mbps.toFixed(1)} Mbps\n` +
    `📤 Upload: ${result.basic.upload_mbps.toFixed(1)} Mbps\n` +
    `⚡ Latency: ${result.basic.latency_ms.toFixed(1)} ms\n` +
    (result.loaded_latency ? `🎯 Bufferbloat: ${result.loaded_latency.bufferbloat_grade}\n` : '') +
    (result.aim_scores ? `📊 Overall Score: ${result.aim_scores.overall_score.toFixed(0)}/100` : '');

  const copyToClipboard = async () => {
    try {
      await navigator.clipboard.writeText(shareText);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  };

  const shareToTwitter = () => {
    const url = `https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}&url=${encodeURIComponent(shareUrl)}`;
    window.open(url, '_blank', 'width=550,height=420');
  };

  const shareToFacebook = () => {
    const url = `https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(shareUrl)}`;
    window.open(url, '_blank', 'width=550,height=420');
  };

  const shareNative = async () => {
    if (navigator.share) {
      try {
        await navigator.share({
          title: 'My SpeedTestPro Results',
          text: shareText,
          url: shareUrl,
        });
      } catch (err) {
        console.error('Share failed:', err);
      }
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: 0.6 }}
      className="bg-dark-800/50 backdrop-blur-lg rounded-2xl p-6 border border-dark-700"
    >
      <div className="flex items-center gap-3 mb-6">
        <Share2 className="w-6 h-6 text-primary-400" />
        <h3 className="text-xl font-bold text-white">Share Results</h3>
      </div>

      <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
        <button
          onClick={copyToClipboard}
          className="flex flex-col items-center justify-center gap-2 p-4 bg-dark-900/50 rounded-lg hover:bg-dark-900/70 transition-all group"
        >
          {copied ? (
            <Check className="w-6 h-6 text-green-400" />
          ) : (
            <Copy className="w-6 h-6 text-gray-400 group-hover:text-primary-400 transition-colors" />
          )}
          <span className="text-sm text-gray-400 group-hover:text-gray-300">
            {copied ? 'Copied!' : 'Copy'}
          </span>
        </button>

        <button
          onClick={shareToTwitter}
          className="flex flex-col items-center justify-center gap-2 p-4 bg-dark-900/50 rounded-lg hover:bg-dark-900/70 transition-all group"
        >
          <Twitter className="w-6 h-6 text-gray-400 group-hover:text-blue-400 transition-colors" />
          <span className="text-sm text-gray-400 group-hover:text-gray-300">Twitter</span>
        </button>

        <button
          onClick={shareToFacebook}
          className="flex flex-col items-center justify-center gap-2 p-4 bg-dark-900/50 rounded-lg hover:bg-dark-900/70 transition-all group"
        >
          <Facebook className="w-6 h-6 text-gray-400 group-hover:text-blue-500 transition-colors" />
          <span className="text-sm text-gray-400 group-hover:text-gray-300">Facebook</span>
        </button>

        {typeof navigator !== 'undefined' && 'share' in navigator && (
          <button
            onClick={shareNative}
            className="flex flex-col items-center justify-center gap-2 p-4 bg-dark-900/50 rounded-lg hover:bg-dark-900/70 transition-all group"
          >
            <Link2 className="w-6 h-6 text-gray-400 group-hover:text-primary-400 transition-colors" />
            <span className="text-sm text-gray-400 group-hover:text-gray-300">Share</span>
          </button>
        )}
      </div>

      {/* Share text preview */}
      <div className="mt-6 p-4 bg-dark-900/50 rounded-lg">
        <div className="text-xs text-gray-500 mb-2">Preview:</div>
        <div className="text-sm text-gray-300 whitespace-pre-wrap font-mono">
          {shareText}
        </div>
      </div>
    </motion.div>
  );
}
