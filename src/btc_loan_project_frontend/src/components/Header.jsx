// src/components/Header.jsx
import React from "react";

const Header = ({ toggleTheme, theme, onAboutClick }) => {
  return (
    <header className="header">
      <div className="logo">
        <div className="logo-main">
          <span className="logo-white">Bitcoin</span>
          <span className="logo-orange">Loans</span>
        </div>
        <div className="logo-tagline">Your Bitcoin, Your Terms</div>
      </div>

      <nav className="nav">
        <a href="#connect">Connect Wallet</a>
        <a href="#dashboard">Dashboard</a>

        <button onClick={onAboutClick} className="about-button">
          About
        </button>

        <button onClick={toggleTheme} className="theme-toggle">
          {theme === "dark" ? "🌞" : "🌙"}
        </button>
      </nav>
    </header>
  );
};

export default Header;
