use std::collections::HashMap;
use termion::event::Key;

fn gen_hashmap() ->HashMap {
    let mut mymap = HashMap::new();
    //let mut keyboard = [Key, 16];
    let keyboard = ['1','2','3','4',
                    'q','w','e','r',
                    'a','s','d','f',
                    'z','x','c','v'];
    let kbKeys = keyboard.map(Key::Char());    
    kbKeys.map(|x| mymap.insert(x, )); 
    map
}

fn inputfromkeyboard(){

}