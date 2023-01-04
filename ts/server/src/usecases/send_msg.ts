import {Producer} from "kafka-node";

export function sendMsg(producer: Producer, msg: string): { topic: string; messages: any }[] {
  const payloads = [
    { topic: 'test-topic', messages: msg }
  ];
  producer.on("ready", () => {
    producer.send(payloads, function (err: any, data: any) {
      console.log(err)
      console.log(data)
    })
  })

  return payloads
}
