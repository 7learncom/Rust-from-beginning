const INITIAL_BALANCE: f64 = 1000.0;

#[derive(Debug)]
struct Account<'a> {
    account_holder: &'a String,
    balance: f64,
}

#[derive(Debug)]
enum Transaction {
    Deposit { amount: f64 },
    Withdrawal { amount: f64 },
    Transfer { amount: f64, recipient: String },
}

fn process_account(account: &mut Account, transactions: Vec<Transaction>) {
    println!("Processing Account for: {}", account.account_holder);

    for transaction in transactions {
        match transaction {
            Transaction::Deposit { amount } => {
                account.balance += amount;
                println!("Deposit: +${}", amount)
            }
            Transaction::Withdrawal { amount } => {
                if amount <= account.balance {
                    account.balance -= amount;
                    println!("Withdrawal: -${}", amount);
                } else {
                    println!("Insufficient funds for withdrawal");
                }
            }
            Transaction::Transfer { amount, recipient } => {
                if amount <= account.balance {
                    account.balance -= amount;
                    println!("Transfer: -${} to {}", amount, recipient);
                } else {
                    println!("Insufficient funds for transfer");
                }
            }
        }
    }
}

fn print_account(account: &Account) {
    println!("Account Details: {:?}", account);
}

fn main() {
    let account_holder = String::from("Pouya");

    {}
    let mut initial_account = Account {
        account_holder: &account_holder,
        balance: INITIAL_BALANCE,
    };

    let transactions = vec![
        Transaction::Deposit { amount: 500.0 },
        Transaction::Withdrawal { amount: 200.0 },
        Transaction::Transfer {
            amount: 3000.0,
            recipient: String::from("Ali"),
        },
    ];

    process_account(&mut initial_account, transactions);
    println!("{}", account_holder);
    print_account(&initial_account);
}
