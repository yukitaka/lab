# Typscript Server with Apache Kafka

## Run
```
docker compose up -d
npm run dev
```

## Kafka
### Show Topics
```
docker run -it --rm --network server_app-tier \
    -e KAFKA_CFG_ZOOKEEPER_CONNECT=server-zookeeper-1:2181 \
    bitnami/kafka:latest kafka-topics.sh --list  --bootstrap-server server-kafka-1:9092
```

### Create Topic
```
docker run -it --rm --network server_app-tier \
    -e KAFKA_CFG_ZOOKEEPER_CONNECT=server-zookeeper-1:2181 \
    bitnami/kafka:latest kafka-topics.sh --create --topic test-topic --bootstrap-server server-kafka-1:9092
```

### Send Message
```
docker run -it --rm --network server_app-tier \
    -e KAFKA_CFG_ZOOKEEPER_CONNECT=server-zookeeper-1:2181 \
    bitnami/kafka:latest kafka-console-producer.sh --topic test-topic --broker-list=server-kafka-1:9092
```

### Receive Message
```
docker run -it --rm --network server_app-tier \
    -e KAFKA_CFG_ZOOKEEPER_CONNECT=server-zookeeper-1:2181 \
    bitnami/kafka:latest kafka-console-consumer.sh --topic test-topic --from-beginning --bootstrap-server=server-kafka-1:9092
```

## Schema Registry
Using httpie
```
http POST http://localhost:8081/subjects/test-topic-value/versions \
Content-Type:application/vnd.schemaregistry.v1+json \
schema:='"{\"type\": \"record\", \"name\": \"test\", \"fields\": [{\"name\":\"message\", \"type\": \"string\"}]}"'
```