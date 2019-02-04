use super::schema::words;

#[derive(Queryable)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub Gender: String,
}


#[derive(Insertable)]
#[table_name = "words"]
pub struct NewWord<'a> {
    pub word: &'a str,
    pub gender: &'a str,
}
