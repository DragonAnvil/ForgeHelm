// Imports React to file for use (required for all .jsx)
import React from 'react';

// Import UserLogo for use
import UserLogo from './UserLogo';
// Import Logo for use
import Logo from './Logo';

function HeaderContainer() {
    return (
        <div
            style={{
                display: 'flex',
                background: '#07082B', // Dark background
                width: '100%',
                top: '0',
                height: '5%',
                borderBottomStyle: 'solid',
                borderBottomWidth: '3px',
                borderBottomColor: '#032954', // Darker border color
                color: '#e6e8fa', // White text
            }}
            >
                <Logo />
                <UserLogo />
        </div>
    )
}

// Export this component for use in App.jsx
export default HeaderContainer