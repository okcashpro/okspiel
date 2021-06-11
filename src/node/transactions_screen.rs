use crate::connect::ConnectMsg;
use crate::ok_client::Transaction;
use chrono::NaiveDateTime;
use iced::{scrollable, Column, Element, Row, Scrollable, Text};

pub struct TransactionsScreen {
    transactions: Vec<Transaction>,
    scroll: scrollable::State,
}

impl TransactionsScreen {
    pub fn new() -> Self {
        Self {
            transactions: vec![],
            scroll: scrollable::State::new(),
        }
    }

    pub fn set_transactions(&mut self, transactions: Vec<Transaction>) {
        self.transactions = transactions;
    }

    pub fn remove_transactions(&mut self) {
        self.transactions = vec![];
    }

    pub fn view(&mut self) -> Element<ConnectMsg> {
        Column::new()
            .spacing(40)
            .push::<Element<ConnectMsg>>(
                Scrollable::new(&mut self.scroll)
                    .push(
                        self.transactions
                            .iter_mut()
                            .rev()
                            .fold(Column::new(), move |c, t| {
                                c.spacing(30).push(
                                    Column::new()
                                        .push(Row::new().push(Text::new("Adress: ")).push(
                                            Text::new(t.account.clone()).color([0.5, 0.5, 0.5]),
                                        ))
                                        .push(Row::new().push(Text::new("Category: ")).push(
                                            Text::new(t.category.clone()).color([0.5, 0.5, 0.5]),
                                        ))
                                        .push(Row::new().push(Text::new("Amount: ")).push(
                                            Text::new(t.amount.to_string()).color([0.5, 0.5, 0.5]),
                                        ))
                                        .push(
                                            Row::new().push(Text::new("Confirmations: ")).push(
                                                Text::new(t.confirmations.to_string())
                                                    .color([0.5, 0.5, 0.5]),
                                            ),
                                        )
                                        .push(Row::new().push(Text::new("Block hash: ")).push(
                                            Text::new(t.blockhash.clone()).color([0.5, 0.5, 0.5]),
                                        ))
                                        .push(
                                            Row::new().push(Text::new("Txid: ")).push(
                                                Text::new(t.txid.clone()).color([0.5, 0.5, 0.5]),
                                            ),
                                        )
                                        .push(
                                            Row::new().push(Text::new("Time received: ")).push(
                                                Text::new(
                                                    NaiveDateTime::from_timestamp(
                                                        t.timereceived,
                                                        0,
                                                    )
                                                    .format("%Y-%m-%d %H:%M:%S")
                                                    .to_string(),
                                                )
                                                .color([0.5, 0.5, 0.5]),
                                            ),
                                        ),
                                )
                            }),
                    )
                    .into(),
            )
            .into()
    }
}
