use rune_vm::Rune;

use std::*;

use runes::add_tag::AddTag;
use runes::deal_card::DealCard;
use runes::start_game::StartGame;
use runes::rotate_turn::RotateTurn;
use runes::shuffle_card::ShuffleCard;
use runes::new_controller::NewController;
//use runes::mulligan::Mulligan;
//use runes::play_card::PlayCard;
use runes::kill_minion::KillMinion;
use runes::set_mana::SetMana;
use runes::set_health::SetHealth;
use runes::set_attack::SetAttack;
use runes::modify_attack::ModifyAttack;
use runes::modify_health::ModifyHealth;
use runes::set_base_mana::*;
use runes::play_minion::PlayMinion;
use runes::remove_tag::RemoveTag;
//use runes::create_minion::CreateMinion;
use runes::summon_minion::SummonMinion;
use runes::damage_rune::DamageRune;
use rustc_serialize::json;
use rustc_serialize::json::Json;


pub fn get_rune(json_obj : &str)->Box<Rune>{
    let j_message: Json = Json::from_str(json_obj.trim()).unwrap();
    let obj = j_message.as_object().unwrap();
    let message_type : String = obj.get("runeType").unwrap().to_string();

    println!("Decoding to Boxed Struct : {0}", json_obj);

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
        ref x if x ==  "\"SetBaseMana\""=>{
            println!("SetBaseMana found");
            let ns = json_obj.replace("{\"runeType\":\"SetBaseMana\",","{");
            let dc : SetBaseMana = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        ref x if x ==  "\"SetMana\""=>{
            println!("SetMana found");
            let ns = json_obj.replace("{\"runeType\":\"SetMana\",","{");
            let dc : SetMana = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        ref x if x ==  "\"RotateTurn\""=>{
            println!("RotateTurn found");
            let ns = json_obj.replace("{\"runeType\":\"RotateTurn\",","{");
            let dc : RotateTurn = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        ref x if x ==  "\"AddTag\""=>{
            println!("AddTag found");
            let ns = json_obj.replace("{\"runeType\":\"AddTag\",","{");
            let dc : AddTag = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        ref x if x ==  "\"SummonMinion\""=>{
            println!("SummonMinion found");
            let ns = json_obj.replace("{\"runeType\":\"SummonMinion\",","{");
            let dc : SummonMinion = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        ref x if x ==  "\"PlayMinion\""=>{
            println!("PlayMinion found");
            let ns = json_obj.replace("{\"runeType\":\"PlayMinion\",","{");
            let dc : PlayMinion = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        ref x if x ==  "\"RemoveTag\""=>{
            println!("RemoveTag found");
            let ns = json_obj.replace("{\"runeType\":\"RemoveTag\",","{");
            let dc : RemoveTag = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        ref x if x ==  "\"SetHealth\""=>{
            println!("SetHealth found");
            let ns = json_obj.replace("{\"runeType\":\"SetHealth\",","{");
            let dc : SetHealth = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        ref x if x ==  "\"ModifyHealth\""=>{
            println!("ModifyHealth found");
            let ns = json_obj.replace("{\"runeType\":\"ModifyHealth\",","{");
            let dc : ModifyHealth = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        ref x if x ==  "\"SetAttack\""=>{
            println!("SetAttack found");
            let ns = json_obj.replace("{\"runeType\":\"SetAttack\",","{");
            let dc : SetAttack = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        ref x if x ==  "\"ModifyAttack\""=>{
            println!("ModifyAttack found");
            let ns = json_obj.replace("{\"runeType\":\"ModifyAttack\",","{");
            let dc : ModifyAttack = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        ref x if x ==  "\"DamageRune\""=>{
            println!("ModifyAttack found");
            let ns = json_obj.replace("{\"runeType\":\"DamageRune\",","{");
            let dc : DamageRune = json::decode(ns.trim()).unwrap();
            return dc.into_box();
        },
        ref x if x ==  "\"KillMinion\""=>{
            println!("DamageRune found");
            let ns = json_obj.replace("{\"runeType\":\"KillMinion\",","{");
            let dc : KillMinion = json::decode(ns.trim()).unwrap();
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