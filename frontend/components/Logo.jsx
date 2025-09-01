import React from 'react';

function Logo() {
    return (
        <div style={{
            marginLeft: 'calc(100vw / 8)',
            marginTop: '0.5%',
            color: '#fff',
            fontSize: 'clamp(0.8rem, 2rem, 2rem)'
        }}>
            <div>
                /|/ FlowField \|\
            </div>
        </div>
    )
}

export default Logo;