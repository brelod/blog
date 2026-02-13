Changes the size of the datasegment

# Description
This function let the user allocate / free up memory by changing the size of the datasegment.

# Arguments
- **addr**: The address of the new endpoint of the datasegment.

# Return value
This system call returns the address of the current program break. This means that an invalid address (like 0)
can be used to get the current program break.

# Notes

# Examples
