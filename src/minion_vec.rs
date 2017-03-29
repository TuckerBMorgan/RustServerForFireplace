use minion_card::Minion;
use hlua::LuaTable;

#[derive(Clone)]
pub struct MinionVec {

    data_struct : Vec<Minion>
}

impl MinionVec {
    
    pub fn new() -> MinionVec {
        MinionVec{
            data_struct : vec![]
        }
    }
    
    pub fn from_real_vec(mins: Vec<Minion>) -> MinionVec{
        MinionVec {
            data_struct : mins.clone()
        }
    }

    pub fn push(&mut self, min : Minion) {
        self.data_struct.push(min);
    }

    pub fn get(&mut self, index: i64) -> Minion {
        self.data_struct[index as usize].clone()
    }

    pub fn get_whole_vec(&self) -> Vec<Minion> {
        self.data_struct.clone()
    }

    pub fn len(&self) -> usize{
        self.data_struct.len()
    }

}