# How to use

Here is a basic example to specify your Okapi key, and to retrieve the status from a given tracking number

```
// src/main.rs

#[tokio::main]
async fn main() {
    let client = laposte::Client::new(String::from("your_api_key"));
    let res = client.get_tracking_info(String::from("the_tracking_number"));
    match res.await {
        Ok(resp) => println!("Le colis n°{} est au statut {}", resp.shipment.id_ship, resp.shipment.event[0].label),
        Err(_) => println!("Oh no")
    };

}

```
