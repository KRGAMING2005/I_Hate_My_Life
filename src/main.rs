use reqwest;

#[tokio::main]
async fn test() {
    let result = reqwest::get("https://am.i.mullvad.net/connected").await.unwrap().text().await;
    
    println!("{:?}", result);
}
fn main() {
    test();
}
