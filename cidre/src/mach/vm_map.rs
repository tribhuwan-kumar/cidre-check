use super::{KernReturn, VmAddr, VmAllocationFlags, VmSize};

extern "C" {
    pub fn vm_allocate(
        target_task: super::VmMap,
        address: *mut VmAddr,
        size: VmSize,
        flags: VmAllocationFlags,
    ) -> KernReturn;

    pub fn vm_deallocate(target_task: super::VmMap, address: VmAddr, size: VmSize) -> KernReturn;
}
