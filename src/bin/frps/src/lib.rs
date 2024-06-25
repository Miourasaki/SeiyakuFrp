pub mod config {
    pub struct Configure {
        pub host: String,
        pub port: u16,
    }

    impl Configure {
        pub fn get(args: Vec<String>) -> Self {
            let mut host: String = String::from("0.0.0.0");
            let mut port: u16 = 7100;

            for (i, arg) in args.iter().enumerate() {
                match arg.as_str() {
                    "-h" => host = args[i+1].clone(),
                    "-p" => port = args[i + 1].clone().parse().unwrap_or(7100),
                    _ => ()
                }
            }


            Configure {
                host,
                port,
            }
        }
    }
}


pub mod server;