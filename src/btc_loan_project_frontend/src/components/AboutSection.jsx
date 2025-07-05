// src/components/AboutSection.jsx
import React from "react";
import "./AboutSection.css";

const AboutSection = ({ onClose }) => {
  return (
    <div className="about-slide-in">
      <button className="close-about" onClick={onClose}>✖</button>
      <h2><br></br><br></br>About This Platform</h2>
      <p>
        This platform allows users to borrow stablecoins using Bitcoin as collateral,
        powered by decentralized finance (DeFi) and secured with Plug Wallet on Internet Computer.
      </p>
      <p>
        It's a fast, secure, and user-friendly way to unlock liquidity from your BTC without selling it.
      </p>
    </div>
  );
};

export default AboutSection;
