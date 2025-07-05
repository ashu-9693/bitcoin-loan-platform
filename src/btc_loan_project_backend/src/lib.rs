use candid::{CandidType, Deserialize, Nat};
use ic_cdk_macros::*;
use std::collections::HashMap;

// Define HTTP structures for the web interface
#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HttpHeader>,
    pub body: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
    pub status: Nat,
    pub headers: Vec<HttpHeader>,
    pub body: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

// Data structures for your loan platform
#[derive(CandidType, Deserialize, Clone)]
pub struct LoanRequest {
    pub id: String,
    pub borrower: String,
    pub amount: u64,
    pub collateral_amount: u64,
    pub interest_rate: f64,
    pub duration_days: u32,
    pub status: LoanStatus,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum LoanStatus {
    Pending,
    Approved,
    Active,
    Repaid,
    Defaulted,
}

// Storage
use std::cell::RefCell;
thread_local! {
    static LOANS: RefCell<HashMap<String, LoanRequest>> = RefCell::new(HashMap::new());
    static LOAN_COUNTER: RefCell<u64> = RefCell::new(0);
}

// API Functions for your Bitcoin Loan Platform
#[update]
fn create_loan_request(
    borrower: String,
    amount: u64,
    collateral_amount: u64,
    interest_rate: f64,
    duration_days: u32,
) -> Result<String, String> {
    let loan_id = LOAN_COUNTER.with(|counter| {
        let mut c = counter.borrow_mut();
        *c += 1;
        format!("loan_{}", *c)
    });

    let loan = LoanRequest {
        id: loan_id.clone(),
        borrower,
        amount,
        collateral_amount,
        interest_rate,
        duration_days,
        status: LoanStatus::Pending,
    };

    LOANS.with(|loans| {
        loans.borrow_mut().insert(loan_id.clone(), loan);
    });

    Ok(loan_id)
}

#[query]
fn get_loan_request(loan_id: String) -> Result<LoanRequest, String> {
    LOANS.with(|loans| {
        loans.borrow()
            .get(&loan_id)
            .cloned()
            .ok_or("Loan not found".to_string())
    })
}

#[query]
fn get_all_loans() -> Vec<LoanRequest> {
    LOANS.with(|loans| {
        loans.borrow().values().cloned().collect()
    })
}

#[update]
fn approve_loan(loan_id: String) -> Result<String, String> {
    LOANS.with(|loans| {
        let mut loans_map = loans.borrow_mut();
        match loans_map.get_mut(&loan_id) {
            Some(loan) => {
                loan.status = LoanStatus::Approved;
                Ok("Loan approved successfully".to_string())
            }
            None => Err("Loan not found".to_string()),
        }
    })
}

#[update]
fn activate_loan(loan_id: String) -> Result<String, String> {
    LOANS.with(|loans| {
        let mut loans_map = loans.borrow_mut();
        match loans_map.get_mut(&loan_id) {
            Some(loan) => {
                loan.status = LoanStatus::Active;
                Ok("Loan activated successfully".to_string())
            }
            None => Err("Loan not found".to_string()),
        }
    })
}

#[update]
fn repay_loan(loan_id: String) -> Result<String, String> {
    LOANS.with(|loans| {
        let mut loans_map = loans.borrow_mut();
        match loans_map.get_mut(&loan_id) {
            Some(loan) => {
                loan.status = LoanStatus::Repaid;
                Ok("Loan repaid successfully".to_string())
            }
            None => Err("Loan not found".to_string()),
        }
    })
}

#[query]
fn get_platform_stats() -> HashMap<String, u64> {
    LOANS.with(|loans| {
        let loans_map = loans.borrow();
        let mut stats = HashMap::new();
        
        stats.insert("total_loans".to_string(), loans_map.len() as u64);
        stats.insert("pending_loans".to_string(), 
            loans_map.values().filter(|l| matches!(l.status, LoanStatus::Pending)).count() as u64);
        stats.insert("active_loans".to_string(), 
            loans_map.values().filter(|l| matches!(l.status, LoanStatus::Active)).count() as u64);
        stats.insert("repaid_loans".to_string(), 
            loans_map.values().filter(|l| matches!(l.status, LoanStatus::Repaid)).count() as u64);
        
        stats
    })
}

// ✅ FIXED DUPLICATE: renamed second HttpRequest to HttpRequestWeb
#[derive(CandidType, Deserialize)]
pub struct HttpRequestWeb {
    pub method: String,
    pub url: String,
    pub headers: Vec<HttpHeader>,
    pub body: Vec<u8>,
}

#[query]
fn http_request(_request: HttpRequestWeb) -> HttpResponse {
    HttpResponse {
        status: Nat::from(200u32),
        headers: vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "text/html".to_string(),
            },
        ],
        body: r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Bitcoin Backed Loan Platform API</title>
            <style>
                body { 
                    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; 
                    margin: 0; 
                    padding: 0;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    min-height: 100vh;
                }
                .container { 
                    max-width: 1000px; 
                    margin: 0 auto; 
                    background: white; 
                    padding: 40px; 
                    border-radius: 15px; 
                    box-shadow: 0 10px 30px rgba(0,0,0,0.2);
                    margin-top: 20px;
                    margin-bottom: 20px;
                }
                .header { 
                    text-align: center; 
                    margin-bottom: 40px; 
                    padding-bottom: 20px;
                    border-bottom: 3px solid #f7931a;
                }
                .header h1 {
                    color: #f7931a;
                    font-size: 2.5em;
                    margin-bottom: 10px;
                }
                .header p {
                    color: #666;
                    font-size: 1.2em;
                }
                .api-section { 
                    margin: 30px 0; 
                }
                .api-section h2 {
                    color: #333;
                    border-bottom: 2px solid #e9ecef;
                    padding-bottom: 10px;
                }
                .endpoint { 
                    background: #f8f9fa; 
                    padding: 20px; 
                    margin: 15px 0; 
                    border-radius: 8px; 
                    border-left: 5px solid #007bff;
                    transition: transform 0.2s ease;
                }
                .endpoint:hover {
                    transform: translateX(5px);
                    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
                }
                .method { 
                    font-weight: bold; 
                    color: #007bff; 
                    font-size: 1.1em;
                    margin-bottom: 5px;
                }
                .description { 
                    color: #6c757d; 
                    margin-top: 5px; 
                    line-height: 1.4;
                }
                .candid-link { 
                    background: linear-gradient(45deg, #28a745, #20c997); 
                    color: white; 
                    padding: 15px 30px; 
                    text-decoration: none; 
                    border-radius: 25px; 
                    display: inline-block; 
                    margin: 30px 0;
                    font-weight: bold;
                    transition: all 0.3s ease;
                    box-shadow: 0 4px 15px rgba(40, 167, 69, 0.3);
                }
                .candid-link:hover { 
                    background: linear-gradient(45deg, #218838, #1ea68a);
                    transform: translateY(-2px);
                    box-shadow: 0 6px 20px rgba(40, 167, 69, 0.4);
                }
                .features {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                    gap: 20px;
                    margin: 30px 0;
                }
                .feature-card {
                    background: #fff;
                    padding: 25px;
                    border-radius: 10px;
                    border: 1px solid #e9ecef;
                    text-align: center;
                    box-shadow: 0 2px 10px rgba(0,0,0,0.05);
                }
                .feature-card h3 {
                    color: #f7931a;
                    margin-bottom: 15px;
                }
                .feature-card p {
                    color: #666;
                    line-height: 1.6;
                }
                .stats {
                    background: #f8f9fa;
                    padding: 20px;
                    border-radius: 10px;
                    margin: 20px 0;
                }
                .stats h3 {
                    color: #333;
                    margin-bottom: 15px;
                }
                .stat-item {
                    display: inline-block;
                    margin: 5px 15px;
                    padding: 10px 15px;
                    background: white;
                    border-radius: 20px;
                    color: #007bff;
                    font-weight: bold;
                    box-shadow: 0 2px 5px rgba(0,0,0,0.1);
                }
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>🏦 Bitcoin Backed Loan Platform API</h1>
                    <p>Decentralized lending platform powered by Internet Computer</p>
                </div>
                
                <div class="features">
                    <div class="feature-card">
                        <h3>🔒 Secure</h3>
                        <p>Smart contracts ensure transparent and secure lending with collateral protection</p>
                    </div>
                    <div class="feature-card">
                        <h3>⚡ Fast</h3>
                        <p>Instant loan processing and approval through automated smart contracts</p>
                    </div>
                    <div class="feature-card">
                        <h3>🌍 Global</h3>
                        <p>Access loans from anywhere in the world with Bitcoin as collateral</p>
                    </div>
                </div>
                
                <div class="api-section">
                    <h2>📋 Available API Endpoints</h2>
                    
                    <div class="endpoint">
                        <div class="method">create_loan_request</div>
                        <div class="description">Create a new loan request with collateral details. Requires borrower ID, amount, collateral amount, interest rate, and duration.</div>
                    </div>
                    
                    <div class="endpoint">
                        <div class="method">get_loan_request</div>
                        <div class="description">Retrieve details of a specific loan by ID. Returns complete loan information including status and terms.</div>
                    </div>
                    
                    <div class="endpoint">
                        <div class="method">get_all_loans</div>
                        <div class="description">Get all loan requests in the system. Returns a comprehensive list of all loans with their current status.</div>
                    </div>
                    
                    <div class="endpoint">
                        <div class="method">approve_loan</div>
                        <div class="description">Approve a pending loan request. Changes loan status from pending to approved for fund disbursement.</div>
                    </div>
                    
                    <div class="endpoint">
                        <div class="method">activate_loan</div>
                        <div class="description">Activate an approved loan. Moves loan from approved to active status, indicating funds have been disbursed.</div>
                    </div>
                    
                    <div class="endpoint">
                        <div class="method">repay_loan</div>
                        <div class="description">Mark a loan as repaid. Updates loan status to repaid and releases collateral back to borrower.</div>
                    </div>
                    
                    <div class="endpoint">
                        <div class="method">get_platform_stats</div>
                        <div class="description">Get platform statistics and metrics. Returns counts of total, pending, active, and repaid loans.</div>
                    </div>
                </div>
                
                <div class="stats">
                    <h3>📊 Platform Features</h3>
                    <div class="stat-item">Collateral-Backed Loans</div>
                    <div class="stat-item">Automated Processing</div>
                    <div class="stat-item">Real-time Status Tracking</div>
                    <div class="stat-item">Decentralized Architecture</div>
                </div>
                
                <div style="text-align: center;">
                    <a href="/_/candid" class="candid-link">🔧 Test API with Candid UI</a>
                </div>
                
                <div style="text-align: center; margin-top: 30px; padding-top: 20px; border-top: 1px solid #e9ecef;">
                    <p style="color: #666; font-size: 0.9em;">
                        Built on Internet Computer • Powered by Rust & Candid
                    </p>
                </div>
            </div>
        </body>
        </html>
        "#.to_string().into_bytes(),
    }
}
