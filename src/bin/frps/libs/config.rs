use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};


pub struct Configure {
    pub frp_host: String,
    pub frp_port: u16,
    pub frp_token: String
}

impl Configure {
    pub fn get(_args: Vec<String>) -> Self {
        let frp_host: String = String::from("0.0.0.0");
        let frp_port: u16 = 7100;
        let frp_token: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        Configure {
            frp_host,
            frp_port,
            frp_token,
        }
    }
}
