pub enum EEntityType {
    Controller,
    Card,
    CinionCard,
    CpellCard,
}

pub trait Entity: Send {
    fn get_health(&self) -> u16;
    fn get_entity_type(&self) -> EEntityType;
}
