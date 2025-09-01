// Import React for use
import React from 'react';

// 
function NavbarLeftUpperContainer( {collapsed} ) {

    const marginBottom = '20%';

    return (
        <div style={{
            display: 'flex',
            flexDirection: 'column',
            justifyContent: 'flex-start',
            alignItems: 'flex-start',
            marginTop: '125px',
            marginLeft: '20%',
            minHeight: '250px',
            background: '#020825',
            gap: collapsed ? '10px' : '10px',   // Change item gap based on collapsed state
        }}>
            <div style={{
                marginBottom: marginBottom,
            }}>
                <span role="img" aria-label="dashboard">📊</span>
                {!collapsed && <span style={{ marginLeft: '8px' }}>Workspaces</span>}
            </div>
            <div style={{
                marginBottom: marginBottom,
            }}>
                <span role="img" aria-label="dashboard">📊</span>
                {!collapsed && <span style={{ marginLeft: '8px' }}>Dashboards</span>}
            </div>
            <div style={{
                marginBottom: marginBottom,
            }}>
                <span role="img" aria-label="dashboard">📊</span>
                {!collapsed && <span style={{ marginLeft: '8px' }}>Projects</span>}
            </div>
            <div style={{
                marginBottom: marginBottom,
            }}>
                <span role="img" aria-label="dashboard">📊</span>
                {!collapsed && <span style={{ marginLeft: '8px' }}>Flows</span>}
            </div>
            <div style={{
                marginBottom: marginBottom,
            }}>
                <span role="img" aria-label="dashboard">📊</span>
                {!collapsed && <span style={{ marginLeft: '8px' }}>Forms</span>}
            </div>
            <div style={{
                marginBottom: marginBottom,
            }}>
                <span role="img" aria-label="dashboard">📊</span>
                {!collapsed && <span style={{ marginLeft: '8px' }}>Reports</span>}
            </div>
            <div style={{
                marginBottom: marginBottom,
            }}>
                <span role="img" aria-label="dashboard">📊</span>
                {!collapsed && <span style={{ marginLeft: '8px' }}>MTG Minutes</span>}
            </div>
            <div style={{
                marginBottom: marginBottom,
            }}>
                <span role="img" aria-label="dashboard">📊</span>
                {!collapsed && <span style={{ marginLeft: '8px' }}>RFIs</span>}
            </div>
            <div style={{
                marginBottom: marginBottom,
            }}>
                <span role="img" aria-label="dashboard">📊</span>
                {!collapsed && <span style={{ marginLeft: '8px' }}>DataLake</span>}
            </div>
        </div>
    )
}

// Export NavbarLeftUpperContainer for use
export default NavbarLeftUpperContainer;