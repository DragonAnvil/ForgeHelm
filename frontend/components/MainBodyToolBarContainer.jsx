import React from 'react';

function MainBodyToolBarContainer({toggleAddNew}) {
    return (
        <div style={{
            width: '100%',
            display: 'flex',
            flexDirection: 'row',
            justifyContent: 'flex-start',
            alignItems: 'center',
            backgroundColor: '#07082B',
        }}>
            <div>
                <button 
                onClick={toggleAddNew}    
                style={{
                    padding: '5px',
                    marginTop: '10px',
                    marginBottom: '10px',
                    marginLeft: '5vw',
                    backgroundColor: '#e6e8fa',
                    color: '#07082B',
                    border: 'none',
                    borderRadius: '5px',
                    cursor: 'pointer',
                    marginRight: '10px',
                    fontFamily: 'MyFont, Disket-Mono-Regular, sans-serif'
                }}>
                    Add New
                </button>
            </div>
        </div>
    )
}

export default MainBodyToolBarContainer;