#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectNodeModel {
    pub name: String,
    pub address: String,
    pub username: String,
    pub password: String,
    pub phrase: String,
}

impl From<(String, String, String, String, String)> for ConnectNodeModel {
    fn from(connect_node: (String, String, String, String, String)) -> Self {
        let (name, address, username, password, phrase) = connect_node;

        Self {
            name,
            address,
            username,
            password,
            phrase,
        }
    }
}
