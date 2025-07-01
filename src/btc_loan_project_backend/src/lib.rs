use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Loan {
    pub id: u64,
    pub borrower: Principal,
    pub btc_collateral_amount: u64, // in satoshis
    pub loan_amount_usd: u64,       // in cents
    pub interest_rate: f64,         // annual percentage
    pub duration_days: u32,
    pub status: LoanStatus,
    pub created_at: u64,
    pub btc_address: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum LoanStatus {
    Pending,
    Active,
    Repaid,
    Liquidated,
    Defaulted,
}

thread_local! {
    static LOANS: RefCell<HashMap<u64, Loan>> = RefCell::new(HashMap::new());
    static LOAN_COUNTER: RefCell<u64> = RefCell::new(0);
}

#[update]
fn create_loan_request(
    btc_collateral_amount: u64,
    loan_amount_usd: u64,
    interest_rate: f64,
    duration_days: u32,
) -> u64 {
    let caller = ic_cdk::caller();
    
    let loan_id = LOAN_COUNTER.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter += 1;
        *counter
    });

    let loan = Loan {
        id: loan_id,
        borrower: caller,
        btc_collateral_amount,
        loan_amount_usd,
        interest_rate,
        duration_days,
        status: LoanStatus::Pending,
        created_at: time(),
        btc_address: format!("tb1q{}", loan_id), // Placeholder testnet address
    };

    LOANS.with(|loans| {
        loans.borrow_mut().insert(loan_id, loan);
    });

    loan_id
}

#[query]
fn get_loan(loan_id: u64) -> Option<Loan> {
    LOANS.with(|loans| loans.borrow().get(&loan_id).cloned())
}

#[query]
fn get_all_loans() -> Vec<Loan> {
    LOANS.with(|loans| loans.borrow().values().cloned().collect())
}

#[update]
fn approve_loan(loan_id: u64) -> bool {
    LOANS.with(|loans| {
        let mut loans = loans.borrow_mut();
        if let Some(loan) = loans.get_mut(&loan_id) {
            if matches!(loan.status, LoanStatus::Pending) {
                loan.status = LoanStatus::Active;
                return true;
            }
        }
        false
    })
}

#[update]
fn repay_loan(loan_id: u64) -> bool {
    let caller = ic_cdk::caller();
    
    LOANS.with(|loans| {
        let mut loans = loans.borrow_mut();
        if let Some(loan) = loans.get_mut(&loan_id) {
            if loan.borrower == caller && matches!(loan.status, LoanStatus::Active) {
                loan.status = LoanStatus::Repaid;
                return true;
            }
        }
        false
    })
}

ic_cdk::export_candid!();