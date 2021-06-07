use crate::communication;
use crate::error::Result;
use crate::wrapper::channel;
use crate::wrapper::dbus;
use crate::wrapper::thread;

const ERROR_NAME: &str = "resume watcher";
const INTERFACE_LOGIN1: &str = "org.freedesktop.login1.Manager";
const MEMBER_PREPARE_FOR_SLEEP: &str = "PrepareForSleep";
const PATH_LOGIN1: &str = "/org/freedesktop/login1";

pub(super) fn init_resume_notifier(sender: &channel::Sender<communication::Message>) -> Result<()> {
    let notifier = Notifier {
        sender: sender.clone(),
    };

    thread::Thread::new(ERROR_NAME, notifier).run()
}

struct Notifier {
    sender: channel::Sender<communication::Message>,
}

impl thread::Runnable for Notifier {
    fn run(&self) -> Result<()> {
        let connection = dbus::Connection::init()?;

        connection.add_match(dbus::Match::new(
            INTERFACE_LOGIN1,
            MEMBER_PREPARE_FOR_SLEEP,
            PATH_LOGIN1,
        ))?;

        connection.listen_for_signals(|signal| {
            // return value is true if going to sleep, false if waking up
            if signal.is_interface(INTERFACE_LOGIN1)? && !signal.return_value::<bool>()? {
                self.sender.send(communication::Message::UpdateAll)?;
            }

            Ok(())
        })
    }
}
