#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg: u32 = std::env::args().nth(1).unwrap().parse()?;

    let client = nhentai::Client::new(None, None);

    let gallery = client.gallery(arg).await?;

    println!("{gallery:#?}");

    Ok(())
}
