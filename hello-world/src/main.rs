extern crate yaml_rust;
pub mod server_conf;
use async_trait::async_trait;
use pingora::prelude::*;
use std::sync::Arc;
use yaml_rust::{YamlLoader, YamlEmitter};

fn main() {
        let s =
"
foo:
    - list1
    - list2
bar:
    - 1
    - 2.0
";
    let docs = YamlLoader::load_from_str(s).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Debug support
    println!("{:?}", doc);

    // Index access for map & array
    assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
    assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

    // Chained key/array access is checked and won't panic,
    // return BadValue if they are not exist.
    assert!(doc["INVALID_KEY"][100].is_badvalue());

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
}

fn start_server() {
    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();

    let upstreams =
        LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443"]).unwrap();

    let mut lb = http_proxy_service(&my_server.configuration, LB(Arc::new(upstreams)));
        lb.add_tcp("0.0.0.0:6188");

    my_server.add_service(lb);

    my_server.run_forever();
}

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> () {
        ()
    }

    async fn upstream_request_filter(
            &self,
            _session: &mut Session,
            _upstream_request: &mut RequestHeader,
            _ctx: &mut Self::CTX,
        ) -> Result<()> {
       _upstream_request.insert_header("Host", "one.one.one.one").unwrap();
       Ok(())
    }

    async fn upstream_peer(
            &self,
            session: &mut Session,
            ctx: &mut Self::CTX,
        ) -> Result<Box<HttpPeer>> {
       let upstream = self.0
           .select(b"", 256)
           .unwrap();

       println!("upstream peer is: {upstream:?}");

       let peer = Box::new(HttpPeer::new(upstream, true, "one.one.one.one".to_string()));
       Ok(peer)
    }
}
