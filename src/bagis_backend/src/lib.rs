use ic_cdk::export::{candid::CandidType, Principal};
use std::collections::HashMap;

// User data structure
#[derive(Debug, CandidType)]
struct User {
    username: String,
    password: String,
    email: String,
    balance: u32,
}

// Donation data structure
#[derive(Debug, CandidType)]
struct Donation {
    from_user: Principal,
    amount: u32,
}

// Donation service
#[derive(Default)]
struct DonationService {
    users: HashMap<Principal, User>,
    donations: Vec<Donation>,
}

impl DonationService {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            donations: Vec::new(),
        }
    }

    // Function to add a new user
    fn add_user(&mut self, caller: Principal, username: String, password: String, email: String) {
        let user = User {
            username,
            password,
            email,
            balance: 0,
        };
        self.users.insert(caller, user);
    }

    // Function to authenticate a user
    fn authenticate_user(&self, caller: Principal, password: String) -> Option<&User> {
        if let Some(user) = self.users.get(&caller) {
            if user.password == password {
                Some(user)
            } else {
                None
            }
        } else {
            None
        }
    }

    // Function to make a donation
    fn make_donation(&mut self, caller: Principal, amount: u32) -> Result<(), &'static str> {
        if let Some(user) = self.users.get_mut(&caller) {
            if user.balance >= amount {
                user.balance -= amount;
                self.donations.push(Donation {
                    from_user: caller,
                    amount,
                });
                Ok(())
            } else {
                Err("Insufficient balance.")
            }
        } else {
            Err("User not found.")
        }
    }
}

#[ic_cdk_macros::main]
async fn main() {
    // Create a donation service
    let mut donation_service = DonationService::new();

    // Add users
    let caller1: Principal = ic_cdk::caller();
    donation_service.add_user(caller1, "alice".to_string(), "alice123".to_string(), "alice@example.com".to_string());

    let caller2: Principal = ic_cdk::caller();
    donation_service.add_user(caller2, "bob".to_string(), "bob123".to_string(), "bob@example.com".to_string());

    // User authentication
    let caller: Principal = ic_cdk::caller();
    match donation_service.authenticate_user(caller, "alice123".to_string()) {
        Some(user) => {
            println!("Authenticated User: {:?}", user);

            // Make a donation
            if let Err(err) = donation_service.make_donation(caller, 100) {
                println!("Donation failed: {}", err);
            } else {
                println!("Donation successful!");
            }
        }
        None => {
            println!("User authentication failed.");
        }
    }
}
