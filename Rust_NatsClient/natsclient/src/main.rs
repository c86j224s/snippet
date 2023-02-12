use std::{thread, time::Duration};

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let addr: &'static str = "127.0.0.1";
    let subject: &'static str = "nats1.subject";

    let nc = nats::connect(addr)?;

    let nc2 = nats::connect(addr)?;
    nc2.subscribe(subject)?.with_handler(move |msg| {
        println!("nc2 received from {}, msg[{}]", subject, msg);

        if let Some(reply) = msg.reply {
            nc2.publish(
                &reply,
                &format!(
                    "hey bro, your message is {}",
                    String::from_utf8_lossy(&msg.data)
                ),
            )?;
        }

        Ok(())
    });

    nc.publish(subject, "hey world?")?;

    let reply = nc.request_timeout(subject, "hey is there anyone?", Duration::from_secs(1))?;
    println!("reply[{}]", reply);

    thread::sleep(Duration::from_secs(5));

    println!("Goodbye, world!");

    Ok(())
}
