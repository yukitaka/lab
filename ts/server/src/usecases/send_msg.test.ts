import { describe, expect, it } from 'vitest';
import { sendMsg } from './send_msg';
import {HighLevelProducer, KafkaClient, Producer} from "kafka-node";

describe("message", () => {
  it("send", () => {
    const client = new KafkaClient()
    const producer = new HighLevelProducer(client)
    expect(sendMsg(producer, "test")).toStrictEqual([{"messages": "test", "topic": "test-topic"}]);
  })
})
