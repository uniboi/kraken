#[derive(Debug)]
pub struct Player {
    uid: String,
}

impl Player {
    pub fn new( row: postgres::Row ) -> Player{
        Player{
            uid: row.get("uid"),
        }
    }
}