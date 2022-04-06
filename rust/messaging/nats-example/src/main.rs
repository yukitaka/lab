use std::time::Duration;

fn main() {
    let nc = nats::Options::with_token("test-token")
        .with_name("nats example")
        .connect("127.0.0.1");
    println!("{:?}", &nc);

    if let Ok(nc) = nc {
        let sub = nc.subscribe("example").unwrap();

        nc.publish("example", "message").ok();

        let msg = sub.next_timeout(Duration::from_secs(1)).unwrap();
        println!(
            "Received a {:?} {:?}",
            &msg,
            String::from_utf8(msg.data.clone())
        );
    }
}
