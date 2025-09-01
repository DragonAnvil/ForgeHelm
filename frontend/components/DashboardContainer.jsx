import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import React from 'react';

function DashboardContainer({addNew, toggleAddNew, showWorkspaceTools}) {

    // Variable for Header Cell Styling
    const headerCellStyle = {
        padding: '10px',
        textAlign: 'center',
        width: '15%',
        borderBottomStyle: 'solid',
        borderBottomWidth: '1px',
        borderBottomColor: '#e6e8fa',
        //borderRightStyle: 'solid',
        //borderRightWidth: '1px',
        //borderRightColor: '#e6e8fa',
    }
    // Variable for Header Button Styling
    const headerButtonStyle = {
        padding: '10px',
        textAlign: 'center',
        width: '1%',
        borderBottomStyle: 'solid',
        borderBottomWidth: '1px',
        borderBottomColor: '#e6e8fa',
    }

    // Variable for Row Cell Styling
    const rowCellStyle = {
        padding: '5px',
        textAlign: 'center',
        //borderRightStyle: 'solid',
        //borderRightWidth: '1px',
        //borderRightColor: '#e6e8fa',
    }

    // Variable for Row Button Styling
    const rowButtonStyle = {
        padding: '10px',
        textAlign: 'center',
    }

    // Variable for styling input row fields
    const inputRowStyle = {
        padding: '5px',
        borderStyle: 'solid',
        bordersize: '1px',
        borderColor: '#e6e8fa',
        borderRadius: '5px',
        backgroundColor: '#07082B',
        color: '#e6e8fa'
    }


    // State to hold the columns of the workspaces table
    const [ columns, setColumns ] = useState([]);

    // State to hold all workspace data fetched from the backend
    const [ workspacesData, setWorkspacesData ] = useState([]);

    const navigate = useNavigate();

    // Fetch workspace data from backend when component mounts
    useEffect(() => {
        fetch('http://localhost:8080/items')
        .then(res => res.json())
        .then(data => setWorkspacesData(data))
        .catch(error => console.error('Error fetching items:', error));
    }, [])


    // State for input fields in the "add new workspace" row
    const [workspace, setWorkSpace] = useState('');
    const [description, setDescription] = useState('');
    const [owner, setOwner] = useState('');

    // Boolean to check if all input fields are filled
    const allFilled = workspace.trim() && description.trim() && owner.trim();

    // State for tracking which cell is being edited
    const [editingCell, setEditingCell] = useState(null);
    
    // State for storing values being edited (not strictly needed here)
    const [editValues, setEditValues] = useState({});

    // Periodically refresh workspace data every 10 seconds, unless editing a cell
    useEffect(() => {

        if(workspacesData.length === 0) {
            fetch('http://localhost:3000/api/workspaces/columns')
            .then(res => res.json())
            .then(data => setColumns(data))
            .catch(error => console.error('Error fetching workspaces header:', error));
        }

        if (workspacesData.length != 0) {
            const interval = setInterval(() => {
                // Refresh data every 10 seconds
                fetch('http://localhost:3000/api/workspaces')
                .then(res => res.json())
                .then(data => {
                    // Only update if not currently editing
                    if (!editingCell) {
                        setWorkspacesData(data);
                    }
                });
            }, 10000); // 10 seconds
            return () => clearInterval(interval);
        }
        
    }, [editingCell]);        



    // Function to handle Enter Key Submit
    const handleKeyDown = (event) => {

        // Input Row Logic
        if (event.key === 'Enter' && allFilled) {
            // DEBUG Statement
            console.log('New Workspace POST Request Ready to be sent');



            // Immediately update UI
            // Temp Timestamp
            const tempTime = new Date();
            const formattedTimestamp = tempTime.toLocaleString('en-US', {
                hour: '2-digit',
                minute: '2-digit',
                month: '2-digit',
                day: '2-digit',
                year: 'numeric'
            }).replace(',','');
            
            const newWorkspace = {
                name: workspace,
                description: description,
                owner: owner,
                created_at: formattedTimestamp,
                updated_at: formattedTimestamp,
                // need tempKey because the row is being manually added temporarily
                tempKey: Date.now() + Math.random() // unique id for this session
            };
            setWorkspacesData(prev => [...prev, newWorkspace]);
            
            // POST Request
            fetch('http://localhost:3000/api/workspaces', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(newWorkspace)
            })
            .then(response => response.json())
            .then(data => {
                console.log('Workspace added:', data);

                // Optionally replace the last item with the real data (with id)
                setWorkspacesData(prev => {
                    const updated = [...prev];
                    updated[updated.length - 1] = data;
                return updated;
                });

                // Optional to clear inputs or update UI here
                setWorkSpace('');
                setDescription('');
                
                setOwner('');
                // Collapse the input row after submission
                toggleAddNew(false);
            })
            .catch(error => {
                console.error('Error creating workspace:', error);
                // If request fails, remove the optimistically added row
                setWorkspacesData(prev => prev.slice(0, -1));
            });
            event.preventDefault(),
            event.target.blur() // Trigger onBlur to save changes
        }
    }

    // Handle Cell Edit
    const handleCellEdit = async (rowIndex, field, newValue) => {
        const workspace = workspacesData[rowIndex];
        const originalValue = workspace[field];

        // If the cell value is changed, update the cell
        if (newValue !== originalValue) {
            try {
                // Temp Timestamp
                const tempTime = new Date();
                const formattedTimestamp = tempTime.toLocaleString('en-US', {
                    hour: '2-digit',
                    minute: '2-digit',
                    month: '2-digit',
                    day: '2-digit',
                    year: 'numeric'
                }).replace(',','');

                // Update UI
                const updatedData = [...workspacesData];
                updatedData[rowIndex] = {...workspace, [field]: newValue, updated_at: formattedTimestamp};
                setWorkspacesData(updatedData);

                console.log(`newValue: ${newValue}`)
                // Send PUT request to backend to update cell value in database table
                await fetch('http://localhost:3000/api/workspaces', {
                    method: 'PUT',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({
                        id: workspace.id, // Need to identify which row to update on the workspaces table
                        [field]: newValue
                    })
                });
            } catch (error) {
                console.error('Error updating workspace:', error);
                // Revert if error occurs
                const revertedData = [...workspacesData];
                revertedData[rowIndex] = workspace;
                setWorkspacesData(revertedData);
            }
        }
        setEditingCell(null);
    };

    // 
    const handleCellKeyDown = (event, rowIndex, field) => {
        if(event.key === 'Enter') {
            event.preventDefault();
            event.target.blur(); // Trigger onBlur to save changes
        } else if (event.key === 'Escape') {
            event.preventDefault();
            // Revert to original value if Escape is pressed
            const originalValue = workspacesData[rowIndex][field];
            event.target.textContent = originalValue;
            event.target.blur();
        }
    };

    function deleteWorkspace(id) {
        // Send DELETE request to backend to delete workspace
        fetch('http://localhost:3000/api/workspaces', {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ id })
        })
        .then(response => response.json())
        .then(data => {
            console.log('Workspace deleted:', data);
            // Remove the deleted workspace from the UI
            setWorkspacesData(prev => prev.filter(workspace => workspace.id !== id));
        });
    }

    // Variable to declare which columns are not editable from frontend by user
    const uneditableWorkspacesColumns = ['id', 'created_at', 'updated_at'];

    return (
        <div style={{
            width: '90vw',
            height: '90vh',
           // borderStyle: 'solid',
           // borderWidth: '2px',
            //borderColor: '#032954', // Darker border color
            backgroundColor: '#07082B', // Dark background
            color: '#e6e8fa', // Eggwhite text rgba(255, 158, 0, 0.8)
            }}
        >
            <table style={{
                borderStyle: 'solid',
                borderWidth: '1px',
                borderColor: '#e6e8fa', // Darker border color
                marginLeft: 'auto',
                marginRight: 'auto',
                width: '100%',
                borderCollapse: 'collapse', // This merges adjacent border lines to avoid gaps beteween cells
            }}>
                <thead>
                    <tr>
                        <th style={headerButtonStyle}>
                        </th>
                        {columns.map(column => (
                            <th key={column} style={headerCellStyle}>
                                {column.charAt(0).toUpperCase() + column.slice(1)}
                            </th>
                        ))}
                        <th style={headerButtonStyle}>
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {addNew && 
                    <tr>
                        <td style={rowCellStyle}></td>
                        <td style={rowCellStyle}><input style={inputRowStyle} className="inputRowWorkspaces" value={workspace} onChange={event => setWorkSpace(event.target.value)} onKeyDown={handleKeyDown} required /></td>
                        <td style={rowCellStyle}><input style={inputRowStyle} className="inputRowWorkspaces" value={description} onChange={event => setDescription(event.target.value)} onKeyDown={handleKeyDown} required /></td>
                        <td style={rowCellStyle}><input style={inputRowStyle} className="inputRowWorkspaces" value={owner} onChange={event => setOwner(event.target.value)} onKeyDown={handleKeyDown} required /></td>
                        <td style={rowCellStyle}></td>
                        <td style={rowCellStyle}></td>
                        <td style={rowCellStyle}></td>
                    </tr>
                    }
                    {workspacesData.map((row, rowIndex) => ( // Loops through each row in database table, rowIndex is position in array
                        <tr key={row.id ? `id-${row.id}` : `temp-${row.tempKey}`}>
                            
                            <td style={rowButtonStyle}>
                                <button onClick={() => showWorkspaceTools(row.id)} style={{backgroundColor: '#e6e8fa', color: '#ff9e00', padding: '5px 10px', borderRadius: '5px', border: 'none'}}/>
                            </td>
                            {columns.map(column => (    // Loop through columns arrray for each row
                                <td
                                    key={column}
                                    style={rowCellStyle}
                                    contentEditable={!uneditableWorkspacesColumns.includes(column)}
                                    suppressContentEditableWarning={true}
                                    ref={el => {    // Set spellcheck to false for contentEditable cells
                                        if (el) el.setAttribute('spellcheck', 'false');
                                    }}
                                    onFocus={() => setEditingCell({rowIndex, field: column})}
                                    onBlur={(e) => handleCellEdit(rowIndex, column, e.target.textContent)}
                                    onKeyDown={(e) => handleCellKeyDown(e, rowIndex, column)}
                                >
                                    {row[column]}
                                </td>
                            ))}
                            <td style={rowButtonStyle}>
                                <button onClick={() => deleteWorkspace(row.id)} style={{backgroundColor: '#E4080A', color: '#ff9e00', padding: '5px 10px', borderRadius: '5px', border: 'none'}}/>
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>

        </div>
    )
}

export default DashboardContainer;
