import express from "express"
import { KafkaClient, HighLevelProducer } from "kafka-node"
import { hello } from "./usecases/hello"
import { sendMsg } from "./usecases/send_msg"

export function index(req: express.Request, res: express.Response) {
  res.json(hello())
}

export function send(req: express.Request, res: express.Response) {
  console.log(req.body)
  const client = new  KafkaClient({kafkaHost: "127.0.0.1:9093"})
  const producer = new HighLevelProducer(client)

  res.json(sendMsg(producer, req.body.msg))
}
