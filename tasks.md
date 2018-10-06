Cleanup to area_frame_allocator
    Split out a method to print details about an allocator
    Keep track of start frame and current frame
    Give an allocator a name - careful with string ownership (static lifetime)

    Look at freeing page frames


Memory Area
    Add, Remove, Fault etc