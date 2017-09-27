use bson::{Document, Bson};
use mongodb::ThreadedClient;
use mongodb::Client;
use mongodb::db::ThreadedDatabase;
use rune_vm::Rune;
use bson;

pub fn write_history(history: Vec<Document>){
        
        let client = Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

        let coll = client.db("Fireplace").collection("games");

        coll.insert_many(history, None).ok().expect("Failed to insert document.");

}

pub fn to_doc(doc: Bson, game_name: String, count: usize, r_type: String) -> Document{
    match doc{
        bson::Bson::Document(mut d)=>{
            d.insert("game", game_name);
            d.insert("RuneCount", count as u64);
            d.insert("RuneType", r_type);
            return d
        },
        _=>{}
    }
    return Document::new();
}