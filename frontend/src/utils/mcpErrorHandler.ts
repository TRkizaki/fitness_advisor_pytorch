// MCP Error Handling Utilities
export interface MCPError {
  code: number;
  message: string;
  details?: any;
  timestamp: Date;
  context?: string;
}

export class MCPErrorHandler {
  private static errorLog: MCPError[] = [];

  static handleMCPError(error: any, context?: string): MCPError {
    const mcpError: MCPError = {
      code: error?.code || -1,
      message: error?.message || error?.toString() || 'Unknown MCP error',
      details: error,
      timestamp: new Date(),
      context: context
    };

    // Log error
    this.errorLog.push(mcpError);
    console.error('MCP Error:', mcpError);

    // Keep only last 100 errors
    if (this.errorLog.length > 100) {
      this.errorLog = this.errorLog.slice(-100);
    }

    return mcpError;
  }

  static getErrorMessage(error: any): string {
    if (typeof error === 'string') return error;
    
    // Handle MCP-specific error formats
    if (error?.error?.message) return error.error.message;
    if (error?.message) return error.message;
    
    // Handle network errors
    if (error?.name === 'NetworkError') return 'Network connection failed. Please check your internet connection.';
    if (error?.name === 'TimeoutError') return 'Request timed out. The MCP server may be unresponsive.';
    
    // Handle WebSocket errors
    if (error?.type === 'close') return 'WebSocket connection closed unexpectedly.';
    if (error?.type === 'error') return 'WebSocket connection error occurred.';
    
    // Handle HTTP errors
    if (error?.status) {
      switch (error.status) {
        case 401: return 'Authentication failed. Please check your credentials.';
        case 403: return 'Access denied. You may not have permission for this operation.';
        case 404: return 'MCP server not found. Please check the server configuration.';
        case 500: return 'MCP server internal error. Please try again later.';
        case 503: return 'MCP server temporarily unavailable. Please try again later.';
        default: return `HTTP error ${error.status}: ${error.statusText || 'Unknown error'}`;
      }
    }
    
    return 'An unexpected error occurred. Please try again.';
  }

  static getErrorSuggestions(error: any): string[] {
    const message = this.getErrorMessage(error).toLowerCase();
    const suggestions: string[] = [];

    if (message.includes('network') || message.includes('connection')) {
      suggestions.push('Check your internet connection');
      suggestions.push('Verify the MCP server is running');
      suggestions.push('Check firewall settings');
    }

    if (message.includes('timeout')) {
      suggestions.push('Try again with a longer timeout');
      suggestions.push('Check server performance');
      suggestions.push('Verify network stability');
    }

    if (message.includes('authentication') || message.includes('401')) {
      suggestions.push('Verify your API credentials');
      suggestions.push('Check if your session has expired');
      suggestions.push('Ensure proper authentication headers');
    }

    if (message.includes('permission') || message.includes('403')) {
      suggestions.push('Check your account permissions');
      suggestions.push('Contact your administrator');
      suggestions.push('Verify API key scopes');
    }

    if (message.includes('not found') || message.includes('404')) {
      suggestions.push('Verify the MCP server URL');
      suggestions.push('Check server configuration');
      suggestions.push('Ensure the endpoint exists');
    }

    if (message.includes('server error') || message.includes('500')) {
      suggestions.push('Wait a moment and try again');
      suggestions.push('Check server logs');
      suggestions.push('Contact technical support if issue persists');
    }

    if (suggestions.length === 0) {
      suggestions.push('Try refreshing the page');
      suggestions.push('Check browser console for more details');
      suggestions.push('Contact support if the problem continues');
    }

    return suggestions;
  }

  static getErrorSeverity(error: any): 'low' | 'medium' | 'high' | 'critical' {
    const message = this.getErrorMessage(error).toLowerCase();

    if (message.includes('critical') || message.includes('fatal')) {
      return 'critical';
    }

    if (message.includes('authentication') || message.includes('permission') || 
        message.includes('403') || message.includes('401')) {
      return 'high';
    }

    if (message.includes('server error') || message.includes('500') || 
        message.includes('timeout')) {
      return 'medium';
    }

    return 'low';
  }

  static formatUserFriendlyError(error: any, context?: string): {
    message: string;
    suggestions: string[];
    severity: 'low' | 'medium' | 'high' | 'critical';
    canRetry: boolean;
  } {
    const message = this.getErrorMessage(error);
    const suggestions = this.getErrorSuggestions(error);
    const severity = this.getErrorSeverity(error);
    
    // Determine if the error is retriable
    const canRetry = !message.toLowerCase().includes('permission') && 
                    !message.toLowerCase().includes('authentication') &&
                    !message.toLowerCase().includes('403') &&
                    !message.toLowerCase().includes('401');

    return {
      message,
      suggestions,
      severity,
      canRetry
    };
  }

  static getErrorLog(): MCPError[] {
    return [...this.errorLog];
  }

  static clearErrorLog(): void {
    this.errorLog = [];
  }

  static getErrorStats(): {
    total: number;
    byCode: Record<number, number>;
    bySeverity: Record<string, number>;
    recent: number; // last hour
  } {
    const now = new Date();
    const oneHourAgo = new Date(now.getTime() - 60 * 60 * 1000);

    const byCode: Record<number, number> = {};
    const bySeverity: Record<string, number> = {};
    let recent = 0;

    for (const error of this.errorLog) {
      byCode[error.code] = (byCode[error.code] || 0) + 1;
      
      const severity = this.getErrorSeverity(error);
      bySeverity[severity] = (bySeverity[severity] || 0) + 1;

      if (error.timestamp > oneHourAgo) {
        recent++;
      }
    }

    return {
      total: this.errorLog.length,
      byCode,
      bySeverity,
      recent
    };
  }
}

// React hook for MCP error handling
export function useMCPErrorHandler() {
  return {
    handleError: MCPErrorHandler.handleMCPError,
    formatError: MCPErrorHandler.formatUserFriendlyError,
    getErrorMessage: MCPErrorHandler.getErrorMessage,
    getErrorSuggestions: MCPErrorHandler.getErrorSuggestions,
    getErrorLog: MCPErrorHandler.getErrorLog,
    clearErrorLog: MCPErrorHandler.clearErrorLog,
    getErrorStats: MCPErrorHandler.getErrorStats
  };
}