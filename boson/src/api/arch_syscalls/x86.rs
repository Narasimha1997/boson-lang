use core::arch::asm;


#[inline]
pub unsafe fn syscall0(n: usize) -> usize {
    let mut ret: usize;
    asm!(
        "int $$0x80",
        inlateout("eax") n => ret,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall1(n: usize, args: &[usize]) -> usize {
    let mut ret: usize;
    asm!(
        "int $$0x80",
        inlateout("eax") n => ret,
        in("ebx") args[0],
        options(nostack, preserves_flags)
    );
    ret
}


#[inline]
pub unsafe fn syscall2(n: usize, args: &[usize]) -> usize {
    let mut ret: usize;
    asm!(
        "int $$0x80",
        inlateout("eax") n => ret,
        in("ebx") args[0],
        in("ecx") args[1],
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall3(
    n: usize,
    args: &[usize],
) -> usize {
    let mut ret: usize;
    asm!(
        "int $$0x80",
        inlateout("eax") n => ret,
        in("ebx") args[0],
        in("ecx") args[1],
        in("edx") args[2],
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall4(
    n: usize,
    args: &[usize]
) -> usize {
    let mut ret: usize;
    asm!(
        "xchg esi, {arg4}",
        "int $$0x80",
        "xchg esi, {arg4}",
        arg4 = in(reg) args[3],
        inlateout("eax") n => ret,
        in("ebx") args[0],
        in("ecx") args[1],
        in("edx") args[2],
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall5(
    n: usize,
    args: &[usize],
) -> usize {
    let mut ret: usize;
    asm!(
        "xchg esi, {arg4}",
        "int $$0x80",
        "xchg esi, {arg4}",
        arg4 = in(reg) args[4],
        inlateout("eax") n => ret,
        in("ebx") args[0],
        in("ecx") args[1],
        in("edx") args[2],
        in("edi") args[3],
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall6(
    n: usize,
    args: &[usize],
) -> usize {
    // Since using esi and ebp are not allowed and because x86 only has 6
    // general purpose registers (excluding ESP and EBP), we need to push them
    // onto the stack and then set them using a pointer to memory (our input
    // array).
    let mut ret: usize;
    asm!(
        "push ebp",
        "push esi",
        "mov esi, DWORD PTR [eax + 0]", // Set esi to arg4
        "mov ebp, DWORD PTR [eax + 4]", // Set ebp to arg6
        "mov eax, DWORD PTR [eax + 8]", // Lastly, set eax to the syscall number.
        "int $$0x80",
        "pop esi",
        "pop ebp",
        // Set eax to a pointer to our input array.
        inout("eax") &[args[4], args[5], n] => ret,
        in("ebx") args[0],
        in("ecx") args[1],
        in("edx") args[2],
        in("edi") args[3],
        options(preserves_flags)
    );
    ret
}
