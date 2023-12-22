use futures::channel::oneshot::Sender;

pub struct PacketLock {
    pub release_sender: Option<Sender<()>>,
}

impl Drop for PacketLock {
    fn drop(&mut self) {
        if let Some(sender) = self.release_sender.take() {
            let _ = sender.send(());
        }
    }
}
