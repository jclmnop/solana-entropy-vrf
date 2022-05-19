use std::time::Duration;
use anchor_client::{ClientError, EventContext, EventHandle, Program};
use crate::*;
use entropy::RandomNumRequested;
use entropy::constants::DELAY;
use crossbeam::{channel, channel::Receiver, channel::Sender};
use anyhow::Error;
use anchor_client::Client;
use anchor_client::solana_client::pubsub_client::PubsubClientSubscription;
use anchor_client::solana_sdk::signature::Signature;

pub fn run_crank(url: Cluster, keypair_bytes: [u8; 64]) -> Result<()> {
    let mut operator = CrankOperator::init(url, keypair_bytes);
    operator.run()
}

struct CrankOperator {
    pub client_properties: (String, String, [u8; 64], CommitmentConfig),
    pub event_queue: Receiver<RandomNumRequested>,
    pub listener: Sender<RandomNumRequested>,
    pub current_event: Option<RandomNumRequested>,
    pub last_slot: u64
}

impl CrankOperator {
    pub fn init(
        url: Cluster,
        keypair_bytes: [u8; 64],
    ) -> Self {
        let (s, r): (
            Sender<RandomNumRequested>, Receiver<RandomNumRequested>
        ) = channel::unbounded();
        Self {
            client_properties: (
                url.url().parse().unwrap(),
                url.ws_url().parse().unwrap(),
                keypair_bytes,
                CommitmentConfig::confirmed(),
            ),
            event_queue: r,
            listener: s,
            current_event: None,
            last_slot: 0,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let s = self.listener.clone();
            let handle = self.program().on(
                move |_ctx: &EventContext, event: RandomNumRequested| {
                    info!("Event received: {:?}", event);
                    s.send(event).unwrap();
                }
            )?;

            let mut loop_count = 30;
            info!("[*] Listening...\n");
            while loop_count > 0 {
                if self.event_queue.is_empty() && self.current_event.is_none() {
                    std::thread::sleep(Duration::from_millis(1000))
                } else {
                    let current_slot = self.program().rpc().get_slot().unwrap();
                    let current_event: RandomNumRequested;
                    info!("[*] Current slot: {}", current_slot);
                    if let Some(e) = &self.current_event {
                        current_event = *e;
                    } else {
                        current_event = self.event_queue.recv().unwrap();
                        if self.last_slot > current_event.request_slot + DELAY {
                            continue
                        } else {
                            self.current_event = Some(current_event);
                        }
                    }
                    info!("[*] Current event: {:?}", current_event);

                    if current_slot >= current_event.request_slot + DELAY {
                        match self.turn_crank() {
                            Ok(s) => {
                                info!("[*] Crank turned, tx: {:?}", s);
                                self.current_event = None;
                                self.last_slot = current_slot;
                            },
                            Err(e) => {
                                debug!("[*] Error turning crank: {:?}", e);
                                std::thread::sleep(Duration::from_millis(100))
                            }
                        }
                    } else {
                        info!("Slot {} not higher than {}", current_slot, current_event.request_slot + DELAY);
                        std::thread::sleep(Duration::from_millis(500));
                    }
                }
                loop_count -= 1;
                // debug!("[*] {} loops left", loop_count);
                info!("[*] {} events queued", self.event_queue.len())
            }
            info!("[*] Cycling listener...");
            std::thread::spawn(move || {
                drop(handle);
            });
        }
    }

    fn program(&self) -> Program {
        let keypair_bytes = self.client_properties.2;
        let cluster = Cluster::Custom(
            self.client_properties.0.clone(),
            self.client_properties.1.clone()
        );
        let client = Client::new_with_options(
            cluster,
            Rc::new(Keypair::from_bytes(&keypair_bytes).unwrap()),
            self.client_properties.3
        );
        client.program(entropy::ID)
    }

    fn turn_crank(&self) -> std::result::Result<Signature, ClientError> {
        info!("[*] Turning crank...");
        let result = self.program().request().instruction(crank_ix()).send();

        debug!("[*] Result: {:?}", result);
        if result.is_ok() {
            Ok(result.unwrap())
        } else {
            Err(result.unwrap_err())
        }
    }
}
