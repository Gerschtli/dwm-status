use communication;
use error::*;
use std::sync::mpsc;
use wrapper::dbus;
use wrapper::thread;

const ERROR_NAME: &str = "resume watcher";
const INTERFACE_LOGIN1: &str = "org.freedesktop.login1.Manager";
const MEMBER_PREPARE_FOR_SLEEP: &str = "PrepareForSleep";
const PATH_LOGIN1: &str = "/org/freedesktop/login1";

pub(super) fn init_resume_notifier(tx: &mpsc::Sender<communication::Message>) -> Result<()> {
    let notifier = Notifier { tx: tx.clone() };

    thread::Thread::new(ERROR_NAME, notifier).run()
}

struct Notifier {
    tx: mpsc::Sender<communication::Message>,
}

impl thread::Runnable for Notifier {
    fn run(&self) -> Result<()> {
        let connection = dbus::Connection::new()?;

        connection.add_match(dbus::Match {
            interface: INTERFACE_LOGIN1,
            member: Some(MEMBER_PREPARE_FOR_SLEEP),
            path: PATH_LOGIN1,
        })?;

        connection.listen_for_signals(|signal| {
            // return value is true if going to sleep, false if waking up
            if signal.is_interface(INTERFACE_LOGIN1)? && !signal.return_value::<bool>()? {
                self.tx
                    .send(communication::Message::UpdateAll)
                    .wrap_error(ERROR_NAME, "send update failed")?
            }

            Ok(())
        })
    }
}
