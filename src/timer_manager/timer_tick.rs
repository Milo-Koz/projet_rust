
use std::sync::mpsc::{Receiver, Sender}; //RecvError

use std::thread;
use std::time::Duration;
use crate::EventTimerTick;

pub enum Tick {
    Tick
}

pub fn run(rx_timer: Receiver<EventTimerTick>, tx_timer: Sender<Tick>) {

    //println!("Timer tick - Run");

    let mut end_game = false;

    match rx_timer.recv()
    {
        Ok(EventTimerTick::End) =>  {
            end_game = true;
            println!("Timer tick - skipping ticks");
        },
        Ok(_) => {}
        Err(_) => {println!("[ERREUR À DÉFINIR]")},
    }

    while !end_game {

        // Wait for 1 sec
        thread::sleep(Duration::from_millis(1000));
        //println!("tick");
        let _ = tx_timer.send(Tick::Tick);

        // Check if we should pause or quit.
        if let Ok(value_received) = rx_timer.try_recv() {
            match value_received {
                EventTimerTick::Pause => {
                    println!("Pause timer received");
                    end_game = true;
                },
                EventTimerTick::End => {
                    println!("End timer received");
                    end_game = true;
                }
                ErrRec => {}
            }
        } 
    }

    println!("Timer tick - End game");
}
