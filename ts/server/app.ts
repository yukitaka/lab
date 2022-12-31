import * as http from "http";
import { port } from "./config";
import { KafkaClient, HighLevelProducer } from "kafka-node";

const server = http.createServer(
  (request, response) => {
    const client = new  KafkaClient({kafkaHost: "127.0.0.1:9093"});
    const producer = new HighLevelProducer(client);
    const payloads = [
      { topic: 'test-topic', messages: ['hello', 'world'] }
    ];
    producer.on("ready", () => {
      producer.send(payloads, function (err: any, data: any) {
        console.log(err);
        console.log(data);
      });
    });
    response.end("Hello! world");
  }
);


server.listen(port);

console.log(`http://localhost:${port}`);
