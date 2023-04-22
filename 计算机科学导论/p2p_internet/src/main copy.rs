use libp2p::{
    self,
    core::transport,
    futures::StreamExt,
    identity,
    swarm::{DummyBehaviour, Swarm, SwarmBuilder, SwarmEvent},
    PeerId,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 生成ed25519密钥对；非对称密钥
    let new_key = identity::Keypair::generate_ed25519();
    // 使用密钥对的公钥生成对应的，对等层Id，使用公钥进行生成。
    let new_peer_id = PeerId::from(new_key.public());

    let behavour = DummyBehaviour::default();
    // 启动一个开发服务器连接，公开公钥
    let transport = libp2p::development_transport(new_key).await.unwrap();

    // 创建节点
    let mut swarm = Swarm::new(transport, behavour, new_peer_id);
    // 监听节点
    swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .unwrap();

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("监听地址为{:?}", address)
            },
            _ => {
                println!("其它模式")
            }
        };
    };

    println!("对应生成的对等层Id {:?}", new_peer_id);
}
