import { useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Button } from "./ui/button";
import { Badge } from "./ui/badge";
import { AlertCircle, RefreshCw, ChevronDown, ChevronUp, X, HelpCircle } from "lucide-react";
import { MCPErrorHandler } from "../utils/mcpErrorHandler";

interface MCPErrorDisplayProps {
  error: any;
  context?: string;
  onRetry?: () => void;
  onDismiss?: () => void;
  showDetails?: boolean;
}

export function MCPErrorDisplay({ 
  error, 
  context, 
  onRetry, 
  onDismiss,
  showDetails = false 
}: MCPErrorDisplayProps) {
  const [isExpanded, setIsExpanded] = useState(showDetails);
  const [showSuggestions, setShowSuggestions] = useState(false);

  if (!error) return null;

  const formattedError = MCPErrorHandler.formatUserFriendlyError(error, context);

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'critical': return 'bg-red-600/20 border-red-500/30 text-red-300';
      case 'high': return 'bg-orange-600/20 border-orange-500/30 text-orange-300';
      case 'medium': return 'bg-yellow-600/20 border-yellow-500/30 text-yellow-300';
      default: return 'bg-blue-600/20 border-blue-500/30 text-blue-300';
    }
  };

  const getSeverityIcon = (severity: string) => {
    switch (severity) {
      case 'critical': return <AlertCircle className="h-5 w-5 text-red-400" />;
      case 'high': return <AlertCircle className="h-5 w-5 text-orange-400" />;
      case 'medium': return <AlertCircle className="h-5 w-5 text-yellow-400" />;
      default: return <AlertCircle className="h-5 w-5 text-blue-400" />;
    }
  };

  return (
    <div className={`rounded-lg border p-4 ${getSeverityColor(formattedError.severity)}`}>
      <div className="flex items-start justify-between">
        <div className="flex items-start gap-3 flex-1">
          {getSeverityIcon(formattedError.severity)}
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 mb-1">
              <span className="font-medium text-sm">MCP Error</span>
              <Badge variant="outline" className="text-xs capitalize">
                {formattedError.severity}
              </Badge>
              {context && (
                <Badge variant="outline" className="text-xs">
                  {context}
                </Badge>
              )}
            </div>
            <p className="text-sm text-white/90">{formattedError.message}</p>
          </div>
        </div>

        <div className="flex items-center gap-1 ml-2">
          {formattedError.suggestions.length > 0 && (
            <Button
              size="sm"
              variant="ghost"
              onClick={() => setShowSuggestions(!showSuggestions)}
              className="h-8 w-8 p-0"
            >
              <HelpCircle className="h-4 w-4" />
            </Button>
          )}

          <Button
            size="sm"
            variant="ghost"
            onClick={() => setIsExpanded(!isExpanded)}
            className="h-8 w-8 p-0"
          >
            {isExpanded ? <ChevronUp className="h-4 w-4" /> : <ChevronDown className="h-4 w-4" />}
          </Button>

          {onDismiss && (
            <Button
              size="sm"
              variant="ghost"
              onClick={onDismiss}
              className="h-8 w-8 p-0"
            >
              <X className="h-4 w-4" />
            </Button>
          )}
        </div>
      </div>

      {showSuggestions && formattedError.suggestions.length > 0 && (
        <div className="mt-3 pt-3 border-t border-white/10">
          <h4 className="text-xs font-medium mb-2 text-white/80">Suggested Solutions:</h4>
          <ul className="space-y-1">
            {formattedError.suggestions.map((suggestion, index) => (
              <li key={index} className="text-xs text-white/70 flex items-start gap-2">
                <span className="text-white/40 mt-1">•</span>
                <span>{suggestion}</span>
              </li>
            ))}
          </ul>
        </div>
      )}

      {isExpanded && (
        <div className="mt-3 pt-3 border-t border-white/10">
          <div className="space-y-2">
            <div className="text-xs">
              <span className="text-white/60">Error Code:</span>
              <span className="ml-2 font-mono text-white/80">
                {error?.code || error?.status || 'UNKNOWN'}
              </span>
            </div>
            <div className="text-xs">
              <span className="text-white/60">Timestamp:</span>
              <span className="ml-2 text-white/80">
                {new Date().toLocaleString()}
              </span>
            </div>
            {context && (
              <div className="text-xs">
                <span className="text-white/60">Context:</span>
                <span className="ml-2 text-white/80">{context}</span>
              </div>
            )}
          </div>

          {(error?.details || error?.stack) && (
            <div className="mt-3">
              <div className="text-xs text-white/60 mb-1">Technical Details:</div>
              <div className="bg-black/20 rounded p-2 max-h-32 overflow-auto">
                <pre className="text-xs font-mono text-white/70 whitespace-pre-wrap">
                  {JSON.stringify(error?.details || error, null, 2)}
                </pre>
              </div>
            </div>
          )}
        </div>
      )}

      {formattedError.canRetry && onRetry && (
        <div className="mt-3 pt-3 border-t border-white/10 flex gap-2">
          <Button
            size="sm"
            onClick={onRetry}
            className="bg-blue-600/80 hover:bg-blue-600"
          >
            <RefreshCw className="h-4 w-4 mr-2" />
            Try Again
          </Button>
        </div>
      )}
    </div>
  );
}

// Error boundary component for MCP operations
interface MCPErrorBoundaryProps {
  children: React.ReactNode;
  context?: string;
  onError?: (error: any, context?: string) => void;
}

export function MCPErrorBoundary({ children, context, onError }: MCPErrorBoundaryProps) {
  const [error, setError] = useState<any>(null);

  const handleError = (err: any) => {
    const mcpError = MCPErrorHandler.handleMCPError(err, context);
    setError(mcpError);
    onError?.(mcpError, context);
  };

  const retry = () => {
    setError(null);
  };

  if (error) {
    return (
      <MCPErrorDisplay
        error={error}
        context={context}
        onRetry={retry}
        onDismiss={retry}
        showDetails
      />
    );
  }

  return <>{children}</>;
}

// Hook for handling MCP operations with error boundaries
export function useMCPOperation<T>(
  operation: () => Promise<T>,
  context?: string
) {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<any>(null);
  const [data, setData] = useState<T | null>(null);

  const execute = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const result = await operation();
      setData(result);
      return result;
    } catch (err) {
      const mcpError = MCPErrorHandler.handleMCPError(err, context);
      setError(mcpError);
      throw mcpError;
    } finally {
      setIsLoading(false);
    }
  };

  const reset = () => {
    setError(null);
    setData(null);
    setIsLoading(false);
  };

  return {
    execute,
    isLoading,
    error,
    data,
    reset,
    formattedError: error ? MCPErrorHandler.formatUserFriendlyError(error, context) : null
  };
}