// src/components/ConnectWallet.jsx
import React from "react";

function ConnectWallet({ walletAddress, onConnect, onDisconnect }) {
  const handleConnect = async () => {
    try {
      if (!window.ic || !window.ic.plug) {
        alert("Please install the Plug Wallet extension.");
        return;
      }

      const connected = await window.ic.plug.requestConnect();

      if (connected) {
        const principal = await window.ic.plug.getPrincipal();
        onConnect(principal);
      } else {
        alert("Wallet connection failed.");
      }
    } catch (error) {
      console.error("Wallet connect error:", error);
      alert("Something went wrong while connecting your wallet.");
    }
  };

  return (
    <section className="connect-wallet" id="connect">
      <h3>🔐 Connect Your Wallet</h3>
      {walletAddress ? (
        <div className="wallet-status">
          <p className="connected">
            ✅ Wallet Connected:
            <br />
            <span className="wallet-address">{walletAddress}</span>
          </p>
          <button className="disconnect-button" onClick={onDisconnect}>
            Disconnect
          </button>
        </div>
      ) : (
        <button className="connect-button" onClick={handleConnect}>
          Connect Plug Wallet
        </button>
      )}
    </section>
  );
}

export default ConnectWallet;
