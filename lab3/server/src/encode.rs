use tokio::process::Command;

pub async fn encode_video(input_file: &str, output_file: &str) -> anyhow::Result<()> {
    let _ = Command::new("ffmpeg")
        .args(&["-i", input_file, "-lossless", "1", output_file])
        .output()
        .await?;

    Ok(())
}
