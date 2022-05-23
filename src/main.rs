use std::fs::File;
use std::io;
use std::io::{Write, Read, Stdin};
use chip8::Chip8;

// for async input 
use std::thread;
use std::time;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
// end

mod ram;
mod chip8;
mod keyboard;

fn main(){
    ram::test_ram();
    let mut file = File::open("src/data/BRIX").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).unwrap();
    //print!("Data is {:?}", data);
    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);
    chip8.load_fonts();
    println!("chip8 ready to rumble");
    chip8.printmem();


    //input and loop init
    // Set terminal to raw mode to allow reading stdin one key at a time
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();


    print!("{:?}",gen_hashmap());
    
    /* loop {
        // Read input (if any)
        let input = stdin.next();

        // If a key was pressed
        if let Some(Ok(key)) = input {
            match key {
                // Exit if esc is pressed
                termion::event::Key::Esc => break,
                // Else print the pressed key
                _ => {
                    if key == termion::event::Key::Char('a'){
                        write!(
                            stdout,
                            "{}{}{:?} = a",
                            termion::clear::All,
                            termion::cursor::Goto(1, 1),
                            key
                        ).unwrap();
                    }
                    else {
                        write!(
                            stdout,
                            "{}{}Key pressed: {:?}",
                            termion::clear::All,
                            termion::cursor::Goto(1, 1),
                            key
                        )
                        .unwrap();
                    }
                    stdout.lock().flush().unwrap();
                }
            }
        }
        thread::sleep(time::Duration::from_millis(50));
    } */
    
}
