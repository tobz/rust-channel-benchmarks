use std::sync::mpsc;

mod message;

std::include!("settings.in");

fn seq_async() {
    let (tx, rx) = mpsc::channel();

    for i in 0..MESSAGES {
        tx.send(message::new(i)).unwrap();
    }

    for _ in 0..MESSAGES {
        rx.recv().unwrap();
    }
}

fn seq_sync(cap: usize) {
    let (tx, rx) = mpsc::sync_channel(cap);

    for i in 0..MESSAGES {
        tx.send(message::new(i)).unwrap();
    }

    for _ in 0..MESSAGES {
        rx.recv().unwrap();
    }
}

fn spsc_async() {
    let (tx, rx) = mpsc::channel();

    crossbeam::scope(|scope| {
        scope.spawn(move |_| {
            for i in 0..MESSAGES {
                tx.send(message::new(i)).unwrap();
            }
        });

        for _ in 0..MESSAGES {
            rx.recv().unwrap();
        }
    })
    .unwrap();
}

fn spsc_sync(cap: usize) {
    let (tx, rx) = mpsc::sync_channel(cap);

    crossbeam::scope(|scope| {
        scope.spawn(move |_| {
            for i in 0..MESSAGES {
                tx.send(message::new(i)).unwrap();
            }
        });

        for _ in 0..MESSAGES {
            rx.recv().unwrap();
        }
    })
    .unwrap();
}

fn mpsc_async() {
    let (tx, rx) = mpsc::channel();

    crossbeam::scope(|scope| {
        for _ in 0..THREADS {
            let tx = tx.clone();
            scope.spawn(move |_| {
                for i in 0..MESSAGES / THREADS {
                    tx.send(message::new(i)).unwrap();
                }
            });
        }

        for _ in 0..MESSAGES {
            rx.recv().unwrap();
        }
    })
    .unwrap();
}

fn mpsc_sync(cap: usize) {
    let (tx, rx) = mpsc::sync_channel(cap);

    crossbeam::scope(|scope| {
        for _ in 0..THREADS {
            let tx = tx.clone();
            scope.spawn(move |_| {
                for i in 0..MESSAGES / THREADS {
                    tx.send(message::new(i)).unwrap();
                }
            });
        }

        for _ in 0..MESSAGES {
            rx.recv().unwrap();
        }
    })
    .unwrap();
}

fn main() {
    macro_rules! run {
        ($name:expr, $f:expr) => {
            let now = ::std::time::Instant::now();
            $f;
            let elapsed = now.elapsed();
            println!("{},{}", $name, elapsed.as_nanos());
        };
    }

    println!("std::mpsc");
    run!("bounded0_mpsc", mpsc_sync(0));
    run!("bounded0_spsc", spsc_sync(0));

    run!("bounded1_mpsc", mpsc_sync(1));
    run!("bounded1_spsc", spsc_sync(1));

    run!("bounded_mpsc", mpsc_sync(MESSAGES));
    run!("bounded_seq", seq_sync(MESSAGES));
    run!("bounded_spsc", spsc_sync(MESSAGES));

    run!("unbounded_mpsc", mpsc_async());
    run!("unbounded_seq", seq_async());
    run!("unbounded_spsc", spsc_async());
}
