use hi::say;

#[say("run before async rt init")]
#[async_std::main]
async fn main() {
    println!("run inside async rt");
}
