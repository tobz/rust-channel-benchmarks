mod message;

use flume::{bounded, unbounded, Receiver, Sender};

std::include!("settings.in");

fn new<T>(cap: Option<usize>) -> (Sender<T>, Receiver<T>) {
    match cap {
        None => unbounded(),
        Some(cap) => bounded(cap),
    }
}

fn mpmc(cap: Option<usize>) {
    let (tx, rx) = new(cap);

    crossbeam::scope(|scope| {
        for _ in 0..THREADS {
            scope.spawn(|_| {
                for i in 0..MESSAGES / THREADS {
                    tx.send(message::new(i)).unwrap();
                }
            });
        }

        for _ in 0..THREADS {
            scope.spawn(|_| {
                for _ in 0..MESSAGES / THREADS {
                    rx.recv().unwrap();
                }
            });
        }
    })
    .unwrap();
}

fn mpsc(cap: Option<usize>) {
    let (tx, rx) = new(cap);

    crossbeam::scope(|scope| {
        for _ in 0..THREADS {
            scope.spawn(|_| {
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

fn seq(cap: Option<usize>) {
    let (tx, rx) = new(cap);

    for i in 0..MESSAGES {
        tx.send(message::new(i)).unwrap();
    }

    for _ in 0..MESSAGES {
        rx.recv().unwrap();
    }
}

fn spsc(cap: Option<usize>) {
    let (tx, rx) = new(cap);

    crossbeam::scope(|scope| {
        scope.spawn(|_| {
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

/*
fn select_rx(cap: Option<usize>) {
    let chans = (0..THREADS).map(|_| new(cap)).collect::<Vec<_>>();

    crossbeam::scope(|scope| {
        for (tx, _) in &chans {
            let tx = tx.clone();
            scope.spawn(move |_| {
                for i in 0..MESSAGES / THREADS {
                    tx.send(message::new(i)).unwrap();
                }
            });
        }

        for _ in 0..MESSAGES {
            let mut sel = Selector::new();
            for (_, rx) in &chans {
                sel.recv(rx);
            }
            let case = sel.select();
            let index = case.index();
            case.recv(&chans[index].1).unwrap();
        }
    })
    .unwrap();
}

fn select_both(cap: Option<usize>) {
    let chans = (0..THREADS).map(|_| new(cap)).collect::<Vec<_>>();

    crossbeam::scope(|scope| {
        for _ in 0..THREADS {
            scope.spawn(|_| {
                for i in 0..MESSAGES / THREADS {
                    let mut sel = Selector::new();
                    for (tx, _) in &chans {
                        sel.send(tx);
                    }
                    let case = sel.select();
                    let index = case.index();
                    case.send(&chans[index].0, message::new(i)).unwrap();
                }
            });
        }

        for _ in 0..THREADS {
            scope.spawn(|_| {
                for _ in 0..MESSAGES / THREADS {
                    let mut sel = Select::new();
                    for (_, rx) in &chans {
                        sel.recv(rx);
                    }
                    let case = sel.select();
                    let index = case.index();
                    case.recv(&chans[index].1).unwrap();
                }
            });
        }
    })
    .unwrap();
}
 */

fn main() {
    macro_rules! run {
        ($name:expr, $f:expr) => {
            let now = ::std::time::Instant::now();
            $f;
            let elapsed = now.elapsed();
            println!("{},{}", $name, elapsed.as_nanos());
        };
    }

    println!("flume");

    run!("bounded0_mpmc", mpmc(Some(0)));
    run!("bounded0_mpsc", mpsc(Some(0)));
    //run!("bounded0_select_both", select_both(Some(0)));
    //run!("bounded0_select_rx", select_rx(Some(0)));
    run!("bounded0_spsc", spsc(Some(0)));

    run!("bounded1_mpmc", mpmc(Some(1)));
    run!("bounded1_mpsc", mpsc(Some(1)));
    //run!("bounded1_select_both", select_both(Some(1)));
    //run!("bounded1_select_rx", select_rx(Some(1)));
    run!("bounded1_spsc", spsc(Some(1)));

    run!("bounded_mpmc", mpmc(Some(MESSAGES)));
    run!("bounded_mpsc", mpsc(Some(MESSAGES)));
    //run!("bounded_select_both", select_both(Some(MESSAGES)));
    //run!("bounded_select_rx", select_rx(Some(MESSAGES)));
    run!("bounded_seq", seq(Some(MESSAGES)));
    run!("bounded_spsc", spsc(Some(MESSAGES)));

    run!("unbounded_mpmc", mpmc(None));
    run!("unbounded_mpsc", mpsc(None));
    //run!("unbounded_select_both", select_both(None));
    //run!("unbounded_select_rx", select_rx(None));
    run!("unbounded_seq", seq(None));
    run!("unbounded_spsc", spsc(None));
}
