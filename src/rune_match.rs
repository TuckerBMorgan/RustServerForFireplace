use rune_vm::Rune;

use std::*;

use runes::deal_card::DealCard;
use runes::start_game::StartGame;
use runes::rotate_turn::RotateTurn;
use runes::shuffle_card::ShuffleCard;
use runes::new_controller::NewController;
use runes::mulligan::Mulligan;
use runes::play_card::PlayCard;
use runes::kill_minion::KillMinion;
use runes::set_mana::SetMana;
use runes::set_health::SetHealth;
use runes::set_attack::SetAttack;
use runes::modify_attack::ModifyAttack;
use runes::modify_health::ModifyHealth;
//use runes::create_minion::CreateMinion;
use runes::summon_minion::SummonMinion;
use rustc_serialize::json;
use rustc_serialize::json::Json;


pub fn get_rune(json_obj : &str)->Box<Rune>{
    let j_message: Json = Json::from_str(json_obj.trim()).unwrap();
    let obj = j_message.as_object().unwrap();
    let message_type : String = obj.get("runeType").unwrap().to_string();

    println!("AI SEES : {0}", json_obj);

    match message_type {
        ref x if x == "\"DealCard\"" =>{
            println!("DealCard found");
            let ns = json_obj.replace("{\"runeType\":\"DealCard\",","{");
            //let obj = j_message.as_object().unwrap();
            let dc : DealCard = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        ref x if x == "\"NewController\"" =>{
            println!("NewController found");
            let ns = json_obj.replace("{\"runeType\":\"NewController\",","{");
            //let obj = j_message.as_object().unwrap();
            let dc : NewController = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        /*
        ref x if x == "\"CreateMinion\"" =>{
            println!("CreateMinion found");
            let ns = json_obj.replace("{\"runeType\":\"CreateMinion\",","{");
            //let obj = j_message.as_object().unwrap();
            let dc : CreateMinion = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        */
        ref x if x == "\"StartGame\"" =>{
            println!("StartGame found");
            let ns = json_obj.replace("{\"runeType\":\"StartGame\",","{");
            //let obj = j_message.as_object().unwrap();
            let dc : StartGame = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        ref x if x == "\"ShuffleCard\"" =>{
            println!("ShuffleCard found");
            let ns = json_obj.replace("{\"runeType\":\"ShuffleCard\",","{");
            //let obj = j_message.as_object().unwrap();
            let dc : ShuffleCard = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        _=>{
            println!("ShuffleCard found");
            let ns = json_obj.replace("{\"runeType\":\"ShuffleCard\",","{");
            //let obj = j_message.as_object().unwrap();
            let dc : ShuffleCard = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        }

    }

}