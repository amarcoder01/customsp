import { useEffect, useState } from 'react';
import { motion } from 'framer-motion';

interface SpeedGaugeProps {
  value: number;
  maxValue: number;
  label: string;
  unit: string;
  color?: string;
}

export function SpeedGauge({ value, maxValue, label, unit, color = '#0ea5e9' }: SpeedGaugeProps) {
  const [displayValue, setDisplayValue] = useState(0);

  useEffect(() => {
    // Animate the value
    const duration = 2000;
    const steps = 60;
    const increment = value / steps;
    let current = 0;
    
    const timer = setInterval(() => {
      current += increment;
      if (current >= value) {
        setDisplayValue(value);
        clearInterval(timer);
      } else {
        setDisplayValue(current);
      }
    }, duration / steps);

    return () => clearInterval(timer);
  }, [value]);

  const percentage = Math.min((value / maxValue) * 100, 100);
  const circumference = 2 * Math.PI * 70;
  const strokeDashoffset = circumference - (percentage / 100) * circumference;

  return (
    <motion.div
      initial={{ scale: 0.9, opacity: 0 }}
      animate={{ scale: 1, opacity: 1 }}
      transition={{ duration: 0.5 }}
      className="flex flex-col items-center justify-center"
    >
      <div className="relative w-48 h-48">
        <svg className="w-full h-full transform -rotate-90">
          {/* Background circle */}
          <circle
            cx="96"
            cy="96"
            r="70"
            stroke="currentColor"
            strokeWidth="12"
            fill="none"
            className="text-gray-700 dark:text-gray-700"
          />
          {/* Progress circle */}
          <motion.circle
            cx="96"
            cy="96"
            r="70"
            stroke={color}
            strokeWidth="12"
            fill="none"
            strokeLinecap="round"
            initial={{ strokeDasharray: circumference, strokeDashoffset: circumference }}
            animate={{ strokeDashoffset }}
            transition={{ duration: 2, ease: 'easeOut' }}
            style={{
              strokeDasharray: circumference,
            }}
          />
        </svg>
        
        {/* Center text */}
        <div className="absolute inset-0 flex flex-col items-center justify-center">
          <motion.div
            initial={{ scale: 0 }}
            animate={{ scale: 1 }}
            transition={{ delay: 0.3, type: 'spring' }}
            className="text-4xl font-bold text-white"
          >
            {displayValue.toFixed(1)}
          </motion.div>
          <div className="text-sm text-gray-400 mt-1">{unit}</div>
        </div>
      </div>
      
      <div className="mt-4 text-center">
        <div className="text-lg font-semibold text-gray-200">{label}</div>
      </div>
    </motion.div>
  );
}
