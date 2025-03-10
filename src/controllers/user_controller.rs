use crate::service::kafkaservice;

pub fn publish_kafka_message(){
    kafkaservice::publish();
}