use core::arch::asm;

#[inline]
pub unsafe fn syscall0(n: usize) -> usize {
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") n as usize => ret,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall1(n: usize, args: &[usize]) -> usize {
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") n as usize => ret,
        in("rdi") args[0],
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall2(n: usize, args: &[usize]) -> usize {
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") n as usize => ret,
        in("rdi") args[0],
        in("rsi") args[1],
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall3(n: usize, args: &[usize]) -> usize {
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") n as usize => ret,
        in("rdi") args[0],
        in("rsi") args[1],
        in("rdx") args[2],
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall4(n: usize, args: &[usize]) -> usize {
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") n as usize => ret,
        in("rdi") args[0],
        in("rsi") args[1],
        in("rdx") args[2],
        in("r10") args[3],
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall5(n: usize, args: &[usize]) -> usize {
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") n as usize => ret,
        in("rdi") args[0],
        in("rsi") args[1],
        in("rdx") args[2],
        in("r10") args[3],
        in("r8")  args[4],
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall6(n: usize, args: &[usize]) -> usize {
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") n as usize => ret,
        in("rdi") args[0],
        in("rsi") args[1],
        in("rdx") args[2],
        in("r10") args[3],
        in("r8")  args[4],
        in("r9")  args[5],
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}
