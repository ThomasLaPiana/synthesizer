use crate::models::Pipeline;

pub fn get_database() -> Vec<Pipeline> {
    let database: Vec<Pipeline> = vec![Pipeline {
        id: String::from("tp1"),
        name: String::from("Test Pipeline 1"),
        schedule: String::from("1 * * * *"),
    }];
    database
}

pub fn add_to_database() {
    let mut database = get_database();
    database.extend(vec![Pipeline {
        id: String::from("tp2"),
        name: String::from("Test Pipeline 2"),
        schedule: String::from("2 * * * *"),
    }]);
}
