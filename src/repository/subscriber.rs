use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

// Singleton of Database
lazy_static!{
    static ref SUBSCRIBERS: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository{
    pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
        let subscriber_value = subscriber.clone();
        if SUBSCRIBERS.get(product_type).is_none(){
            SUBSCRIBERS.insert(String::from(product_type), DashMap::new());

        };

        SUBSCRIBERS.get(product_type).unwrap()
            .insert(subscriber_value.url.clone(), subscriber_value);
        return subscriber;
    }

    pub fn list_all(product_type: &str) -> Vec<Subscriber> {
        if SUBSCRIBERS.get(product_type).is_none(){
            SUBSCRIBERS.insert(String::from(product_type), DashMap::new());
        };

        return SUBSCRIBERS.get(product_type).unwrap().iter()
            .map(|f| f.value().clone()).collect();
    }

    pub fn delete(product_type: &str, url: &str) -> Option<Subscriber>{
        // membuat product_type baru apabila belum ada product_type tersebut
        if SUBSCRIBERS.get(product_type).is_none() {
            SUBSCRIBERS.insert(String::from(product_type), DashMap::new());
        }

        // Menghapus subscriber dengan product_type & url
        let result = SUBSCRIBERS.get(product_type).unwrap().remove(url);
        if !result.is_none(){
            return Some(result.unwrap().1);
        }
        return None;
    }
}