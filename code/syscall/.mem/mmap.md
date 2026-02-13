Creates a new memory mapping

# Description
The function creates a new memory mapping and allows the user to set how this memory area should work.
In general there are two types of memory: anonymous and file-backed. The anymous memory means that there
is (typically) no file on the filesystem in which the content of this memory could be written into. As opposed
to the file-backed memory the anonymous memory is written into the swap space of the system as part of the normal
memory management of the kernel. If one maps a file into virtual memory address space of the program, the content
of the file will be available in this memory area and the changes of this area can be synchronized into the
underlying file.

# Arguments
- **addr**: The address of the new memory mapping must be page aligned, always greater then the one set in 
    `/proc/sys/vm/mmap_min_addr` and can be defined by the 
    - **Kernel**: If the 0 is used as addr argument the kernel will pick an address most usable for the mapping
    - **User**: If the address is specified and MAP_FIXED or MAP_FIXED_NOREPLACE is used in the flags
        argument then the kernel tries to use the address specified by the user.
    - **Both**: If the address is specified but the `MAP_FIXED*` flags are not used the kernel takes the address
        as a hint and if it's not suitable then it tries to find the nearest address available.
- **len**: length of the new mapping. it must be greater than 0
- **prot**: memory protection to use - it must align with the file protection in case of file mapping
    - **PROT_NONE**: Memory page may not be accessed
    - **PROT_EXEC**: Memory page may be executed
    - **PROT_READ**: Memory page may be read
    - **PROT_WRITE**: Memory page may be written
    - **PROT_SEM**: Memory page may be used for atomic operations
    - **PROT_GROWSUP**: Memory page grows upwards
    - **PROT_GROWSDOWN**: Memory page grows downwards

- **flags**: The following flags are available to set the behaviour of the mapping:
    - Access:
        - **MAP_SHARED**: 
            Memory page can be shared with other processes and in case of file backed mapping
            it will be written to the disk. Use [msync] to control when this happens.

        - **MAP_PRIVATE**: 
            Mamory page can not be access from another process and the changes are not written to the disk
            in case of file backed mapping. It it unspecified if changes to the file are mirrored in the memory.

        - **MAP_SHARED_VALIDATE**: 
            This works the same as MAP_SHARED but it also validates the flags passed to the function
            and returns EOPNOTSUPP error in case of invalid flags.

        - **MAP_ANONYMOUS**: 
            Create anonymous mapping (not backed by a file) which is initialized by zeros.
            fd needs to be -1 and the offset 0 in the mmap function.

    - Address:
        - **MAP_32BIT**: 
             This flag allows to create a mapping in the first 2GB of the process address space.
             On old x84-86 platforms this made the context-switch faster if the stack of the threads
             were defined in this region. On new architechtures it's no a problem anymore.
             Note: this flag is ignored if MAP_FIXED is specifed.

        - **MAP_FIXED**: 
            Use the addr parameter not just as a hint but as a hard rule. To make it work the 
            process needs to make sure that the addr is page-aligned. If the new memory mapping
            is overlapped with an existing one, the old memory mapping will be overwritten.

        - **MAP_FIXED_NOREPLACE**: 
            This flags has the same effect as MAP_FIXED but it doesn't overwrite the content of an
            existing mapping. If the new mapping overlaps mmap returns with EEXIST error. Old kernels
            are doesn't return with error but with a different address.

    - Initialization:
        - **MAP_POPULATE**: 
            Populate the page tables of the mapping in case of file backed mapping. This allows
            readahead operation and descrease the number of pages faults. It must be used with MAP_PRIVATE.

        - **MAP_NONBLOCK**: 
            It disables the functionality of MAP_POPULATE

        - **MAP_UNINITIALIZED**: 
            This allows to create anonymous mapping without zeroing out the address space. To work the
            kernel needs to be compiled with `CONFIG_MMAP_ALLOW_UNINITIALIZED`

    - Synchronization:
        - **MAP_SYNC**: 
            This flag can only be used with MAP_SHARED_VALIDTAE and files which are supporting DAX
            (direct memory access). This option guarantees that by writing into the memory it will be
            immediatelly accessible in the file as a persistent backend.

    - Layout
        - **MAP_STACK**: 
            Select an address which is well suited for process/thread stack.

        - **MAP_GROWSDOWN**: 
            Stack like mapping

        - **MAP_NORESERVE**: 
            Do not allocate swap space for an anonymous memory. Return SIGSEGV instead in case of not enough
            physical memory presents.

    - Huge pages:
        - **MAP_HUGETLB**:
            Allocate mapping with huge pages. There is a default huge page size but one can use
            an alternative size to by combining this flag with one of the MAP_HUGE_ flags
            The supported huge page sizes can be seen here: `/sys/kernel/mm/hugepages`

        - **MAP_HUGE_SHIFT**
        - **MAP_HUGE_MASK**
        - **MAP_HUGE_16KB**
        - **MAP_HUGE_64KB**
        - **MAP_HUGE_512KB**
        - **MAP_HUGE_1MB**
        - **MAP_HUGE_2MB**
        - **MAP_HUGE_8MB**
        - **MAP_HUGE_16MB**
        - **MAP_HUGE_32MB**
        - **MAP_HUGE_256MB**
        - **MAP_HUGE_512MB**
        - **MAP_HUGE_1GB**
        - **MAP_HUGE_2GB**
        - **MAP_HUGE_16GB**

- **fd**: File descriptor in case of file backed memory mappings. After the return of the call the fd can be closed.
- **off**: Offset in the file to map from (must be page-aligned). In case of using the Huge page support the offset
    must be aligned to the huge page size

# Return value
This function returns with either a valid memory address or one of the following errors:
- [EACCES]:
    - Mapping a non-regular file
    - File is not open for reading
    - File is not open for writing but MAP_SHARED and PROT_WRITE is set
    - File is open with append-only and PROT_WRITE is set
- [EAGAIN]:
    - File has been locked
    - Too much memory has been locked
- [EBADF]: 
    - Fd is not valid and MAP_ANONYMOUS is not set
- [EINVAL]:
    - Address is not page-aligned
    - Length is zero or too big
    - Offset is too big
    - Flags doesn't contain any of MAP_PRIVATE, MAP_SHARED, MAP_SHARED_VALIDATE
    - Invalid flag is used
- [ENFILE]: 
    - Limit of open files has been reached
- [ENODEV]: 
    - Filesystem of the specified file does not support memory mapping
- [ENOMEM]: 
    - No memory is available
    - Maximum number of mappings has been reached
    - [RLIMIT_DATA] has been reached
- [EOVERFLOW]:
    - On x86 with large file extension (64bit offset) can overflow the usize (32bit).
- [EPERM]:
    - The file is located on a filesystem without exec permissions but the mapping is created with PROT_EXEC
    - Operation was prevented by file seal (see File sealing in fcntl(2))

# Notes:
The incorrect use of a mapping can result the following signals:
- [SIGSEGV]: Attempt to write into a mapping without PROT_WRITE
- [SIGBUS]: Attempt to access a page which lies beyond the mapped file

# Examples:

