import { useState, useEffect } from "react";
import React from 'react';

function WorkspaceTools({addNew, toggleAddNew}) {

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
    const [ workspaceToolData, setWorkspaceToolData ] = useState([]);

    // useEffect to fetch workspace tools when component mounts
    useEffect(() => {
        fetch('http://localhost:3000/api/workspace_tools')
        .then(res => res.json())
        .then(data => setWorkspaceToolData(data))
        .catch(error => console.error('Error fetching Workspace Tools:', error));
    }, [])

    // State for input fields in the "add new workspace tool" row
    const [workspaceTool, setWorkspaceTool] = useState('');
    const [description, setDescription] = useState('');
    const [type, setType] = useState('');
    const [owner, setOwner] = useState('');

    // Boolean to check if all input fields are filled
    const allFilled = workspaceTool.trim() && description.trim() && type.trim() && owner.trim();

    // State for tracking with cell is being edited
    const [editingCell, setEditingCell] = useState(null);

    // Periodically refresh and fetch workspace_tools data every 10 seconds
    useEffect(() => {
        if (workspaceToolData.length === 0) {
            fetch('http://localhost:3000/api/workspace_tools/columns')
            .then(res => res.json())
            .then(data => setColumns(data))
            .catch(error => console.error('Error fetching Workspace Tools Header:', error));
        }
        if (workspaceToolData.length != 0) {
            const interval = setInterval(() => {
                // Refresh data every 10 seconds
                fetch('http://localhost:3000/api/workspace_tools')
                .then(res => res.json())
                .then(data => {
                    // Only update if not currently editing
                    if (!editingCell) {
                        setWorkspaceToolData(data);
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
            console.log('New Workspace Tool POST Request ready to be sent');

            // Immediate Update UI
            // Temp Time Stamp
            const tempTime = new Date();
            const formattedTimestamp = tempTime.toLocaleString('en-Us', {
                hour: '2-digit',
                minute: '2-digit',
                month: '2-digit',
                day: '2-digit',
                year: 'numeric'
            }).replace(',','');
            
            const newWorkspaceTool = {
                name: workspaceTool,
                description: description,
                type: type,
                owner: owner,
                created_at: formattedTimestamp,
                updated_at: formattedTimestamp,
                // Need tempKey because the row is being manually added temporarily
                tempKey: Date.now() + Math.random() // Unique id for this session
            };
            setWorkspaceToolData(prev => [...prev, newWorkspaceTool]);

            // POST Request
            fetch('http://localhost:3000/api/workspace_tools', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(newWorkspaceTool)
            })
            .then(response => response.json())
            .then(data => {
                console.log('New Workspace Tool Added:', data);
                // Optionally replace the last item with the real data (with id)
                setWorkspaceToolData(prev => {
                    const updated = [...prev];
                    updated[updated.length - 1] = data;
                    return updated;
                });

                // Optional to clear inputs or update UI here
                setWorkspaceTool('');
                setDescription('');
                setType('');
                setOwner('');
                // Collapse the input row after submission
                toggleAddNew(false);
            })
            .catch(error => {
                console.error('Error creating Workspace Tool:', error);
                // If request fails, remove the optimistically added row
                setWorkspaceToolData(prev => prev.slice(0, -1));
            });
            event.preventDefault(),
            event.target.blur() // Trigger onBlur to save changes
        }
    }

    // Handle Cell Edits
    const handleCellEdit = async (rowIndex, field, newValue) => {
        const workspaceTool = workspaceToolData[rowIndex];
        const originalValue = workspaceTool[field];

        // If the cell value is changed, update the cell
        if (newValue !== originalValue) {
            try {
                // Temp TimeStamp
                const tempTime = new Date();
                const formattedTimestamp = tempTime.toLocaleString('en-US', {
                    hours: '2-digit',
                    minute: '2-digit',
                    month: '2-digit',
                    day: '2-digit',
                    year: 'numeric'
                }).replace(',','');

                // Update UI
                const updatedData = [...workspaceToolData];
                updateData[rowIndex] = {...workspaceTool, [field]: newValue, updated_at: formattedTimestamp };
                setWorkspaceToolData(updatedData);
            
                console.log(`newValue: ${newValue}`);
                // Send PUT request to backend to update cell value in database table
                await fetch('http://localhost:3000/api/workspace_tools', {
                    method: 'PUT',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({
                        id: workspaceTool.id,
                        [field]: newValue
                    })
                });
            } catch (error) {
                console.error('Error updating Workspace Tool:', error);
                // Revert if error occurs
                const revertedData = [...workspaceToolData];
                revertedData[rowIndex] = workspaceTool;
                setWorkspaceToolData(revertedDatat);
            }
        }
        setEditingCell(null); // Reset editing cell state
    };
    
    // 
    const handleCellKeyDown = (event, rowIndex, field) => {
        if(event.key === 'Enter') {
            event.preventDefault();
            event.target.blur(); // Trigger onBlur to save changes
        } else if (event.key === 'Escape') {
            event.preventDefault();
            // Revert to original value if Escape is pressed
            const originalValue = workspaceToolData[rowIndex][field];
            event.target.textContent = originalValue;
            event.target.blur();
        }
    };

    // Function to handle deleting a workspace tool
    function deleteWorkspaceTool(id) {
        // Send DELETE request to backend to delete workspace tool
        fetch('http://localhost:3000/api/workspace_tools', {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ id })
        })
        .then(response => response.json())
        .then(data => {
            console.log('Workspace Tool Deleted:', data);
            // Remove the deleted workspace from the UI
            setWorkspaceToolData(prev => prev.filter(workspaceTool => workspaceTool.id !== id));
        });
    }

    // Variable to declare which columns are not editable from frontend by user
    // Maybe add 'Type' to this list later
    const uneditableWorkspaceToolColumns = ['id', 'created_at', 'updated_at'];

    return (
        <div style={{
            width: '90vw',
            height: '90vh',
            backgroundColor: '#07082B', // Dark background
            color: '#e6e8fa', // Eggwhite text rgba(255, 158, 0, 0.8)
        }}
        >
            <table style={{
                borderStyle: 'solid',
                borderWidht: '1px',
                borderColor: '#e6e8fa',
                marginLeft: 'auto',
                marginRight: 'auto',
                width: '100%',
                borderCollapse: 'collapse'
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
                        <td style={rowCellStyle}><input style={inputRowStyle} className="inputRowWorkspacesTools" value={workspaceTool} onChange={event => setWorkspaceTool(event.target.value)} onKeyDown={handleKeyDown} required /></td>
                        <td style={rowCellStyle}><input style={inputRowStyle} className="inputRowWorkspacesTools" value={description} onChange={event => setDescription(event.target.value)} onKeyDown={handleKeyDown} required /></td>
                        <td style={rowCellStyle}><input style={inputRowStyle} className="inputRowWorkspacesTools" value={type} onChange={event => setType(event.target.value)} onKeyDown={handleKeyDown} required /></td>
                        <td style={rowCellStyle}><input style={inputRowStyle} className="inputRowWorkspacesTools" value={owner} onChange={event => setOwner(event.target.value)} onKeyDown={handleKeyDown} required /></td>
                        <td style={rowCellStyle}></td>
                        <td style={rowCellStyle}></td>
                        <td style={rowCellStyle}></td>
                    </tr>
                    }
                    {workspaceToolData.map((row, rowIndex) => ( // Loops through each row in database table, rowIndex is position in array
                        <tr key={row.id ? `id-${row.id}` : `temp-${row.tempKey}`}>
                            
                            <td style={rowButtonStyle}>
                                <button style={{backgroundColor: '#e6e8fa', color: '#ff9e00', padding: '5px 10px', borderRadius: '5px', border: 'none'}}/>
                                
                            </td>
                            {columns.map(column => (    // Loop through columns arrray for each row
                                <td
                                    key={column}
                                    style={rowCellStyle}
                                    contentEditable={!uneditableWorkspaceToolColumns.includes(column)}
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
                                <button onClick={() => deleteWorkspaceTool(row.id)} style={{backgroundColor: '#E4080A', color: '#ff9e00', padding: '5px 10px', borderRadius: '5px', border: 'none'}}/>
                            </td>
                        </tr>
                    ))}
                </tbody>

            </table>
        </div>
    )

}

export default WorkspaceTools;