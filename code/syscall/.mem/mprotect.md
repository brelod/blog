Changes the access protection of a memory region

# Description
This function can change the access protection of memory pages allocated by mmap

# Arguments
- **addr**: Start address of the memory region. It must be page-aligned
- **len**: Length of the memory region
- **prot**: Protection flags
    - [PROT_NONE]
    - [PROT_EXEC]
    - [PROT_READ]
    - [PROT_WRITE]
    - [PROT_SEM]
    - [PROT_GROWSUP]
    - [PROT_GROWSDOWN]

# Return value
This system call returns either `()` or one of the following errors:
- [EACCES]:
    - A protection type can not be assigned to the specified pages. For example a file was mapped with [O_RONLY] and
    now we are trying to apply [PROT_WRITE].
- [EINVAL]
    - addr is not a valid, page-aligned pointer
    - Both [PROT_GROWSUP] and [PROT_GROWSDOWN] were specified
    - Invalid protection was used
- [ENOMEM]
    - Maximum number of mappings has been reached. For example if we change the protection of a page in the middle of
        an existing bigger mapping, then it will result in 3 mappings.
    - Failed to allocate kernel datastructure
    - The address range is not mapped or invalid for the process

# Notes

# Examples
