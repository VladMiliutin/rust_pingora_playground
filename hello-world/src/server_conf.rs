pub mod server_conf {
    use std::{path::PathBuf, fs};

    use yaml_rust::{YamlLoader, YamlEmitter, Yaml};


    const SERVER_CONF_FILE_LOCATION: &str = "./server_conf.yaml";

    struct Route {
        backend: String,
        add_headers: Vec<(String, String)>,
        proxy_headers: Vec<(String, String)>,
    }

    struct Backend {
        host: String,
    }

    struct ServerConf {
        routes: Vec<Route>,
        backend: Vec<Backend>,
    }

    fn load_server_conf() -> ServerConf {
        let path = PathBuf::from(SERVER_CONF_FILE_LOCATION);
        let content = fs::read_to_string(path).unwrap();

        let docs = YamlLoader::load_from_str(&content).unwrap();
        let doc: &Yaml = &docs[0];
        let doc_routes: &Vec<Yaml> = doc["routes"].as_vec().unwrap();
       
        ServerConf {
            backend: Vec::new(),
            routes: read_routes(doc_routes),
        }
    }

    fn read_routes(doc_routes: &Vec<Yaml>) -> Vec<Route> {
        let mut routes: Vec<Route> = Vec::new();

        for route in doc_routes.iter() {
            let backend = route["backend"].as_str().unwrap(); 
            let mut headers: Vec<(String, String)> = Vec::new();
            let mut proxy_headers: Vec<(String, String)> = Vec::new();

            let add_header_iter = route["add_header"].as_hash().unwrap().iter();
            add_header_iter.for_each(|entry| 
                                     headers.push((entry.0.as_str().unwrap().to_string(), entry.1.as_str().unwrap().to_string())));
            let proxy_headers_iter = route["proxy_headers"].as_hash().unwrap().iter();
            proxy_headers_iter.for_each(|entry| 
                                        proxy_headers.push((
                                            entry.0.as_str().unwrap().to_string(),
                                            entry.1.as_str().unwrap().to_string()
                                        )));
            routes.push(Route {
                    backend: backend.to_string(),
                    add_headers: headers,
                    proxy_headers: proxy_headers,
                })
        }
        routes
    }

    fn read_backends(doc_backend: &Vec<Yaml>) -> Vec<Backend> {
        let mut backend_vec: Vec<Backend> = Vec::new();

        for backend in doc_backend.iter() {
            let host = backend["host"].as_str().unwrap(); 
            backend_vec.push(Backend {
                host: host.to_string(),
            })
        }
        backend_vec
    }
}
