## Notes: 
- [ ] Kafka Bridge component: allows to interact with Kafka brokers using HTTP REST API (without the need to use Kafka native protocol).
- [ ] Kafka Connect component: extenal system interactions
- [x] run Kafka in KRaft mode with minimum 3 node quorum for controllers (metadata) and brokers (message data)
  - KRaft limitations https://strimzi.io/docs/operators/latest/deploying#assembly-kraft-mode-str
- Support webhook feed to Kafka topics: 
  - [ ] Simple approach - maintain a simple server to listen for webhook data to translate it to a topic using Kafka client. 

# Resources: 
- https://strimzi.io/quickstarts/
- Kafka Strimzi overview https://strimzi.io/docs/operators/latest/overview
- https://github.com/strimzi/strimzi-kafka-operator/tree/main/examples/kafka/kraft
- https://kafka.apache.org/documentation/

# yaml config
- reference https://strimzi.io/docs/operators/latest/configuring
- config example https://strimzi.io/docs/operators/latest/deploying#config-examples-str
- config example https://github.com/strimzi/strimzi-kafka-operator/tree/main/examples
- config reference - managing the operator with CRDs https://strimzi.io/docs/operators/latest/deploying 
- storage config https://strimzi.io/docs/operators/latest/deploying#assembly-storage-str