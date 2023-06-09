use std::error::Error;

use futures::StreamExt;
use libp2p::{Swarm, swarm::{keep_alive::Behaviour, SwarmEvent}, identity, PeerId, Multiaddr};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {local_peer_id:?}");

    let transport = libp2p::development_transport(local_key).await?;

    let mut swarm = Swarm::with_async_std_executor(transport, Behaviour::default(), local_peer_id);

    swarm.listen_on("/ip4/0.0.0.0/tcp/41067".parse()?)?;

    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {addr}");
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
            _ => {},
        }
    }
}
