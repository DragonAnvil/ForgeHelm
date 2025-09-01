// Import React
import React, { useState } from 'react';

import DashboardContainer from './DashboardContainer';
import WorkspaceTools from './WorkspaceTools';

function MainBodyContainer({addNew, toggleAddNew}) {

    // State to control visibility of WorkspaceTools component
    const [showWorkspaceTools, setShowWorkspaceTools] = useState(false);
    // Function to handle showing WorkspaceTools component
    const handleShowWorkspaceTools = (workspaceId) => {
        setSelectedWorkspaceId(workspaceId);
        setShowWorkspaceTools(true);
    };
    // State to relay specific workspace id
    const [ selectedWorkspaceId, setSelectedWorkspaceId ] = useState(null);

    return (
        <div style={{
            backgroundColor: '#07082B',
            width: '100vw',
            height: '100vh',
            display: 'flex',
            flexDirection: 'column',
            justifyContent: 'center',
            alignItems: 'center',
        }}>
            <div>
                {/* This is where the main body content will go */}

                {showWorkspaceTools ? (
                    <WorkspaceTools addNew={addNew} toggleAddNew={toggleAddNew} selectedWorkspaceId={selectedWorkspaceId}/>
                ) : (
                    <DashboardContainer addNew={addNew} toggleAddNew={toggleAddNew} showWorkspaceTools={handleShowWorkspaceTools} />
                )}
                
            </div>

        </div>
    )
}

// Export MainBodyContainer for use in App.jsx
export default MainBodyContainer;