use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let now = Instant::now();

    for i in 1..10000 {
        tokio::spawn(async move {
            println!("{}", i);
            reqwest::get("http://localhost:8080").await
        });
    }

    let res = reqwest::get("http://localhost:8080").await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let elapsed = now.elapsed();
    println!("Time elapsed: {} ms, {} s", elapsed.as_micros(), elapsed.as_secs_f32());

    Ok(())
}