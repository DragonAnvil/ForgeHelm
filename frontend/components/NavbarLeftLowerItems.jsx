// Import React for use
import React from 'react';

// 
function NavbarLeftLowerItems( { collapsed, toggleNavbar} ) {
    return (
        <div style={{
            marginTop: 'auto',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'flex-start',
            marginLeft: '20%',
            whiteSpace: 'nowrap'
        }}>
            <div style={{
                marginBottom: '20%',
                }}>
                <span role="img" aria-label="dashboard">📊</span>
                {!collapsed && <span style={{ marginLeft: '8px' }}>Themes</span>}
            </div>
            <div style={{
                marginBottom: '20%',
                }}>
                <span role="img" aria-label="dashboard">📊</span>
                {!collapsed && <span style={{ marginLeft: '8px' }}>Help</span>}
            </div>
            <button
                onClick={toggleNavbar} // Toggle collapsed state on click
                style={{
                    background: '#020825',
                    color: '#fff',
                    border: 'none',
                    cursor: 'pointer',
                    alignSelf: 'center',
                    paddingBottom: '10px',
                    width: '100%',
                }}
            >
                {collapsed ? '>' : '<'} {/* Show > when collapsed, < when expanded */}
            </button>
        </div>

    )
}

// Export NavbarLeftLowerContainer
export default NavbarLeftLowerItems;