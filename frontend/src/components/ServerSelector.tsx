import { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Server, MapPin, Activity, ChevronDown } from 'lucide-react';

interface ServerInfo {
  id: string;
  name: string;
  location: {
    lat: number;
    lon: number;
  };
  available: boolean;
  load: number;
}

export function ServerSelector() {
  const [servers, setServers] = useState<ServerInfo[]>([]);
  const [selectedServer, setSelectedServer] = useState<ServerInfo | null>(null);
  const [isOpen, setIsOpen] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchServers();
  }, []);

  const fetchServers = async () => {
    try {
      const response = await fetch('http://localhost:8080/api/servers');
      if (response.ok) {
        const data = await response.json();
        setServers(data.servers || []);
        if (data.servers && data.servers.length > 0) {
          setSelectedServer(data.servers[0]);
        }
      }
    } catch (error) {
      console.error('Failed to fetch servers:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="bg-dark-800/50 backdrop-blur-lg rounded-lg p-4 border border-dark-700 animate-pulse">
        <div className="h-10 bg-dark-700 rounded" />
      </div>
    );
  }

  return (
    <div className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-full bg-dark-800/50 backdrop-blur-lg rounded-lg p-4 border border-dark-700 hover:bg-dark-800/70 transition-all flex items-center justify-between"
      >
        <div className="flex items-center gap-3">
          <Server className="w-5 h-5 text-primary-400" />
          <div className="text-left">
            <div className="text-sm text-gray-500">Test Server</div>
            <div className="font-semibold text-white">
              {selectedServer?.name || 'Select Server'}
            </div>
          </div>
        </div>
        <ChevronDown className={`w-5 h-5 text-gray-400 transition-transform ${isOpen ? 'rotate-180' : ''}`} />
      </button>

      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0, y: -10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            className="absolute top-full left-0 right-0 mt-2 bg-dark-800 backdrop-blur-lg rounded-lg border border-dark-700 overflow-hidden z-50 shadow-2xl"
          >
            {servers.map((server) => (
              <button
                key={server.id}
                onClick={() => {
                  setSelectedServer(server);
                  setIsOpen(false);
                }}
                disabled={!server.available}
                className={`w-full p-4 flex items-center justify-between hover:bg-dark-700/50 transition-all disabled:opacity-50 disabled:cursor-not-allowed ${
                  selectedServer?.id === server.id ? 'bg-primary-500/10' : ''
                }`}
              >
                <div className="flex items-center gap-3">
                  <MapPin className="w-4 h-4 text-primary-400" />
                  <div className="text-left">
                    <div className="font-semibold text-white">{server.name}</div>
                    <div className="text-xs text-gray-500">
                      {server.available ? (
                        <span className="text-green-400">● Online</span>
                      ) : (
                        <span className="text-red-400">● Offline</span>
                      )}
                    </div>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  <Activity className="w-4 h-4 text-gray-500" />
                  <span className="text-sm text-gray-400">{(server.load * 100).toFixed(0)}%</span>
                </div>
              </button>
            ))}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
