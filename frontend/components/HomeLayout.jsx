// Import React
import React from 'react';

// Import NavbarCollapsable
import NavbarCollapsable from './NavbarCollapsable';
// Import HeaderContainer
import HeaderContainer from './HeaderContainer';
import RightContainer from './RightContainer';

function HomeLayout({collapsed, toggleNavbar}) {
    return (
        
        // '<>' is a Fragment, lets you join elements without grouping inside a <div> etc
        <div style={{ 
            display: 'flex',
            flexDirection: 'row',
            flex: '1',
            height: '100%',
            margin: '0',
            width: '100%'
        }}>
            <NavbarCollapsable collapsed={collapsed} toggleNavbar={toggleNavbar} />
            <RightContainer />
        </div>
    )
}

// Export HomeLayout component for use in App.jsx
export default HomeLayout;