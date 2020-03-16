use wasm_bindgen::prelude::*;

use kingslayer::Cli;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str) -> String;
    pub fn prompt(s: &str) -> String;
}

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn start() {
    set_panic_hook();

    let cli = Cli::from_ron_str(r#"
(running:true,last_cmd_res:(action:Active,output:"Brig\nYou are in a small wooden room with a wood pillar in the middle.\n\tThe ground slowly creaks and rocks beneath you.\nThere is a door on the north side. The way is shut.\nThere is a stick here.",request_input:None,),last_cmd:(num_words:1,verb:Some("l"),obj:None,prep:None,obj_prep:None,),player:(hp:(13,13,),xp:(0,1000,),in_combat:Resting,lvl:1,stats:(pts:4,strngth:14,dex:13,con:15,int:12,wis:10,cha:8,),main_hand:None,armor:None,inventory:(items:[],gold:0,),),world:(curr_room:"Brig",rooms:{"Sterncastle":(name:"Sterncastle",desc:"You are on the sterncastle of the ship. There is another mast in the center and a the ships wheel.",paths:[(directions:["stairs","north","down",],target:"Main Deck",desc:"The stairs lead back onto the main deck.",inspect:"The stairs are old and dirty.",opening:None,lock:None,),],enemies:[],allies:[],elements:[],items:[],),"Brig":(name:"Brig",desc:"You are in a small wooden room with a wood pillar in the middle.\n\tThe ground slowly creaks and rocks beneath you.",paths:[(directions:["door","north",],target:"Hold 1",desc:"There is a door on the north side.",inspect:"The door is plain and wooden.",opening:Some(Closed),lock:Some(Locked),),],enemies:[],allies:[],elements:[],items:[Weapon((name:"stick",desc:"There is a stick here.",inspect:"It\'s short but stout.",damage:4,)),],),"Infirmary":(name:"Infirmary",desc:"You are in a room with a few empty beds against one wall.",paths:[(directions:["doorway","north",],target:"Crew Deck 2",desc:"The doorway leads back into the crew\'s quarters.",inspect:"The doorway flickers with lantern light.",opening:None,lock:None,),],enemies:[],allies:[],elements:[],items:[],),"Crew Deck 2":(name:"Crew Deck",desc:"You are on a deck with dirty hammocks hanging everywhere.",paths:[(directions:["opening","up",],target:"Cannon Deck 2",desc:"There is an opening above you.",inspect:"You think you might be able to see the light of day.",opening:None,lock:None,),(directions:["north",],target:"Crew Deck 1",desc:"The deck continues to the north.",inspect:"There is a hatch at the north end of the deck.",opening:None,lock:None,),(directions:["empty doorway","south",],target:"Infirmary",desc:"There is an empty doorway to the south.",inspect:"The doorway flickers with lantern light.",opening:None,lock:None,),],enemies:[],allies:[],elements:[],items:[],),"Hold 2":(name:"Hold",desc:"You are at the end of a spacious hold. Crates and barrels array the sides.",paths:[(directions:["hatch","up",],target:"Crew Deck 1",desc:"There is a hatch above you.",inspect:"It looks like a metal grate.",opening:Some(Closed),lock:None,),(directions:["south",],target:"Hold 1",desc:"The hold continues to the south.",inspect:"The middle of the hold is lit by a lantern.",opening:None,lock:None,),],enemies:[(name:"swarm rats",desc:"A swarm of rats raves along the floor.",inspect:"The creatures chatter and scrape viciously.",hp:24,ac:0,xp:50,damage:3,status:Angry,loot:[],),],allies:[],elements:[(name:"root beer barrel barrels",desc:"You can detect the slight scent of root beer.",inspect:"You can find no way to open the barrels, but there is definitely root beer inside.",),],items:[Weapon((name:"sword",desc:"There is a sword here.",inspect:"It\'s a basic short sword with a few knicks.",damage:6,)),Armor((name:"leather armor",desc:"There is a leather armor here.",inspect:"It\'s well worn but reliable.",ac:11,)),],),"Empty Room":(name:"Empty Room",desc:"The room is dark and empty.",paths:[(directions:["doorway","north",],target:"Cannon Deck 2",desc:"The doorway leads back into the cannon deck.",inspect:"You see many cannons.",opening:None,lock:None,),],enemies:[],allies:[],elements:[],items:[],),"Main Deck":(name:"Main Deck",desc:"The vast open sea surrounds the ship you stand on.",paths:[(directions:["hatch","down",],target:"Cannon Deck 1",desc:"There is a hatch below you.",inspect:"The hatch is a double-hinged grate made of old rusty metal.",opening:Some(Open),lock:None,),(directions:["mast","platform","up",],target:"Platform",desc:"There is a platform above you on the central mast.",inspect:"The platform can be reached through holds on the mast.",opening:None,lock:None,),(directions:["stairs","south",],target:"Sterncastle",desc:"Stairs towards the south lead upwards onto the sterncastle.",inspect:"The stairs are old and dirty.",opening:None,lock:None,),(directions:["door",],target:"Captains Cabin",desc:"There is door on the wall beneath the sterncastle of the ship.",inspect:"The door is large with a small dim window in the center.",opening:Some(Closed),lock:None,),],enemies:[],allies:[],elements:[],items:[],),"Platform":(name:"Platform",desc:"You stand on a platform several feet above the main deck.",paths:[(directions:["down",],target:"Main Deck",desc:"The main deck is below you.",inspect:"The main deck is several feet below.",opening:None,lock:None,),(directions:["crows nest","up",],target:"Crows Nest",desc:"There is a crows nest above you on the central mast.",inspect:"The crows nest can be reached through holds and rigging on the mast.",opening:None,lock:None,),],enemies:[],allies:[],elements:[],items:[],),"Hold 1":(name:"Hold",desc:"You are in the middle of a spacious hold. Crates and barrels array the sides.",paths:[(directions:["door","south",],target:"Brig",desc:"There is a door at the south end of the hold.",inspect:"The door is plain and wooden.",opening:Some(Open),lock:None,),(directions:["north",],target:"Hold 2",desc:"The hold continues to the north.",inspect:"This end of the hold is too dark to see into.",opening:None,lock:None,),],enemies:[(name:"pirate",desc:"There is a pirate lying in a chair.",inspect:"He seems to be intently snoring.",hp:1,ac:0,xp:75,damage:8,status:Asleep,loot:[],),],allies:[],elements:[(name:"root beer barrel barrels",desc:"You can detect the slight scent of root beer.",inspect:"You can find no way to open the barrels, but there is definitely root beer inside.",),],items:[],),"Cannon Deck 2":(name:"Cannon Deck",desc:"This deck has cannons lining each side.",paths:[(directions:["north",],target:"Cannon Deck 1",desc:"The deck continues to the north.",inspect:"This ship sure has a lot of cannons.",opening:None,lock:None,),(directions:["doorway","south",],target:"Empty Room",desc:"There is a doorway to the south.",inspect:"The doorway is completely dark.",opening:None,lock:None,),(directions:["opening","down",],target:"Crew Deck 2",desc:"There is an opening below you.",inspect:"You can see hammocks through the opening.",opening:None,lock:None,),],enemies:[],allies:[],elements:[],items:[],),"Crew Deck 1":(name:"Crew Deck",desc:"You are on a deck with dirty hammocks hanging everywhere.",paths:[(directions:["hatch","down",],target:"Hold 2",desc:"There is a hatch on the ground",inspect:"It looks like a metal grate.",opening:Some(Open),lock:None,),(directions:["south",],target:"Crew Deck 2",desc:"The deck continues to the south.",inspect:"There is a doorway at the south end of the deck.",opening:None,lock:None,),],enemies:[(name:"pirate",desc:"There is a pirate here.",inspect:"The pirate is armed and smells vile.",hp:18,ac:9,xp:75,damage:8,status:Asleep,loot:[Gold((name:"gold",desc:"There is 10 gold here.",inspect:"It is shiny and smooth",amount:10,)),],),(name:"pirate",desc:"There is a pirate here.",inspect:"The pirate is armed and smells vile.",hp:17,ac:9,xp:75,damage:8,status:Asleep,loot:[Weapon((name:"cutlass",desc:"There is a cutlass here.",inspect:"It has thick steel and many notches.",damage:8,)),Gold((name:"gold",desc:"There is 10 gold here.",inspect:"It is shiny and smooth",amount:10,)),],),],allies:[],elements:[],items:[],),"Captains Cabin":(name:"Captains Cabin",desc:"You stand in a large cabin with the captains belongings littering the ground and hanging on the walls.",paths:[(directions:["door","north",],target:"Main Deck",desc:"There is door at the north end of the room.",inspect:"The door is large with a small dim window in the center.",opening:Some(Open),lock:None,),],enemies:[(name:"pirate captain",desc:"There is a pirate captain here.",inspect:"He grins, showing off multiple golden teeth.",hp:50,ac:12,xp:600,damage:12,status:Angry,loot:[Thing((name:"blue ring",desc:"There is a blue ring here.",inspect:"When the ring catches the sunlight, the surface shimmers like the waves of the sea.",)),Gold((name:"gold",desc:"There is 50 gold here.",inspect:"It is shiny and smooth",amount:50,)),],),],allies:[],elements:[],items:[],),"Cannon Deck 1":(name:"Cannon Deck",desc:"This deck has cannons lining each side.",paths:[(directions:["south",],target:"Cannon Deck 2",desc:"The deck continues to the south.",inspect:"This ship sure has a lot of cannons.",opening:None,lock:None,),(directions:["hatch","up",],target:"Main Deck",desc:"A hatch above you bring in bright sunlight.",inspect:"The hatch is a double-hinged grate made of old rusty metal.",opening:Some(Closed),lock:None,),],enemies:[(name:"pirate",desc:"There is a pirate here.",inspect:"The pirate is armed and smells vile.",hp:15,ac:9,xp:75,damage:8,status:Angry,loot:[Gold((name:"gold",desc:"There is 10 gold here.",inspect:"It is shiny and smooth",amount:10,)),],),],allies:[],elements:[],items:[],),"Crows Nest":(name:"Crows Nest",desc:"You are in a crows nest overlooking the entire ship and sea.",paths:[(directions:["mast","platform","down",],target:"Platform",desc:"There is a platform below you on the central mast.",inspect:"The platform can be reached through holds on the mast.",opening:None,lock:None,),],enemies:[(name:"pirate",desc:"There is a pirate here.",inspect:"The pirate is armed and smells vile.",hp:17,ac:9,xp:75,damage:8,status:Asleep,loot:[Gold((name:"gold",desc:"There is 20 gold here.",inspect:"It is shiny and smooth",amount:20,)),],),],allies:[],elements:[],items:[],),},),)
    "#);

    let mut res = cli.ask("l");
    loop {
        res = cli.ask(&prompt(&res));
        if res.contains("You died.") {
            alert(&res);
            break;
        }
    }
}
