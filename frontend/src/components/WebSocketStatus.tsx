import { useState, useEffect } from "react";
import { Card, CardContent } from "./ui/card";
import { Badge } from "./ui/badge";
import { Button } from "./ui/button";
import { Wifi, WifiOff, Activity, AlertCircle, CheckCircle } from "lucide-react";
import { FitnessApiClient } from "../api/client";

export function WebSocketStatus() {
  const [mcpStatus, setMcpStatus] = useState<'connected' | 'connecting' | 'disconnected' | 'error'>('disconnected');
  const [lastMessage, setLastMessage] = useState<any>(null);
  const [messageCount, setMessageCount] = useState(0);
  const [connectionTime, setConnectionTime] = useState<Date | null>(null);

  useEffect(() => {
    // Check connection status every second
    const statusInterval = setInterval(() => {
      const status = FitnessApiClient.getMCPConnectionStatus();
      setMcpStatus(status);
    }, 1000);

    return () => clearInterval(statusInterval);
  }, []);

  const connectWebSocket = () => {
    try {
      const ws = FitnessApiClient.createMCPWebSocket();
      setConnectionTime(new Date());
      
      // Listen for messages (this would be enhanced in a real implementation)
      ws.addEventListener('message', (event) => {
        try {
          const message = JSON.parse(event.data);
          setLastMessage(message);
          setMessageCount(prev => prev + 1);
        } catch (error) {
          console.error('Failed to parse WebSocket message:', error);
        }
      });

      ws.addEventListener('open', () => {
        setConnectionTime(new Date());
      });

    } catch (error) {
      console.error('Failed to connect WebSocket:', error);
    }
  };

  const getStatusIcon = () => {
    switch (mcpStatus) {
      case 'connected': 
        return <Wifi className="h-4 w-4 text-green-400" />;
      case 'connecting': 
        return <Activity className="h-4 w-4 text-yellow-400 animate-pulse" />;
      case 'error': 
        return <AlertCircle className="h-4 w-4 text-red-400" />;
      default: 
        return <WifiOff className="h-4 w-4 text-gray-400" />;
    }
  };

  const getStatusColor = () => {
    switch (mcpStatus) {
      case 'connected': return 'text-green-300 border-green-500/30';
      case 'connecting': return 'text-yellow-300 border-yellow-500/30';
      case 'error': return 'text-red-300 border-red-500/30';
      default: return 'text-gray-300 border-gray-500/30';
    }
  };

  const getStatusText = () => {
    switch (mcpStatus) {
      case 'connected': return 'Connected';
      case 'connecting': return 'Connecting...';
      case 'error': return 'Error';
      default: return 'Disconnected';
    }
  };

  return (
    <Card className="bg-black/40 backdrop-blur-lg border-white/10 text-white">
      <CardContent className="p-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            {getStatusIcon()}
            <div className="space-y-1">
              <div className="flex items-center gap-2">
                <span className="text-sm font-medium">MCP WebSocket</span>
                <Badge variant="outline" className={getStatusColor()}>
                  {getStatusText()}
                </Badge>
              </div>
              <div className="flex items-center gap-4 text-xs text-white/60">
                {connectionTime && mcpStatus === 'connected' && (
                  <span>Connected: {connectionTime.toLocaleTimeString()}</span>
                )}
                {messageCount > 0 && (
                  <span>Messages: {messageCount}</span>
                )}
              </div>
            </div>
          </div>

          <div className="flex gap-2">
            {mcpStatus === 'disconnected' && (
              <Button
                size="sm"
                onClick={connectWebSocket}
                className="bg-blue-600 hover:bg-blue-700"
              >
                Connect
              </Button>
            )}
            
            {mcpStatus === 'connected' && (
              <div className="flex items-center gap-1">
                <CheckCircle className="h-4 w-4 text-green-400" />
                <span className="text-xs text-green-400">Live</span>
              </div>
            )}
          </div>
        </div>

        {lastMessage && (
          <div className="mt-3 p-2 bg-white/5 rounded text-xs">
            <div className="text-white/60 mb-1">Last Message:</div>
            <div className="text-white/80 font-mono truncate">
              {JSON.stringify(lastMessage).slice(0, 100)}...
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}