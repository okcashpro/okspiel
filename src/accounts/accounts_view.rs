use crate::connect::{AccountAddresses, ConnectMsg, ConnectNodeDto};
use crate::ok_client::Walletlocked;
use iced::{pick_list, Element, PickList, Row};

#[derive(Debug, Clone)]
pub struct AccountsView {
    node: ConnectNodeDto,
    pick_accounts: pick_list::State<String>,
    accounts: Vec<AccountAddresses>,
    selected_account: String,
}

impl AccountsView {
    pub fn new() -> Self {
        Self {
            node: ConnectNodeDto::from((
                String::from(""),
                String::from(""),
                vec![],
                String::from(""),
                String::from(""),
                String::from(""),
                Walletlocked::Unlocked,
                false,
                false,
            )),
            pick_accounts: pick_list::State::default(),
            accounts: vec![],
            selected_account: "Select Account".to_string(),
        }
    }

    pub fn set_accounts(&mut self, accounts: Vec<AccountAddresses>, node: ConnectNodeDto) {
        self.selected_account = "Select Account".to_string();
        self.accounts = accounts;
        self.node = node;
    }

    pub fn set_select_account(&mut self, account: String) {
        self.selected_account = account;
    }

    pub fn view(&mut self) -> Element<ConnectMsg> {
        let accounts = self
            .accounts
            .to_vec()
            .into_iter()
            .map(|a| a.account)
            .collect::<Vec<String>>();

        let accounts_copy = self.accounts.to_vec();

        let node_copy = self.node.clone();

        Row::new()
            .spacing(10)
            .push::<Element<ConnectMsg>>(
                PickList::new(
                    &mut self.pick_accounts,
                    accounts,
                    Some(self.selected_account.clone()),
                    move |account| {
                        let account_addresses = accounts_copy
                            .clone()
                            .into_iter()
                            .find(|a| a.account == account);
                        ConnectMsg::GetAddresses(account_addresses, node_copy.clone())
                    },
                )
                .into(),
            )
            .into()
    }
}
