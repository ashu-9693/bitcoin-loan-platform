import React, { useState, useEffect } from "react";
import Header from "./components/Header";
import HeroSection from "./components/HeroSection";
import ConnectWallet from "./components/ConnectWallet";
import LoanForm from "./components/LoanForm";
import LoanDashboard from "./components/LoanDashboard";
import Footer from "./components/Footer";
import AboutSection from "./components/AboutSection";
import "./styles.css";

const quotes = [
  "Your Bitcoin, your future 💡",
  "HODL your BTC, borrow smart 💰",
  "Decentralized. Transparent. Secure 🔒",
  "Let your Bitcoin work for you 🚀",
  "No banks. Just code. 🧠"
];

function App() {
  const [walletAddress, setWalletAddress] = useState(null);
  const [loans, setLoans] = useState([]);
  const [theme, setTheme] = useState("dark");
  const [showAbout, setShowAbout] = useState(false);
  const [quoteIndex, setQuoteIndex] = useState(0);
  const [showQuote, setShowQuote] = useState(true);

  // Theme toggle
  const toggleTheme = () => {
    setTheme((prev) => (prev === "light" ? "dark" : "light"));
  };

  // About toggle
  const toggleAbout = () => {
    setShowAbout((prev) => !prev);
  };

  // Wallet connect
  const connectWallet = (principal) => {
    setWalletAddress(principal.toText());
  };

  // Wallet disconnect
  const disconnectWallet = () => {
    setWalletAddress(null);
  };

  // Add new loan
  const handleNewLoan = (loan) => {
    const newLoan = {
      ...loan,
      id: Date.now(),
      owner: walletAddress,
    };
    setLoans([...loans, newLoan]);
  };

  // Repay loan
  const handleRepayLoan = (loanId) => {
    const updatedLoans = loans.filter((loan) => loan.id !== loanId);
    setLoans(updatedLoans);
  };

  // Quote rotation
  useEffect(() => {
    const interval = setInterval(() => {
      setQuoteIndex((prev) => (prev + 1) % quotes.length);
      setShowQuote(true);

      setTimeout(() => {
        setShowQuote(false);
      }, 20000);
    }, 8000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className={`App ${theme}`}>
      <Header toggleTheme={toggleTheme} theme={theme} onAboutClick={toggleAbout} />

      {showAbout && (
        <div className="about-overlay">
          <AboutSection onClose={toggleAbout} />
        </div>
      )}

      <div className="container">
        <HeroSection />

        <ConnectWallet
          walletAddress={walletAddress}
          onConnect={connectWallet}
          onDisconnect={disconnectWallet}
        />

        {walletAddress ? (
          <>
            <LoanForm onSubmitLoan={handleNewLoan} />
            <LoanDashboard loans={loans} onRepayLoan={handleRepayLoan} />
          </>
        ) : (
          <p style={{ textAlign: "center", marginTop: "2rem" }}>
            Please connect your Plug Wallet to access loan features.
          </p>
        )}
      </div>

      {/* Sliding Quote Box */}
      {showQuote && (
        <div className="quote-box slide-in-left">
          {quotes[quoteIndex]}
        </div>
      )}

      <Footer />
    </div>
  );
}

export default App;
