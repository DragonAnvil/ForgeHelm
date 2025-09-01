// Import React for use
import React from 'react';

// Import NavbarLeftUpperItems for use
import NavbarLeftUpperItems from './NavbarLeftUpperItems';
// Import NavbarLeftLowerItems for use
import NavbarLeftLowerItems from './NavbarLeftLowerItems';

// Export NavbarLeftContainer for use
function NavbarLeftContainer( {collapsed, toggleNavbar} ) {
    return (
        <div style={{
            width: '75%',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'stretch',
            height: '100%',
            background: '#020825'
        }}>
            <NavbarLeftUpperItems collapsed={ collapsed } />
            <NavbarLeftLowerItems collapsed={ collapsed } toggleNavbar={ toggleNavbar } />
        </div>
    )
}

// Export NavbarLeftContainer for use
export default NavbarLeftContainer;