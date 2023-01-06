import express from "express"
import KafkaAvro from "kafka-node-avro"
import { hello } from "./usecases/hello"
import { sendMsg } from "./usecases/send_msg"

export function index(req: express.Request, res: express.Response) {
  res.json(hello())
}

export function send(req: express.Request, res: express.Response) {
  const settings = {
    "kafka": {
      "kafkaHost": "127.0.0.1:9093"
    },
    "schema": {
      "registry": "http://127.0.0.1:8081"
    }
  }
  const avro = KafkaAvro.init(settings)

  res.json(sendMsg(avro, req.body.msg))
}
