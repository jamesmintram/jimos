Cleanup
    Remove all of the Result warnings for printing

Process
    Execute a process in EL0 that triggers an on demand page in
        Demarcate a segment of special code to be copied as a process "image" using linkerscript trickery
        
        memcopy this segment into the correctly sized text segment
        
        execute this code via return_to_userspace
        
Frame Allocator 
    Release
    Page management
    Reverse mapping (maybe outside of the Frame Allocator)
    Page frame ownership

Address Space
    Unmapping a segment
    Splitting/Merging etc
    MProtect/W^X
    Use Result type in "new"




