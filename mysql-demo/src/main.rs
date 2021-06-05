use mysql::prelude::Queryable;
use mysql::Pool;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}
fn main() {
    let url = "mysql://root:root@127.0.0.1:3306/testdb";
    let pool = match Pool::new(url) {
        Ok(a) => a,
        Err(_) => {
            panic!("err")
        }
    };
    let mut conn = match pool.get_conn() {
        Ok(a) => a,
        Err(_) => {
            panic!("err")
        }
    };
    conn.query_drop(
        r"CREATE TEMPORARY TABLE payment (
        customer_id int not null,
        amount int not null,
        account_name text
    )",
    );
    let payments = vec![
        Payment {
            customer_id: 1,
            amount: 2,
            account_name: None,
        },
        Payment {
            customer_id: 2,
            amount: 3,
            account_name: Some("foo".into()),
        },
        Payment {
            customer_id: 3,
            amount: 7,
            account_name: None,
        },
        Payment {
            customer_id: 4,
            amount: 9,
            account_name: Some("bar".into()),
        },
        Payment {
            customer_id: 5,
            amount: 10,
            account_name: None,
        },
    ];
    conn.exec_batch(
        r"INSERT INTO payment (customer_id,amount,account_name) VALUES
    (:customer_id,:amount,:account_name)",
        payments.iter().map(|p| {
            params! {
            "customer_id" => p.customer_id,
            "amount" => p.amount,
            "account_name" => &p.account_name,
            }
        }),
    );
    let selected_payments = match conn.query_map(
        "SELECT customer_id, amount, account_name from payment",
        |(customer_id, amount, account_name)| Payment {
            customer_id,
            amount,
            account_name,
        },
    ) {
        Ok(a) => a,
        Err(_) => {
            panic!("erro")
        }
    };
    assert_eq!(payments, selected_payments);
    println!("Yay!");
}
