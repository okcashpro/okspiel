use crate::ok_client::{Account, Walletlocked};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectNodeModel {
    pub name: String,
    pub address: String,
    pub username: String,
    pub password: String,
    pub phrase: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountAddresses {
    pub account: String,
    pub addresses: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectNodeDto {
    pub name: String,
    pub address: String,
    pub accounts: Vec<Account>,
    pub username: String,
    pub password: String,
    pub phrase: String,
    pub status: Walletlocked,
    pub staking: bool,
    pub connected: bool,
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

impl From<ConnectNodeDto> for ConnectNodeModel {
    fn from(connect_node_dto: ConnectNodeDto) -> Self {
        Self {
            name: connect_node_dto.name,
            address: connect_node_dto.address,
            username: connect_node_dto.username,
            password: connect_node_dto.password,
            phrase: connect_node_dto.phrase,
        }
    }
}

impl
    From<(
        String,
        String,
        Vec<Account>,
        String,
        String,
        String,
        Walletlocked,
        bool,
        bool,
    )> for ConnectNodeDto
{
    fn from(
        connect_node: (
            String,
            String,
            Vec<Account>,
            String,
            String,
            String,
            Walletlocked,
            bool,
            bool,
        ),
    ) -> Self {
        let (name, address, accounts, username, password, phrase, status, staking, connected) =
            connect_node;

        Self {
            name,
            address,
            accounts,
            username,
            password,
            phrase,
            status,
            staking,
            connected,
        }
    }
}

impl From<(String, Vec<String>)> for AccountAddresses {
    fn from(account_and_addresses: (String, Vec<String>)) -> Self {
        let (account, addresses) = account_and_addresses;

        Self { account, addresses }
    }
}
