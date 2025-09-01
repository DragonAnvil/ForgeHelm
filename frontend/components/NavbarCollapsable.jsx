// Import React and useState hook
import React from 'react';

// Import NavbarLeftContainer for use
import NavbarLeftContainer from './NavbarLeftContainer';

function NavbarCollapsable({ collapsed, toggleNavbar }) {
    return (
        <nav 
            style={{
                position: 'fixed',
                top: '0',
                left: '0',
                height: '100vh',
                width: collapsed ? '60px' : '200px', // Change width based on collapsed state
                background: '#07082B', // Dark background
                color: '#e6e8fa', // White text
                transition: 'width 0.3s', // Smooth transition
                borderRight: '2px solid #032954', // Darker border color
                display: 'flex',
                flexDirection: 'row',
                alignItems: 'stretch',
                justifyContent: 'space-evenly',
                whiteSpace: 'nowrap',
                fontSize: 'clamp(0.1rem, 0.8rem, 1.2rem)', // Responsive font size
            }}
        >

            <NavbarLeftContainer collapsed={ collapsed } toggleNavbar={ toggleNavbar } />
           
            {/* Example icon or menu item */}

            {/* Add more menu items here */}
        </nav>
    );
}

export default NavbarCollapsable;