// src/components/Assets.jsx
import React from "react";

const Assets = ({ assets }) => {
  return (
    <section className="assets-section">
      <h3>📦 Your Assets</h3>
      <ul className="asset-list">
        {assets.map((asset, index) => (
          <li key={index}>
            <strong>{asset.name}</strong>: {asset.balance} {asset.symbol}
          </li>
        ))}
      </ul>
    </section>
  );
};

export default Assets;
