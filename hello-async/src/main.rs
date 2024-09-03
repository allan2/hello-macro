use hi::say;

#[say("run before async rt init")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            println!("run inside async rt");
        });
    Ok(())
}
