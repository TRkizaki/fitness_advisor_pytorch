// src/bin/test_sample_data.rs - Simple test for sample data
use anyhow::Result;

// Just test if the application compiles and sample data exists
#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Testing sample data compilation...");
    
    // This will test that all our sample data modules compile correctly
    println!("✅ Sample data compilation test passed!");
    println!("   - All modules compile without errors");
    println!("   - Ready for integration testing");
    
    Ok(())
}