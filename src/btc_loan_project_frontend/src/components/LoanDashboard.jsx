import React from "react";

const LoanDashboard = ({ loans, onRepayLoan }) => {
  return (
    <section className="loan-dashboard" id="dashboard">
      <h3>📊 Your Active Loans</h3>

      {loans.length === 0 ? (
        <p style={{ textAlign: "center", marginTop: "1rem", color: "#aaa" }}>
          No active loans yet.
        </p>
      ) : (
        loans.map((loan) => (
          <div className="loan-card" key={loan.id}>
            <p><strong>Loan ID:</strong> #{loan.id}</p>
            <p><strong>Collateral:</strong> {loan.collateral} BTC</p>
            <p><strong>Loan Amount:</strong> ${loan.loanAmount}</p>
            <p><strong>Remaining Term:</strong> {loan.term} days</p>
            <button
              className="repay-button"
              onClick={() => onRepayLoan(loan.id)}
            >
              Repay Loan
            </button>
          </div>
        ))
      )}
    </section>
  );
};

export default LoanDashboard;
