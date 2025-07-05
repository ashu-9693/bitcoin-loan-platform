// src/components/LoanForm.jsx
import React, { useState } from "react";

function LoanForm({ onSubmitLoan }) {
  const [collateral, setCollateral] = useState("");
  const [loanAmount, setLoanAmount] = useState("");
  const [term, setTerm] = useState("");

  const BTC_PRICE = 90000; // assumed price of 1 BTC
  const MAX_LTV = 0.6; // 60% loan-to-value

  const parsedCollateral = parseFloat(collateral) || 0;
  const maxLoanAllowed = parsedCollateral * BTC_PRICE * MAX_LTV;

  const handleSubmit = (e) => {
    e.preventDefault();

    if (!collateral || !loanAmount || !term) {
      alert("Please fill in all fields.");
      return;
    }

    if (parseFloat(loanAmount) > maxLoanAllowed) {
      alert(`You can only borrow up to $${maxLoanAllowed.toFixed(2)} for ${collateral} BTC.`);
      return;
    }

    const loan = {
      collateral: parsedCollateral,
      loanAmount: parseFloat(loanAmount),
      term: parseInt(term),
    };

    onSubmitLoan(loan);
    setCollateral("");
    setLoanAmount("");
    setTerm("");
  };

  return (
    <section className="loan-form" id="loan-form">
      <h3>📝 Apply for a Bitcoin-Backed Loan</h3>
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label>Bitcoin Collateral (BTC):</label>
          <input
            type="number"
            step="0.01"
            placeholder="e.g. 0.5"
            value={collateral}
            onChange={(e) => setCollateral(e.target.value)}
          />
          {parsedCollateral > 0 && (
            <p className="helper-text">
              💡 You can borrow up to <strong>${maxLoanAllowed.toFixed(2)}</strong> based on current rates.
            </p>
          )}
        </div>

        <div className="form-group">
          <label>Loan Amount (USD):</label>
          <input
            type="number"
            placeholder="e.g. 1000"
            value={loanAmount}
            onChange={(e) => setLoanAmount(e.target.value)}
          />
        </div>

        <div className="form-group">
          <label>Loan Term (Days):</label>
          <input
            type="number"
            placeholder="e.g. 30"
            value={term}
            onChange={(e) => setTerm(e.target.value)}
          />
        </div>

        <button className="loan-button" type="submit">Request Loan</button>
      </form>
    </section>
  );
}

export default LoanForm;
