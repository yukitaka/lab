import express from "express"
import { KafkaClient, HighLevelProducer } from "kafka-node"
import { hello } from "./usecases/hello"

export function index(req: express.Request, res: express.Response) {
  res.json(hello())
}

export function send(req: express.Request, res: express.Response) {
  console.log(req.body)
  const payloads = [
    { topic: 'test-topic', messages: req.body.msg }
  ];
  const client = new  KafkaClient({kafkaHost: "127.0.0.1:9093"})
  const producer = new HighLevelProducer(client)
  producer.on("ready", () => {
    producer.send(payloads, function (err: any, data: any) {
      console.log(err)
      console.log(data)
    })
  })

  res.send(JSON.stringify(payloads))
}
