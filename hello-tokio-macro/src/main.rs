use hi::say;

#[say("run before async rt init")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("run inside async rt");
    Ok(())
}
