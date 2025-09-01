// This is the container for the dashboard and navbar
// index.html is shell with a #root div
// main.jsx renders your React app into the index.html root div
// App.jsx is your main dashboard layout, with collapsable sidebar
// The sidebar uses React state useState to toggle between collapsed and expanded

// Import React and useState hook
import React, { useState } from 'react'
import { Routes, Route } from 'react-router-dom';

// Import components
import HomeLayout from './components/HomeLayout';
import DashboardContainer from './components/DashboardContainer';
import WorkspaceTools from './components/WorkspaceTools';

// Import Tailwind CSS styles
import './index.css';
import './App.css'

// Main App component
function App() {
    // State to track if the navbar is collapsed or expanded
    const [collapsed, setCollapsed] = useState(false);
    // Function to toggle the collapsed state
    const toggleNavbar = () => {
        setCollapsed(!collapsed);
    };

    // Render the dashboard layout
    return (
        <div style={{ height: '100vh' }}>{/* Flex container for sidebar and main content */}

            <HomeLayout collapsed={collapsed} toggleNavbar={toggleNavbar} />

        </div>
        
    )
}

// Export the App component
export default App;