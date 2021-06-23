enum Movement {
    Up,
    Down,
    Left,
    Right,
}

enum Payment {
    Cash(f32),
    CreditCard(String, f32),
    DebitCard(DebitData),
    Crypto { account_id: String, amount: f32 },
}

struct DebitData {
    pub card_number: String,
    pub amount: f32,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

fn process_payment(some_payment: Payment) {
    match some_payment {
        Payment::Cash(amt) => {
            println!("Paying with cash in the amount of {}", amt);
        }
        Payment::CreditCard(some_string, _) => {
            println!("Paying with credit card some string {}", some_string);
        }
        Payment::DebitCard(data) => {
            println!(
                "Paying with debit card_number {}, amount of {}",
                data.card_number, data.amount
            );
        }
        Payment::Crypto { account_id, amount } => {
            println!(
                "Paying with crypto account_id {}, amount of {}",
                account_id, amount
            );
        }
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

    let some_payment = Payment::Cash(100.);
    process_payment(some_payment);

    let cc_payment = Payment::CreditCard("CC 6666".to_string(), 300.);
    process_payment(cc_payment);

    let debit_payment = Payment::DebitCard(DebitData {
        card_number: "Debit 12234".to_string(),
        amount: 100.,
    });
    process_payment(debit_payment);

    let crypto_payment = Payment::Crypto {
        account_id: "aof".to_string(),
        amount: 100.,
    };
    process_payment(crypto_payment);
}
