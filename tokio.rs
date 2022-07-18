imod message;

use tokio::sync::mpsc::{channel, unbounded_channel};

std::include!("settings.in");

async fn mpmc(cap: Option<usize>) {
    match cap {
        Some(cap) => mpmc_bounded(cap).await,
        None => mpmc_unbounded().await,
    }
}

async fn mpmc_bounded(cap: usize) {
    let (tx, rx) = channel(cap);
    let mut list = Vec::new();
    for _ in 0..THREADS {
        let tx = tx.clone();
        let h = tokio::spawn(async move {
            for i in 0..MESSAGES / THREADS {
                tx.send(message::new(i)).await.unwrap();
            }
        });
        list.push(h);
    }

    for _ in 0..THREADS {
        let rx = rx.clone();
        let h = tokio::spawn(async move {
            for _ in 0..MESSAGES / THREADS {
                rx.recv().await.unwrap();
            }
        });
        list.push(h);
    }

    for h in list {
        h.await.unwrap();
    }
}

async fn mpmc_unbounded() {
    let (tx, rx) = unbounded_channel();
    let mut list = Vec::new();
    for _ in 0..THREADS {
        let tx = tx.clone();
        let h = tokio::spawn(async move {
            for i in 0..MESSAGES / THREADS {
                tx.send(message::new(i)).await.unwrap();
            }
        });
        list.push(h);
    }

    for _ in 0..THREADS {
        let rx = rx.clone();
        let h = tokio::spawn(async move {
            for _ in 0..MESSAGES / THREADS {
                rx.recv().await.unwrap();
            }
        });
        list.push(h);
    }

    for h in list {
        h.await.unwrap();
    }
}

async fn mpsc(cap: Option<usize>) {
    match cap {
        Some(cap) => mpsc_bounded(cap).await,
        None => mpsc_unbounded().await,
    }
}

async fn mpsc_bounded(cap: usize) {
    let (tx, rx) = channel(cap);
    let mut list = Vec::new();

    for _ in 0..THREADS {
        let tx = tx.clone();
        let h = tokio::spawn(async move {
            for i in 0..MESSAGES / THREADS {
                tx.send(message::new(i)).await.unwrap();
            }
            true
        });
        list.push(h);
    }

    for _ in 0..MESSAGES {
        rx.recv().await.unwrap();
    }
    for h in list {
        h.await.unwrap();
    }
}

async fn mpsc_unbounded() {
    let (tx, rx) = unbounded_channel();
    let mut list = Vec::new();

    for _ in 0..THREADS {
        let tx = tx.clone();
        let h = tokio::spawn(async move {
            for i in 0..MESSAGES / THREADS {
                tx.send(message::new(i)).await.unwrap();
            }
            true
        });
        list.push(h);
    }

    for _ in 0..MESSAGES {
        rx.recv().await.unwrap();
    }
    for h in list {
        h.await.unwrap();
    }
}

async fn seq(cap: Option<usize>) {
    match cap {
        Some(cap) => seq_bounded(cap).await,
        None => seq_unbounded().await,
    }
}

async fn seq_bounded(cap: usize) {
    let (tx, rx) = channel(cap);

    for i in 0..MESSAGES {
        tx.send(message::new(i)).await.unwrap();
    }

    for _ in 0..MESSAGES {
        rx.recv().await.unwrap();
    }
}

async fn seq_unbounded() {
    let (tx, rx) = channel(cap);

    for i in 0..MESSAGES {
        tx.send(message::new(i)).await.unwrap();
    }

    for _ in 0..MESSAGES {
        rx.recv().await.unwrap();
    }
}

async fn spsc(cap: Option<usize>) {
    match cap {
        Some(cap) => spsc_bounded(cap).await,
        None => spsc_unbounded().await,
    }
}

async fn spsc_bounded(cap: usize) {
    let (tx, rx) = channel(cap);

    let h = tokio::spawn(async move {
        for i in 0..MESSAGES {
            tx.send(message::new(i)).await.unwrap();
        }
    });

    for _ in 0..MESSAGES {
        rx.recv().await.unwrap();
    }

    h.await.unwrap();
}

async fn spsc_unbounded() {
    let (tx, rx) = unbounded_channel();

    let h = tokio::spawn(async move {
        for i in 0..MESSAGES {
            tx.send(message::new(i)).await.unwrap();
        }
    });

    for _ in 0..MESSAGES {
        rx.recv().await.unwrap();
    }

    h.await.unwrap();
}

#[tokio::main]
async fn main() {:w
	
    macro_rules! run {
        ($name:expr, $f:expr) => {
            let now = ::std::time::Instant::now();
            $f.await;
            let elapsed = now.elapsed();
            println!("{},{}", $name, elapsed.as_nanos());
        };
    }

    println!("tokio");
    run!("bounded0_mpmc", mpmc(Some(0)));
    run!("bounded0_mpsc", mpsc(Some(0)));
    run!("bounded0_spsc", spsc(Some(0)));

    run!("bounded1_mpmc", mpmc(Some(1)));
    run!("bounded1_mpsc", mpsc(Some(1)));
    run!("bounded1_spsc", spsc(Some(1)));

    run!("bounded_mpmc", mpmc(Some(MESSAGES)));
    run!("bounded_mpsc", mpsc(Some(MESSAGES)));
    run!("bounded_seq", seq(Some(MESSAGES)));
    run!("bounded_spsc", spsc(Some(MESSAGES)));

    run!("unbounded_mpmc", mpmc(None));
    run!("unbounded_mpsc", mpsc(None));
    run!("unbounded_seq", seq(None));
    run!("unbounded_spsc", spsc(None));
}
