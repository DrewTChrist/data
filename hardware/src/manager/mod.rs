use crate::{
    device::{mock::MockDelay, mock::MockSensor, Device},
    new_device,
};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub enum Message {
    Data(f32),
}

pub struct DeviceManager {
    thread: Option<std::thread::JoinHandle<()>>,
    sender: mpsc::Sender<Message>,
    pub receiver: mpsc::Receiver<Message>,
}

impl DeviceManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            thread: None,
            sender: tx,
            receiver: rx,
        }
    }

    pub fn run(mut self) -> Self {
        let tx = self.sender.clone();
        self.thread = Some(thread::spawn(move || {
            let mut device: Device<MockSensor, f32, MockDelay> = new_device!(MockSensor);
            loop {
                //println!("{}", device.read());
                tx.send(Message::Data(device.read())).unwrap();
                thread::sleep(Duration::from_millis(5000));
            }
        }));
        self
    }
}
