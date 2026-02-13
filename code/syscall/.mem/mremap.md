Extends or shrinks an existing memory mapping

# Description
If the memory is locked it will continue to be locked in the new location too.

# Arguments
- **addr**: Address of the current mapping. It must be page-aligned
- **old**: Size of the current mapping
- **new**: Size of the new mapping
- **flags**: 
    - **MREMAP_MAYMOVE**:
        Allows the content of the memory to be moved on reallocation
        By default mremap fails if there is not enough memory in the current location to extend into
        If this flag is specified then the content of the memory may be moved into another location
        if it's necessarry. This invalidates every pointer pointing to the old location.

    - **MREMAP_FIXED**:
        Allows the user to decide on which address should be the content of the memory moved into
        This flags needs to be used together with the MREMAP_MAYMOVE and similarly to the MAP_FIXED
        flag it allows one to overwrite existing memory mappings

    - **MREMAP_DONTUNMAP**:
        Allows the user to move the content of the memory without deallocating the old value.
        This flags must be used together with the MREMAP_MAYMOVE and it can only be performed on
        mappings with PROT_PRIVATE and MAP_ANONYMOUS. After completion the access to the old value
        results in a page fault which can be handled by `userfaultfd(2)`

# Return value
This function returns either with a new memory address or one of the following errors:
- [EAGAIN]: 
    - [RLIMIT_MEMLOCK] has been reached while trying to remap a locked memory region
- [EFAULT]
    - The old address range specified includes an invalid memory address
- [EINVAL]
    - invalid old address (not page-aliged)
    - invalid new memory region
        - no page-aligned address
        - zero length
        - overlaps with old region
    - invalid flag was used:
        - MREMAP_FIXED or MREMAP_DONTUNMAP without MREMAP_MAYMOVE
        - MREMAP_DONTUNMAP with old region which is **not** PROT_PRIVATE && MAP_ANONYMOUS
        - MREMAP_DONTUNMAP and old size != new size
    - old size == 0 without PROT_SHARED && MREMAP_MAYMOVE
- [ENOMEM]
    - Not enough memory
    - Memory can not be extended in place and MREMAP_MAYMOVE was not used
    - Maximum number of mappings has been reached

# Notes

# Examples
