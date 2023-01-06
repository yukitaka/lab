import {Kafka} from "kafka-node-avro";

export function sendMsg(avro: Promise<Kafka>, msg: string): { topic: string; messages: { msg: string }; key: string } {
  const payload = { topic: 'test-topic', key: "test", messages: { msg: msg } }

  avro.then(kafka => {
    kafka.send(payload).then(success => {
      console.log(success)
    }, error => {
      console.log(error)
    })
  })

  return payload
}
