import express from "express"
import { KafkaClient, HighLevelProducer } from "kafka-node";

export function index(req: express.Request, res: express.Response) {
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

    res.send(JSON.stringify(payloads))
}
