use libp2p::{
    self,
    identity, ping,
    swarm::{keep_alive, NetworkBehaviour, SwarmBuilder, SwarmEvent},
    Multiaddr,
    PeerId, futures::StreamExt,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 生成ed25519密钥对；非对称密钥
    let new_key = identity::Keypair::generate_ed25519();
    // 使用密钥对的公钥生成对应的，对等层Id，使用公钥进行生成。
    let new_peer_id = PeerId::from(new_key.public());
    println!("生成的用户id{new_peer_id:?}");
    // 传输
    // 启动一个开发服务器连接，公开公钥
    // 使用development_transport构建多路复用传输，多个逻辑子流可以在同一底层（TCP）上共存连接。
    let transport = libp2p::tokio_development_transport(new_key).unwrap();

    // 网络行为
    // ping，这种行为不关心在网络上面的发送方式，是否通过TCP连接方式。无论它们是否是通过噪声加密还是仅以明文形式加密，只关心发送什么消息在网络上。
    let behaviour = Behaviour::default();

    // 群，蜂窝
    // 需要把传输和网络行为联系起来就需要群，让双方都互相连接。这份连接是一群客户端连接到一起，简而言之，Swarm同时驱动传输和NetworkBehavior向前移动，
    // 将命令从NetworkBehavior传递到传输，以及将事件从传输传递到NetworkBehavior。
    // let mut swarm =Swarm::with_threadpool_executor(transport, behaviour, new_peer_id);
    let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, new_peer_id).build();

    // 多地址
    // 随着Swarm准备好，监听传入的连接。只需要将一个地址传递给Swarm，就像std::net::TcpListener::bind一样。
    // 但是我们没有传递IP地址，而是传递了和IP类似的一个多地址，这是libp2p价值的另一个核心概念。
    // Multiaddr是一个子描述的网络地址和协议栈，它用于建立与对等方的连接。

    // 监听对应ip4协议 所有ip tcp 协议 端口号随机
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // 监听另一个地址
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        // 拨打已知的地址
        swarm.dial(remote)?;
        println!("拨打的地址为{addr}")
    }

    // 持续轮询群

    // 一切准备就是，接下来就是轮询群，允许它侦听传入的连接并建立传出连接
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr{address,..} => println!("监听的地址为{address:?}"),
            SwarmEvent::Behaviour(event) => println!("事件是{event:?}"),
            _ => {}
        }
    }

    // Ok(())

    // println!("对应生成的对等层Id {:?}", new_peer_id);
}

// 网络行为
// 传输和网络行为这俩个特征使我们能够干净俐落地发送字节与要发送的字节分开。
// 这个结构体包含了KeepAlive行为，因此可以连续进行一系列操作。
#[derive(NetworkBehaviour, Default)]
struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
}
