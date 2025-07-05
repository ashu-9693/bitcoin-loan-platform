import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './index.css'; // this file should now exist

createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
