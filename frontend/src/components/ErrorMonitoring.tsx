import { useState, useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Button } from "./ui/button";
import { Badge } from "./ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./ui/tabs";
import { AlertTriangle, Activity, Clock, TrendingDown, RefreshCw, Trash2 } from "lucide-react";
import { MCPErrorHandler, MCPError } from "../utils/mcpErrorHandler";

export function ErrorMonitoring() {
  const [errorLog, setErrorLog] = useState<MCPError[]>([]);
  const [errorStats, setErrorStats] = useState<any>(null);
  const [isRefreshing, setIsRefreshing] = useState(false);

  const refreshData = () => {
    setIsRefreshing(true);
    setErrorLog(MCPErrorHandler.getErrorLog());
    setErrorStats(MCPErrorHandler.getErrorStats());
    setTimeout(() => setIsRefreshing(false), 300);
  };

  useEffect(() => {
    refreshData();
    
    // Refresh data every 30 seconds
    const interval = setInterval(refreshData, 30000);
    return () => clearInterval(interval);
  }, []);

  const clearErrorLog = () => {
    MCPErrorHandler.clearErrorLog();
    refreshData();
  };

  const getSeverityColor = (error: MCPError) => {
    const severity = MCPErrorHandler.getErrorSeverity(error);
    switch (severity) {
      case 'critical': return 'bg-red-600/20 border-red-500/30 text-red-300';
      case 'high': return 'bg-orange-600/20 border-orange-500/30 text-orange-300';
      case 'medium': return 'bg-yellow-600/20 border-yellow-500/30 text-yellow-300';
      default: return 'bg-blue-600/20 border-blue-500/30 text-blue-300';
    }
  };

  const formatTimestamp = (timestamp: Date) => {
    return new Date(timestamp).toLocaleString();
  };

  const getErrorFrequency = () => {
    if (!errorStats || errorStats.total === 0) return [];
    
    return Object.entries(errorStats.byCode)
      .map(([code, count]) => ({ code: parseInt(code), count: count as number }))
      .sort((a, b) => b.count - a.count)
      .slice(0, 5);
  };

  return (
    <Card className="bg-black/40 backdrop-blur-lg border-white/10 text-white">
      <CardHeader>
        <CardTitle className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <AlertTriangle className="h-5 w-5" />
            Error Monitoring
          </div>
          <div className="flex gap-2">
            <Button
              size="sm"
              variant="outline"
              onClick={refreshData}
              disabled={isRefreshing}
              className="border-white/20 text-white hover:bg-white/10"
            >
              <RefreshCw className={`h-4 w-4 mr-2 ${isRefreshing ? 'animate-spin' : ''}`} />
              Refresh
            </Button>
            <Button
              size="sm"
              variant="outline"
              onClick={clearErrorLog}
              className="border-red-500/50 text-red-300 hover:bg-red-600/20"
            >
              <Trash2 className="h-4 w-4 mr-2" />
              Clear Log
            </Button>
          </div>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        <Tabs defaultValue="overview" className="w-full">
          <TabsList className="grid w-full grid-cols-3 bg-white/10">
            <TabsTrigger value="overview" className="data-[state=active]:bg-white/20">
              <Activity className="h-4 w-4 mr-2" />
              Overview
            </TabsTrigger>
            <TabsTrigger value="recent" className="data-[state=active]:bg-white/20">
              <Clock className="h-4 w-4 mr-2" />
              Recent Errors
            </TabsTrigger>
            <TabsTrigger value="analysis" className="data-[state=active]:bg-white/20">
              <TrendingDown className="h-4 w-4 mr-2" />
              Analysis
            </TabsTrigger>
          </TabsList>

          {/* Overview Tab */}
          <TabsContent value="overview" className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
              <Card className="bg-white/5 border-white/10">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-white/80">Total Errors</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold text-white">
                    {errorStats?.total || 0}
                  </div>
                  <p className="text-xs text-white/60">All time</p>
                </CardContent>
              </Card>
              
              <Card className="bg-white/5 border-white/10">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-white/80">Recent Errors</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold text-orange-400">
                    {errorStats?.recent || 0}
                  </div>
                  <p className="text-xs text-white/60">Last hour</p>
                </CardContent>
              </Card>
              
              <Card className="bg-white/5 border-white/10">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-white/80">Critical Errors</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold text-red-400">
                    {errorStats?.bySeverity?.critical || 0}
                  </div>
                  <p className="text-xs text-white/60">High priority</p>
                </CardContent>
              </Card>
              
              <Card className="bg-white/5 border-white/10">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-white/80">Error Rate</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold text-blue-400">
                    {errorStats?.recent && errorStats.recent > 0 ? 'High' : 'Low'}
                  </div>
                  <p className="text-xs text-white/60">Current trend</p>
                </CardContent>
              </Card>
            </div>

            {errorStats && (
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div className="space-y-4">
                  <h3 className="text-lg">Error Severity Distribution</h3>
                  <div className="space-y-2">
                    {Object.entries(errorStats.bySeverity || {}).map(([severity, count]) => (
                      <div key={severity} className="flex items-center justify-between p-2 bg-white/5 rounded">
                        <span className="capitalize text-sm">{severity}</span>
                        <div className="flex items-center gap-2">
                          <div className="w-16 bg-white/10 rounded-full h-2">
                            <div 
                              className={`h-2 rounded-full ${
                                severity === 'critical' ? 'bg-red-500' :
                                severity === 'high' ? 'bg-orange-500' :
                                severity === 'medium' ? 'bg-yellow-500' : 'bg-blue-500'
                              }`}
                              style={{width: `${Math.min(100, (count as number) / Math.max(1, errorStats.total) * 100)}%`}}
                            ></div>
                          </div>
                          <span className="text-sm font-mono w-8 text-right">{count as number}</span>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>

                <div className="space-y-4">
                  <h3 className="text-lg">Most Frequent Error Codes</h3>
                  <div className="space-y-2">
                    {getErrorFrequency().map(({ code, count }) => (
                      <div key={code} className="flex items-center justify-between p-2 bg-white/5 rounded">
                        <div className="flex items-center gap-2">
                          <Badge variant="outline" className="font-mono">
                            {code === -1 ? 'UNKNOWN' : code}
                          </Badge>
                          <span className="text-sm text-white/70">
                            {code === -1 ? 'Unknown Error' :
                             code === 404 ? 'Not Found' :
                             code === 500 ? 'Server Error' :
                             code === 401 ? 'Unauthorized' :
                             code === 403 ? 'Forbidden' : 
                             `Error ${code}`}
                          </span>
                        </div>
                        <div className="flex items-center gap-2">
                          <div className="w-12 bg-white/10 rounded-full h-2">
                            <div 
                              className="bg-blue-500 h-2 rounded-full"
                              style={{width: `${Math.min(100, count / Math.max(1, errorStats.total) * 100)}%`}}
                            ></div>
                          </div>
                          <span className="text-sm font-mono w-6 text-right">{count}</span>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            )}
          </TabsContent>

          {/* Recent Errors Tab */}
          <TabsContent value="recent" className="space-y-4">
            <div className="space-y-3">
              <h3 className="text-lg">Recent Error Log</h3>
              
              {errorLog.length === 0 ? (
                <div className="bg-green-600/10 border border-green-500/30 rounded-lg p-6 text-center">
                  <div className="text-green-400 mb-2">No errors recorded</div>
                  <p className="text-white/60 text-sm">Your MCP integration is running smoothly!</p>
                </div>
              ) : (
                <div className="space-y-2 max-h-96 overflow-auto">
                  {errorLog.slice().reverse().map((error, index) => (
                    <div key={index} className={`border rounded-lg p-3 ${getSeverityColor(error)}`}>
                      <div className="flex items-start justify-between">
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center gap-2 mb-1">
                            <Badge variant="outline" className="font-mono text-xs">
                              {error.code === -1 ? 'UNKNOWN' : error.code}
                            </Badge>
                            {error.context && (
                              <Badge variant="outline" className="text-xs">
                                {error.context}
                              </Badge>
                            )}
                          </div>
                          <p className="text-sm text-white/90 mb-1">{error.message}</p>
                          <p className="text-xs text-white/60">{formatTimestamp(error.timestamp)}</p>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </TabsContent>

          {/* Analysis Tab */}
          <TabsContent value="analysis" className="space-y-4">
            <div className="space-y-4">
              <h3 className="text-lg">Error Analysis & Recommendations</h3>
              
              {errorStats?.total === 0 ? (
                <div className="bg-blue-600/10 border border-blue-500/30 rounded-lg p-6 text-center">
                  <div className="text-blue-400 mb-2">No error data available</div>
                  <p className="text-white/60 text-sm">Start using MCP features to see error analysis here</p>
                </div>
              ) : (
                <div className="space-y-4">
                  <div className="bg-white/5 rounded-lg p-4">
                    <h4 className="text-md font-medium mb-3">System Health Assessment</h4>
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                      <div className="text-center">
                        <div className={`text-2xl font-bold ${
                          errorStats.recent > 10 ? 'text-red-400' :
                          errorStats.recent > 5 ? 'text-yellow-400' : 'text-green-400'
                        }`}>
                          {errorStats.recent > 10 ? 'Poor' :
                           errorStats.recent > 5 ? 'Fair' : 'Good'}
                        </div>
                        <p className="text-xs text-white/60">Overall Health</p>
                      </div>
                      
                      <div className="text-center">
                        <div className={`text-2xl font-bold ${
                          (errorStats.bySeverity?.critical || 0) > 0 ? 'text-red-400' :
                          (errorStats.bySeverity?.high || 0) > 5 ? 'text-yellow-400' : 'text-green-400'
                        }`}>
                          {(errorStats.bySeverity?.critical || 0) > 0 ? 'High' :
                           (errorStats.bySeverity?.high || 0) > 5 ? 'Medium' : 'Low'}
                        </div>
                        <p className="text-xs text-white/60">Risk Level</p>
                      </div>
                      
                      <div className="text-center">
                        <div className="text-2xl font-bold text-blue-400">
                          {errorStats.recent === 0 ? 'Stable' : 'Active'}
                        </div>
                        <p className="text-xs text-white/60">Status</p>
                      </div>
                    </div>
                  </div>

                  <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
                    <div className="bg-orange-600/10 border border-orange-500/30 rounded-lg p-4">
                      <h4 className="text-orange-300 text-sm font-medium mb-2">Common Issues</h4>
                      <ul className="text-xs text-white/70 space-y-1">
                        {errorStats.byCode[404] && <li>• MCP server endpoint not found</li>}
                        {errorStats.byCode[500] && <li>• Internal server errors detected</li>}
                        {errorStats.byCode[401] && <li>• Authentication failures</li>}
                        {errorStats.byCode[-1] && <li>• Network connectivity issues</li>}
                        {errorStats.recent > 5 && <li>• High error rate in recent period</li>}
                        {!Object.keys(errorStats.byCode).length && <li>• No specific patterns detected</li>}
                      </ul>
                    </div>

                    <div className="bg-green-600/10 border border-green-500/30 rounded-lg p-4">
                      <h4 className="text-green-300 text-sm font-medium mb-2">Recommendations</h4>
                      <ul className="text-xs text-white/70 space-y-1">
                        {errorStats.recent > 10 && <li>• Check MCP server status and connectivity</li>}
                        {errorStats.bySeverity?.critical > 0 && <li>• Address critical errors immediately</li>}
                        {errorStats.byCode[401] && <li>• Verify authentication configuration</li>}
                        {errorStats.byCode[404] && <li>• Review MCP server endpoint URLs</li>}
                        {errorStats.byCode[500] && <li>• Check server logs for issues</li>}
                        <li>• Monitor error trends regularly</li>
                        <li>• Implement retry logic for transient failures</li>
                      </ul>
                    </div>
                  </div>
                </div>
              )}
            </div>
          </TabsContent>
        </Tabs>
      </CardContent>
    </Card>
  );
}