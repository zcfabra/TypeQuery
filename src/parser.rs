#[derive(PartialEq, Eq, Debug)]
enum PostgresObject {
    Table(String, String),
    View(String, String),
}

struct ParseSQL {}
