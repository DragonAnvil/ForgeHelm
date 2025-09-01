// This file renders your main React component (App.jsx) into the root div

// Import React library
import React from 'react';
// Import ReactDOM for rendering
import ReactDOM from 'react-dom/client';
// Import the BrowserRouter component from the library
import { BrowserRouter } from 'react-router-dom';
// Import your main App component
import App from './App.jsx';
// Import Tailwind CSS styles
import './index.css';

// Find the HTML element with id 'root'
const rootElement = document.getElementById('root');

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode> {/* Helps catch common bugs in development */}
    <BrowserRouter> {/* Wrapping App in BrowserRouter allows all components inside App to utilize routing feature */}
      <App /> {/* Render your main App component */}
    </BrowserRouter>
  </React.StrictMode>,
)
