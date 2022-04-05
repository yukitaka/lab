fn main() {
    let nc = nats::Options::new()
        .with_name("nats example")
        .connect("127.0.0.1");
    println!("{:?}", &nc);

    if let Ok(nc) = nc {
        if let Err(err) = nc.publish("example", "message") {
            println!("{}", err);
        }
        if let Ok(sub) = nc.subscribe("example") {
            for msg in sub.messages() {
                println!("Received a {:?}", msg);
            }
        }
    }
}
