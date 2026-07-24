/// 비동기 애플리케이션 데몬 구동 및 생명주기 관리
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(error) = sensor_studio_core::run().await {
        eprintln!("Core execution failed: {error}");
        std::process::exit(1);
    }

    Ok(())
}
