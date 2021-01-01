mod hello;
use hello::{HelloReply, HelloRequest};

mod hello_grpc;
use hello_grpc::{create_greeter, Greeter, GreeterClient};

use grpcio::{ChannelBuilder, Environment, ResourceQuota, RpcContext, ServerBuilder, UnarySink};

use std::sync::Arc;
use std::io;

use smol::prelude::*;
use futures::prelude::*;
use async_channel;
use num_cpus;

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext<'_>, req: HelloRequest, sink: UnarySink<HelloReply>) {
        let msg = format!("Ahoi, {}", req.get_name());
        let mut resp = HelloReply::default();
        resp.set_message(msg);
        resp.set_num(num_cpus::get() as u32);

        let f = sink.success(resp).unwrap_or_else(|_| ());
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = create_greeter(GreeterService);

    let quota = ResourceQuota::new(Some("HelloServerQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("[::1]", 50052)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    server.start();

    for (host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }

    // Set a handler that sends a message through a channel.
    let (s, ctrl_c) = async_channel::bounded(100);
    let handle = move || {
        s.try_send(()).ok();
    };
    ctrlc::set_handler(handle).unwrap();

    smol::block_on(async {
        println!("Waiting for Ctrl-C...");

        // Receive a message that indicates the Ctrl-C signal occurred.
        ctrl_c.recv().await.ok();

        println!("Done!");
    })
}



// use grpcio::EnvBuilder;

// fn main() {
//     let env = Arc::new(EnvBuilder::new().build());
//     let ch = ChannelBuilder::new(env).connect("localhost:50051");
//     let client = GreeterClient::new(ch);

//     let mut req = HelloRequest::default();
//     req.set_name("ASD".to_owned());
//     let reply = client.say_hello(&req).expect("rpc");
//     println!("Greeter received: {}", reply.get_message());
// }