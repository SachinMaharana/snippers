use crate::models;
// use diesel::debug_query;
// use diesel::mysql::Mysql;
// use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn find_snippet_by_id(
    nm: i32,
    conn: &MysqlConnection,
) -> Result<Option<models::Snippet>, diesel::result::Error> {
    use crate::schema::snippets::dsl::*;

    let snippet = snippets
        .filter(id.eq(nm))
        .first::<models::Snippet>(conn)
        .optional()?;

    Ok(snippet)
}

pub fn insert_new_snippet(
    title: &str,
    content: &str,
    conn: &MysqlConnection,
) -> Result<models::Snippet, diesel::result::Error> {
    use crate::schema::snippets;

    let new_snippet = models::NewSnippet {
        title: title.to_string(),
        content: content.to_string(),
    };
    // let q = diesel::insert_into(snippets::table).values(&new_snippet);
    // println!("{}", debug_query::<Mysql, _>(&q).to_string());

    diesel::insert_into(snippets::table)
        .values(&new_snippet)
        .execute(conn)?;

    Ok(snippets::table
        .order(snippets::id.desc())
        .first(conn)
        .unwrap())
}
