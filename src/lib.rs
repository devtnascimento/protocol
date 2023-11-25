pub mod message;

pub use serde_json;

#[cfg(test)]
mod tests {
    use super::message::*;
    #[test]
    fn register_behavior() {
        let user1 = User {
            name: "foo".to_string(),
            last_name: "bar".to_string(),
            cpf: "12312312312".to_string(),
            pix_key: "some_cool_key".to_string(),
        };

        let user2 = User {
            name: "Thiago".to_string(),
            last_name: "Nascimento".to_string(),
            cpf: "12312312311".to_string(),
            pix_key: "some_coolest_key".to_string(),
        };

        let users = vec![user1, user2];

        let pix = register::Pix {
            bank_name: "bank1".to_string(),
            users,
        };

        let serialized = serde_json::to_string(&pix).unwrap();
        println!("serialized = {}", serialized);

        let deserialized: register::Pix = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
    }

    #[test]
    fn request_behavior() {
        let user1 = User {
            name: "foo".to_string(),
            last_name: "bar".to_string(),
            cpf: "12312312312".to_string(),
            pix_key: "some_cool_key".to_string(),
        };

        let user2 = User {
            name: "Thiago".to_string(),
            last_name: "Nascimento".to_string(),
            cpf: "12312312311".to_string(),
            pix_key: "some_coolest_key".to_string(),
        };

        let transaction = request::Transaction {
            bank_name: "bank1".to_string(),
            from_user: user1,
            to_user: user2,
            amount: 10.0,
        };

        let pix = request::Pix {
            key: "some_coolest_key".to_string(),
        };

        let serialized_transaction = serde_json::to_string(&transaction).unwrap();
        println!("serialized_transaction = {}", serialized_transaction);

        let deserialized_transaction: request::Transaction =
            serde_json::from_str(&serialized_transaction).unwrap();
        println!("deserialized_transaction = {:?}", deserialized_transaction);

        let serialized_pix = serde_json::to_string(&pix).unwrap();
        println!("serialized_pix = {}", serialized_pix);

        let deserialized_pix: request::Pix = serde_json::from_str(&serialized_pix).unwrap();
        println!("deserialized_pix = {:?}", deserialized_pix);

        assert_eq!(
            serialized_transaction,
            r#"{"bank_name":"bank1","from_user":{"name":"foo","last_name":"bar","cpf":"12312312312","pix_key":"some_cool_key"},"to_user":{"name":"Thiago","last_name":"Nascimento","cpf":"12312312311","pix_key":"some_coolest_key"},"amount":10.0}"#
        );
    }

    #[test]
    fn response_behavior() {
        let user2 = User {
            name: "Thiago".to_string(),
            last_name: "Nascimento".to_string(),
            cpf: "12312312311".to_string(),
            pix_key: "some_coolest_key".to_string(),
        };

        let transaction = response::Transaction { status: Status::Ok };

        let pix = response::Pix {
            status: Status::Ok,
            user: user2,
        };

        let serialized_transaction = serde_json::to_string(&transaction).unwrap();
        println!("serialized_transaction = {}", serialized_transaction);

        let deserialized_transaction: response::Transaction =
            serde_json::from_str(&serialized_transaction).unwrap();
        println!("deserialized_transaction = {:?}", deserialized_transaction);

        let serialized_pix = serde_json::to_string(&pix).unwrap();
        println!("serialized_pix = {}", serialized_pix);

        let deserialized_pix: response::Pix = serde_json::from_str(&serialized_pix).unwrap();
        println!("deserialized_pix = {:?}", deserialized_pix);
    }
}
