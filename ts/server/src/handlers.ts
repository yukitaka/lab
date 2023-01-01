import express from "express"
import { KafkaClient, HighLevelProducer } from "kafka-node";

export function index(req: express.Request, res: express.Response) {
    res.send(JSON.stringify({"msg": "Hello World!"}))
}

export function send(req: express.Request, res: express.Response) {
  console.log(req.body)
  const payloads = [
    { topic: 'test-topic', messages: req.body.msg }
  ];
  const client = new  KafkaClient({kafkaHost: "127.0.0.1:9093"});
  const producer = new HighLevelProducer(client);
  producer.on("ready", () => {
    producer.send(payloads, function (err: any, data: any) {
      console.log(err);
      console.log(data);
    });
  });

  res.send(JSON.stringify(payloads))
}
