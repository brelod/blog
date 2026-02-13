Destroyes a memory mapping

# Description
The munmap function deletes the mapping of the memory region specified by the addr and len arguments.
After it has been executed the subsequent access to this region will cause SIGSEGV. 

**Note**: Every page will be unmapped which is touched by this call. This means that to unmap a complete (4096 byte long) page
it's enough to call this function with legth 1.

If a mapping with huge page support is being destroyed the addr and the length must be a multiple of the underlying huge page size.

# Arguments
- **addr**: Start address of the memory region to unmap. It must be page aligned
- **len**: Length of the meory region

# Return value
- [EINVAL]:
    - Address is not page-aligned
- [ENOMEM]: 
    - It can happen if we unmap a region inside of a bigger map which results two independent smaller maps. 
    If the max number of mappings has been already reached the munmap fails with this error.
