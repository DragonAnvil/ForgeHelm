// Import React and useState hook
import React, { useState } from 'react'
// Import HeaderContainer
import HeaderContainer from './HeaderContainer';
import MainBodyToolBarContainer from './MainBodyToolBarContainer';
import MainBodyContainer from './MainBodyContainer';

function RightContainer() {

    // State to track if the "Add New" button is clicked
    const [addNew, setAddNew] = useState(false);
    // Function to toggle the "Add New" state
    const toggleAddNew = () => {
        setAddNew(!addNew);
        console.log("Add New button clicked, current state:", !addNew);
    }

    return (
        <div style={{display: 'flex', flexDirection: 'column', width: '100%', marginLeft: '0', marginRight: '0'}}>
            <HeaderContainer />
            <MainBodyToolBarContainer toggleAddNew={toggleAddNew} />
            <MainBodyContainer addNew={addNew} toggleAddNew={toggleAddNew} />
        </div>
    )
}

// Export RightContainer for use in App.jsx
export default RightContainer