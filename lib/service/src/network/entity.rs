use std::sync::{Arc, OnceLock};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::mpsc,
};
use tokio_util::sync::CancellationToken;
use tracing::error;

use super::{
    NetworkEventListener,
    packet::{DecodeError, NetPacket},
};

pub struct NetworkEntity {
    sender: mpsc::UnboundedSender<NetPacket>,
    encryption_state: Arc<EncryptionState>,
    cancellation: CancellationToken,
}

struct EncryptionState {
    initial_xorpad: Option<&'static [u8; 4096]>,
    session_xorpad: OnceLock<[u8; 4096]>,
}

impl EncryptionState {
    pub fn xor(&self, buffer: &mut [u8]) {
        let xorpad = match (self.initial_xorpad, self.session_xorpad.get()) {
            (None, None) => return,
            (Some(xorpad), None) => xorpad,
            (_, Some(xorpad)) => xorpad,
        };

        buffer
            .iter_mut()
            .zip(xorpad.iter().cycle())
            .for_each(|(a, b)| *a ^= b)
    }
}

impl NetworkEntity {
    pub fn start(
        id: u64,
        stream: TcpStream,
        listener: Arc<dyn NetworkEventListener>,
        xorpad: Option<&'static [u8; 4096]>,
    ) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let (r, w) = stream.into_split();

        let encryption_state = Arc::new(EncryptionState {
            initial_xorpad: xorpad,
            session_xorpad: OnceLock::new(),
        });

        let cancellation = CancellationToken::new();

        tokio::spawn(Self::receive_loop(
            r,
            cancellation.clone(),
            id,
            listener,
            Arc::clone(&encryption_state),
        ));

        tokio::spawn(Self::send_loop(w, rx));

        Self {
            sender: tx,
            encryption_state,
            cancellation,
        }
    }

    pub fn set_session_key(&self, xorpad: [u8; 4096]) {
        let _ = self.encryption_state.session_xorpad.set(xorpad);
    }

    pub fn send(&self, mut packet: NetPacket) {
        self.encryption_state.xor(&mut packet.body);
        let _ = self.sender.send(packet);
    }

    pub fn disconnect(&self) {
        self.cancellation.cancel();
    }

    async fn receive_loop(
        mut r: OwnedReadHalf,
        cancellation: CancellationToken,
        id: u64,
        listener: Arc<dyn NetworkEventListener>,
        encryption_state: Arc<EncryptionState>,
    ) {
        let mut receive_buffer = vec![0u8; 16384];
        let mut recv_index = 0;

        loop {
            tokio::select! {
                result = r.read(&mut receive_buffer[recv_index..]) => {
                    let Ok(r @ 1..) = result else {
                        break;
                    };

                    recv_index += r;

                    if loop {
                        match NetPacket::decode(&receive_buffer[..recv_index]) {
                            Ok((mut packet, nread)) => {
                                receive_buffer.copy_within(nread..recv_index, 0);
                                recv_index -= nread;

                                encryption_state.xor(&mut packet.body);
                                listener.on_receive(id, packet);
                            }
                            Err(DecodeError::Incomplete(_, _)) => break Ok(()),
                            Err(err) => {
                                error!("failed to decode incoming packet: {err}");
                                break Err(());
                            }
                        }
                    }
                    .is_err()
                    {
                        break;
                    }
                },
                _ = cancellation.cancelled() => break,
            }
        }

        listener.on_disconnect(id);
    }

    async fn send_loop(mut w: OwnedWriteHalf, mut rx: mpsc::UnboundedReceiver<NetPacket>) {
        while let Some(packet) = rx.recv().await {
            if w.write_all(&Vec::from(packet)).await.is_err() {
                break;
            }
        }
    }
}
