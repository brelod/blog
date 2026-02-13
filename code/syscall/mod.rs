#![allow(unused)]

const SYS_READ:                    isize = 0;
const SYS_WRITE:                   isize = 1;
const SYS_OPEN:                    isize = 2;
const SYS_CLOSE:                   isize = 3;
const SYS_NEWSTAT:                 isize = 4;
const SYS_NEWFSTAT:                isize = 5;
const SYS_NEWLSTAT:                isize = 6;
const SYS_POLL:                    isize = 7;
const SYS_LSEEK:                   isize = 8;
const SYS_MMAP:                    isize = 9;
const SYS_MPROTECT:                isize = 10;
const SYS_MUNMAP:                  isize = 11;
const SYS_BRK:                     isize = 12;
const SYS_RT_SIGACTION:            isize = 13; // missing
const SYS_RT_SIGPROCMASK:          isize = 14; // missing
const SYS_RT_SIGRETURN:            isize = 15; // missing
const SYS_IOCTL:                   isize = 16; // missing
const SYS_PREAD64:                 isize = 17;
const SYS_PWRITE64:                isize = 18;
const SYS_READV:                   isize = 19;
const SYS_WRITEV:                  isize = 20;
const SYS_ACCESS:                  isize = 21;
const SYS_PIPE:                    isize = 22;
const SYS_SELECT:                  isize = 23;
const SYS_SCHED_YIELD:             isize = 24;
const SYS_MREMAP:                  isize = 25;
const SYS_MSYNC:                   isize = 26;
const SYS_MINCORE:                 isize = 27;
const SYS_MADVISE:                 isize = 28;
const SYS_SHMGET:                  isize = 29;
const SYS_SHMAT:                   isize = 30;
const SYS_SHMCTL:                  isize = 31;
const SYS_DUP:                     isize = 32;
const SYS_DUP2:                    isize = 33;
const SYS_PAUSE:                   isize = 34;
const SYS_NANOSLEEP:               isize = 35;
const SYS_GETITIMER:               isize = 36;
const SYS_ALARM:                   isize = 37;
const SYS_SETITIMER:               isize = 38;
const SYS_GETPID:                  isize = 39;
const SYS_SENDFILE64:              isize = 40;
const SYS_SOCKET:                  isize = 41;
const SYS_CONNECT:                 isize = 42;
const SYS_ACCEPT:                  isize = 43;
const SYS_SENDTO:                  isize = 44;
const SYS_RECVFROM:                isize = 45;
const SYS_SENDMSG:                 isize = 46;
const SYS_RECVMSG:                 isize = 47;
const SYS_SHUTDOWN:                isize = 48;
const SYS_BIND:                    isize = 49;
const SYS_LISTEN:                  isize = 50;
const SYS_GETSOCKNAME:             isize = 51;
const SYS_GETPEERNAME:             isize = 52;
const SYS_SOCKETPAIR:              isize = 53;
const SYS_SETSOCKOPT:              isize = 54;
const SYS_GETSOCKOPT:              isize = 55;
const SYS_CLONE:                   isize = 56;
const SYS_FORK:                    isize = 57;
const SYS_VFORK:                   isize = 58;
const SYS_EXECVE:                  isize = 59;
const SYS_EXIT:                    isize = 60;
const SYS_WAIT4:                   isize = 61;
const SYS_KILL:                    isize = 62;
const SYS_NEWUNAME:                isize = 63;
const SYS_SEMGET:                  isize = 64;
const SYS_SEMOP:                   isize = 65;
const SYS_SEMCTL:                  isize = 66;
const SYS_SHMDT:                   isize = 67;
const SYS_MSGGET:                  isize = 68;
const SYS_MSGSND:                  isize = 69;
const SYS_MSGRCV:                  isize = 70;
const SYS_MSGCTL:                  isize = 71;
const SYS_FCNTL:                   isize = 72;
const SYS_FLOCK:                   isize = 73;
const SYS_FSYNC:                   isize = 74;
const SYS_FDATASYNC:               isize = 75;
const SYS_TRUNCATE:                isize = 76;
const SYS_FTRUNCATE:               isize = 77;
const SYS_GETDENTS:                isize = 78;
const SYS_GETCWD:                  isize = 79;
const SYS_CHDIR:                   isize = 80;
const SYS_FCHDIR:                  isize = 81;
const SYS_RENAME:                  isize = 82;
const SYS_MKDIR:                   isize = 83;
const SYS_RMDIR:                   isize = 84;
const SYS_CREAT:                   isize = 85;
const SYS_LINK:                    isize = 86;
const SYS_UNLINK:                  isize = 87;
const SYS_SYMLINK:                 isize = 88;
const SYS_READLINK:                isize = 89;
const SYS_CHMOD:                   isize = 90;
const SYS_FCHMOD:                  isize = 91;
const SYS_CHOWN:                   isize = 92;
const SYS_FCHOWN:                  isize = 93;
const SYS_LCHOWN:                  isize = 94;
const SYS_UMASK:                   isize = 95;
const SYS_GETTIMEOFDAY:            isize = 96;
const SYS_GETRLIMIT:               isize = 97;
const SYS_GETRUSAGE:               isize = 98;
const SYS_SYSINFO:                 isize = 99;
const SYS_TIMES:                   isize = 100;
const SYS_PTRACE:                  isize = 101;
const SYS_GETUID:                  isize = 102;
const SYS_SYSLOG:                  isize = 103;
const SYS_GETGID:                  isize = 104;
const SYS_SETUID:                  isize = 105;
const SYS_SETGID:                  isize = 106;
const SYS_GETEUID:                 isize = 107;
const SYS_GETEGID:                 isize = 108;
const SYS_SETPGID:                 isize = 109;
const SYS_GETPPID:                 isize = 110;
const SYS_GETPGRP:                 isize = 111;
const SYS_SETSID:                  isize = 112;
const SYS_SETREUID:                isize = 113;
const SYS_SETREGID:                isize = 114;
const SYS_GETGROUPS:               isize = 115;
const SYS_SETGROUPS:               isize = 116;
const SYS_SETRESUID:               isize = 117;
const SYS_GETRESUID:               isize = 118;
const SYS_SETRESGID:               isize = 119;
const SYS_GETRESGID:               isize = 120;
const SYS_GETPGID:                 isize = 121;
const SYS_SETFSUID:                isize = 122;
const SYS_SETFSGID:                isize = 123;
const SYS_GETSID:                  isize = 124;
const SYS_CAPGET:                  isize = 125;
const SYS_CAPSET:                  isize = 126;
const SYS_RT_SIGPENDING:           isize = 127;
const SYS_RT_SIGTIMEDWAIT:         isize = 128;
const SYS_RT_SIGQUEUEINFO:         isize = 129;
const SYS_RT_SIGSUSPEND:           isize = 130;
const SYS_SIGALTSTACK:             isize = 131;
const SYS_UTIME:                   isize = 132;
const SYS_MKNOD:                   isize = 133;
const SYS_PERSONALITY:             isize = 135;
const SYS_USTAT:                   isize = 136;
const SYS_STATFS:                  isize = 137;
const SYS_FSTATFS:                 isize = 138;
const SYS_SYSFS:                   isize = 139;
const SYS_GETPRIORITY:             isize = 140;
const SYS_SETPRIORITY:             isize = 141;
const SYS_SCHED_SETPARAM:          isize = 142;
const SYS_SCHED_GETPARAM:          isize = 143;
const SYS_SCHED_SETSCHEDULER:      isize = 144;
const SYS_SCHED_GETSCHEDULER:      isize = 145;
const SYS_SCHED_GET_PRIORITY_MAX:  isize = 146;
const SYS_SCHED_GET_PRIORITY_MIN:  isize = 147;
const SYS_SCHED_RR_GET_INTERVAL:   isize = 148;
const SYS_MLOCK:                   isize = 149;
const SYS_MUNLOCK:                 isize = 150;
const SYS_MLOCKALL:                isize = 151;
const SYS_MUNLOCKALL:              isize = 152;
const SYS_VHANGUP:                 isize = 153;
const SYS_MODIFY_LDT:              isize = 154;
const SYS_PIVOT_ROOT:              isize = 155;
const SYS_NI_SYSCALL:              isize = 156;
const SYS_PRCTL:                   isize = 157;
const SYS_ARCH_PRCTL:              isize = 158;
const SYS_ADJTIMEX:                isize = 159;
const SYS_SETRLIMIT:               isize = 160;
const SYS_CHROOT:                  isize = 161;
const SYS_SYNC:                    isize = 162;
const SYS_ACCT:                    isize = 163;
const SYS_SETTIMEOFDAY:            isize = 164;
const SYS_MOUNT:                   isize = 165;
const SYS_UMOUNT:                  isize = 166;
const SYS_SWAPON:                  isize = 167;
const SYS_SWAPOFF:                 isize = 168;
const SYS_REBOOT:                  isize = 169;
const SYS_SETHOSTNAME:             isize = 170;
const SYS_SETDOMAINNAME:           isize = 171;
const SYS_IOPL:                    isize = 172;
const SYS_IOPERM:                  isize = 173;
const SYS_INIT_MODULE:             isize = 175;
const SYS_DELETE_MODULE:           isize = 176;
const SYS_QUOTACTL:                isize = 179;
const SYS_GETTID:                  isize = 186;
const SYS_READAHEAD:               isize = 187;
const SYS_SETXATTR:                isize = 188;
const SYS_LSETXATTR:               isize = 189;
const SYS_FSETXATTR:               isize = 190;
const SYS_GETXATTR:                isize = 191;
const SYS_LGETXATTR:               isize = 192;
const SYS_FGETXATTR:               isize = 193;
const SYS_LISTXATTR:               isize = 194;
const SYS_LLISTXATTR:              isize = 195;
const SYS_FLISTXATTR:              isize = 196;
const SYS_REMOVEXATTR:             isize = 197;
const SYS_LREMOVEXATTR:            isize = 198;
const SYS_FREMOVEXATTR:            isize = 199;
const SYS_TKILL:                   isize = 200;
const SYS_TIME:                    isize = 201;
const SYS_FUTEX:                   isize = 202;
const SYS_SCHED_SETAFFINITY:       isize = 203;
const SYS_SCHED_GETAFFINITY:       isize = 204;
const SYS_IO_SETUP:                isize = 206;
const SYS_IO_DESTROY:              isize = 207;
const SYS_IO_GETEVENTS:            isize = 208;
const SYS_IO_SUBMIT:               isize = 209;
const SYS_IO_CANCEL:               isize = 210;
const SYS_EPOLL_CREATE:            isize = 213;
const SYS_REMAP_FILE_PAGES:        isize = 216;
const SYS_GETDENTS64:              isize = 217;
const SYS_SET_TID_ADDRESS:         isize = 218;
const SYS_RESTART_SYSCALL:         isize = 219;
const SYS_SEMTIMEDOP:              isize = 220;
const SYS_FADVISE64:               isize = 221;
const SYS_TIMER_CREATE:            isize = 222;
const SYS_TIMER_SETTIME:           isize = 223;
const SYS_TIMER_GETTIME:           isize = 224;
const SYS_TIMER_GETOVERRUN:        isize = 225;
const SYS_TIMER_DELETE:            isize = 226;
const SYS_CLOCK_SETTIME:           isize = 227;
const SYS_CLOCK_GETTIME:           isize = 228;
const SYS_CLOCK_GETRES:            isize = 229;
const SYS_CLOCK_NANOSLEEP:         isize = 230;
const SYS_EXIT_GROUP:              isize = 231;
const SYS_EPOLL_WAIT:              isize = 232;
const SYS_EPOLL_CTL:               isize = 233;
const SYS_TGKILL:                  isize = 234;
const SYS_UTIMES:                  isize = 235;
const SYS_MBIND:                   isize = 237;
const SYS_SET_MEMPOLICY:           isize = 238;
const SYS_GET_MEMPOLICY:           isize = 239;
const SYS_MQ_OPEN:                 isize = 240;
const SYS_MQ_UNLINK:               isize = 241;
const SYS_MQ_TIMEDSEND:            isize = 242;
const SYS_MQ_TIMEDRECEIVE:         isize = 243;
const SYS_MQ_NOTIFY:               isize = 244;
const SYS_MQ_GETSETATTR:           isize = 245;
const SYS_KEXEC_LOAD:              isize = 246;
const SYS_WAITID:                  isize = 247;
const SYS_ADD_KEY:                 isize = 248;
const SYS_REQUEST_KEY:             isize = 249;
const SYS_KEYCTL:                  isize = 250;
const SYS_IOPRIO_SET:              isize = 251;
const SYS_IOPRIO_GET:              isize = 252;
const SYS_INOTIFY_INIT:            isize = 253;
const SYS_INOTIFY_ADD_WATCH:       isize = 254;
const SYS_INOTIFY_RM_WATCH:        isize = 255;
const SYS_MIGRATE_PAGES:           isize = 256;
const SYS_OPENAT:                  isize = 257;
const SYS_MKDIRAT:                 isize = 258;
const SYS_MKNODAT:                 isize = 259;
const SYS_FCHOWNAT:                isize = 260;
const SYS_FUTIMESAT:               isize = 261;
const SYS_NEWFSTATAT:              isize = 262;
const SYS_UNLINKAT:                isize = 263;
const SYS_RENAMEAT:                isize = 264;
const SYS_LINKAT:                  isize = 265;
const SYS_SYMLINKAT:               isize = 266;
const SYS_READLINKAT:              isize = 267;
const SYS_FCHMODAT:                isize = 268;
const SYS_FACCESSAT:               isize = 269;
const SYS_PSELECT6:                isize = 270;
const SYS_PPOLL:                   isize = 271;
const SYS_UNSHARE:                 isize = 272;
const SYS_SET_ROBUST_LIST:         isize = 273;
const SYS_GET_ROBUST_LIST:         isize = 274;
const SYS_SPLICE:                  isize = 275;
const SYS_TEE:                     isize = 276;
const SYS_SYNC_FILE_RANGE:         isize = 277;
const SYS_VMSPLICE:                isize = 278;
const SYS_MOVE_PAGES:              isize = 279;
const SYS_UTIMENSAT:               isize = 280;
const SYS_EPOLL_PWAIT:             isize = 281;
const SYS_SIGNALFD:                isize = 282;
const SYS_TIMERFD_CREATE:          isize = 283;
const SYS_EVENTFD:                 isize = 284;
const SYS_FALLOCATE:               isize = 285;
const SYS_TIMERFD_SETTIME:         isize = 286;
const SYS_TIMERFD_GETTIME:         isize = 287;
const SYS_ACCEPT4:                 isize = 288;
const SYS_SIGNALFD4:               isize = 289;
const SYS_EVENTFD2:                isize = 290;
const SYS_EPOLL_CREATE1:           isize = 291;
const SYS_DUP3:                    isize = 292;
const SYS_PIPE2:                   isize = 293;
const SYS_INOTIFY_INIT1:           isize = 294;
const SYS_PREADV:                  isize = 295;
const SYS_PWRITEV:                 isize = 296;
const SYS_RT_TGSIGQUEUEINFO:       isize = 297;
const SYS_PERF_EVENT_OPEN:         isize = 298;
const SYS_RECVMMSG:                isize = 299;
const SYS_FANOTIFY_INIT:           isize = 300;
const SYS_FANOTIFY_MARK:           isize = 301;
const SYS_PRLIMIT64:               isize = 302;
const SYS_NAME_TO_HANDLE_AT:       isize = 303;
const SYS_OPEN_BY_HANDLE_AT:       isize = 304;
const SYS_CLOCK_ADJTIME:           isize = 305;
const SYS_SYNCFS:                  isize = 306;
const SYS_SENDMMSG:                isize = 307;
const SYS_SETNS:                   isize = 308;
const SYS_GETCPU:                  isize = 309;
const SYS_PROCESS_VM_READV:        isize = 310;
const SYS_PROCESS_VM_WRITEV:       isize = 311;
const SYS_KCMP:                    isize = 312;
const SYS_FINIT_MODULE:            isize = 313;
const SYS_SCHED_SETATTR:           isize = 314;
const SYS_SCHED_GETATTR:           isize = 315;
const SYS_RENAMEAT2:               isize = 316;
const SYS_SECCOMP:                 isize = 317;
const SYS_GETRANDOM:               isize = 318;
const SYS_MEMFD_CREATE:            isize = 319;
const SYS_KEXEC_FILE_LOAD:         isize = 320;
const SYS_BPF:                     isize = 321;
const SYS_EXECVEAT:                isize = 322;
const SYS_USERFAULTFD:             isize = 323;
const SYS_MEMBARRIER:              isize = 324;
const SYS_MLOCK2:                  isize = 325;
const SYS_COPY_FILE_RANGE:         isize = 326;
const SYS_PREADV2:                 isize = 327;
const SYS_PWRITEV2:                isize = 328;
const SYS_PKEY_MPROTECT:           isize = 329;
const SYS_PKEY_ALLOC:              isize = 330;
const SYS_PKEY_FREE:               isize = 331;
const SYS_STATX:                   isize = 332;
const SYS_IO_PGETEVENTS:           isize = 333;
const SYS_RSEQ:                    isize = 334;
const SYS_PIDFD_SEND_SIGNAL:       isize = 424;
const SYS_IO_URING_SETUP:          isize = 425;
const SYS_IO_URING_ENTER:          isize = 426;
const SYS_IO_URING_REGISTER:       isize = 427;
const SYS_OPEN_TREE:               isize = 428;
const SYS_MOVE_MOUNT:              isize = 429;
const SYS_FSOPEN:                  isize = 430;
const SYS_FSCONFIG:                isize = 431;
const SYS_FSMOUNT:                 isize = 432;
const SYS_FSPICK:                  isize = 433;
const SYS_PIDFD_OPEN:              isize = 434;
const SYS_CLONE3:                  isize = 435;
const SYS_CLOSE_RANGE:             isize = 436;
const SYS_OPENAT2:                 isize = 437;
const SYS_PIDFD_GETFD:             isize = 438;
const SYS_FACCESSAT2:              isize = 439;
const SYS_PROCESS_MADVISE:         isize = 440;
const SYS_EPOLL_PWAIT2:            isize = 441;
const SYS_MOUNT_SETATTR:           isize = 442;
const SYS_QUOTACTL_FD:             isize = 443;
const SYS_LANDLOCK_CREATE_RULESET: isize = 444;
const SYS_LANDLOCK_ADD_RULE:       isize = 445;
const SYS_LANDLOCK_RESTRICT_SELF:  isize = 446;
const SYS_MEMFD_SECRET:            isize = 447;
const SYS_PROCESS_MRELEASE:        isize = 448;
const SYS_FUTEX_WAITV:             isize = 449;
const SYS_SET_MEMPOLICY_HOME_NODE: isize = 450;
const SYS_CACHESTAT:               isize = 451;
const SYS_FCHMODAT2:               isize = 452;
const SYS_MAP_SHADOW_STACK:        isize = 453;
const SYS_FUTEX_WAKE:              isize = 454;
const SYS_FUTEX_WAIT:              isize = 455;
const SYS_FUTEX_REQUEUE:           isize = 456;
const SYS_STATMOUNT:               isize = 457;
const SYS_LISTMOUNT:               isize = 458;
const SYS_LSM_GET_SELF_ATTR:       isize = 459;
const SYS_LSM_SET_SELF_ATTR:       isize = 460;
const SYS_LSM_LIST_MODULESV:       isize = 461;


/* ANCHOR: syscall-macro */
macro_rules! syscall {
    ($rax:expr) => {{
        let mut rax: isize;
        core::arch::asm!(
            "syscall",
            inlateout("rax") $rax => rax,
        );
        rax
    }};

    ($rax:expr, $rdi:expr) => {{
        let mut rax: isize;
        core::arch::asm!(
            "syscall",
            inlateout("rax") $rax => rax,
            in("rdi") $rdi,
        );
        rax
    }};

    ($rax:expr, $rdi:expr, $rsi:expr) => {{
        let mut rax: isize;
        core::arch::asm!(
            "syscall",
            inlateout("rax") $rax => rax,
            in("rdi") $rdi,
            in("rsi") $rsi,
        );
        rax
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr) => {{
        let mut rax: isize;
        core::arch::asm!(
            "syscall",
            inlateout("rax") $rax => rax,
            in("rdi") $rdi,
            in("rsi") $rsi,
            in("rdx") $rdx,
        );
        rax
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr) => {{
        let mut rax: isize;
        core::arch::asm!(
            "syscall",
            inlateout("rax") $rax => rax,
            in("rdi") $rdi,
            in("rsi") $rsi,
            in("rdx") $rdx,
            in("r10") $r10,
        );
        rax
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr, $r8:expr) => {{
        let mut rax: isize;
        core::arch::asm!(
            "syscall",
            inlateout("rax") $rax => rax,
            in("rdi") $rdi,
            in("rsi") $rsi,
            in("rdx") $rdx,
            in("r10") $r10,
            in("r8") $r8,
        );
        rax
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr, $r8:expr, $r9:expr) => {{
        let mut rax: isize;
        core::arch::asm!(
            "syscall",
            inlateout("rax") $rax => rax,
            in("rdi") $rdi,
            in("rsi") $rsi,
            in("rdx") $rdx,
            in("r10") $r10,
            in("r8") $r8,
            in("r9") $r9,
        );
        rax
    }};
}
/* ANCHOR_END: syscall-macro */

pub mod fs;
pub mod io;
pub mod ipc;
pub mod aio;
pub mod mem;
pub mod net;
pub mod run;
pub mod sec;
pub mod sig;
pub mod sys;
pub mod file;
pub mod sync;
pub mod time;
pub mod proc;
pub mod event;
pub mod sched;

pub use self::fs::*;
pub use self::io::*;
pub use self::ipc::*;
pub use self::aio::*;
pub use self::mem::*;
pub use self::net::*;
pub use self::run::*;
pub use self::sec::*;
pub use self::sig::*;
pub use self::sys::*;
pub use self::file::*;
pub use self::sync::*;
pub use self::time::*;
pub use self::proc::*;
pub use self::event::*;
pub use self::sched::*;
