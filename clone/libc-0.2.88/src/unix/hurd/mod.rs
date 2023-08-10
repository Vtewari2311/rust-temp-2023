//! Definitions for GNU/Hurd i386

#![allow(dead_code)]

pub type __u8 = ::c_uchar;
pub type __u16 = ::c_ushort;
pub type __s16 = ::c_short;
pub type __u32 = ::c_uint;
pub type __s32 = ::c_int;

pub type Elf32_Half = u16;
pub type Elf32_Word = u32;
pub type Elf32_Off = u32;
pub type Elf32_Addr = u32;

pub type Elf64_Half = u16;
pub type Elf64_Word = u32;
pub type Elf64_Off = u64;
pub type Elf64_Addr = u64;
pub type Elf64_Xword = u64;

pub type c_char = i8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type wchar_t = i32;

pub const _AIO_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __STDC_NO_THREADS__: u32 = 1;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 31;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 32;
pub const __WORDSIZE32_SIZE_ULONG: u32 = 0;
pub const __WORDSIZE32_PTRDIFF_LONG: u32 = 0;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 0;
pub const __LONG_DOUBLE_USES_FLOAT128: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const _SYS_TYPES_H: u32 = 1;
pub const _BITS_TYPES_H: u32 = 1;
pub const __TIMESIZE: u32 = 32;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __FD_SETSIZE: u32 = 256;
pub const __STATFS_MATCHES_STATFS64: u32 = 0;
pub const _BITS_TIME64_H: u32 = 1;
pub const __clock_t_defined: u32 = 1;
pub const __clockid_t_defined: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const _ENDIAN_H: u32 = 1;
pub const _BITS_ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const _BITS_ENDIANNESS_H: u32 = 1;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const _BITS_BYTESWAP_H: u32 = 1;
pub const _BITS_UINTN_IDENTITY_H: u32 = 1;
pub const _SYS_SELECT_H: u32 = 1;
pub const __FD_ZERO_STOS: &'static [u8; 6usize] = b"stosl\0";
pub const __sigset_t_defined: u32 = 1;
pub const ____sigset_t_defined: u32 = 1;
pub const __timeval_defined: u32 = 1;
pub const _STRUCT_TIMESPEC: u32 = 1;
pub const FD_SETSIZE: u32 = 256;
pub const _BITS_PTHREADTYPES_H: u32 = 1;
pub const _THREAD_SHARED_TYPES_H: u32 = 1;
pub const _BITS_PTHREADTYPES_ARCH_H: u32 = 1;
pub const __SIZEOF_PTHREAD_MUTEX_T: u32 = 32;
pub const __SIZEOF_PTHREAD_ATTR_T: u32 = 32;
pub const __SIZEOF_PTHREAD_RWLOCK_T: u32 = 28;
pub const __SIZEOF_PTHREAD_BARRIER_T: u32 = 24;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: u32 = 16;
pub const __SIZEOF_PTHREAD_COND_T: u32 = 20;
pub const __SIZEOF_PTHREAD_CONDATTR_T: u32 = 8;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_ONCE_T: u32 = 8;
pub const _BITS_PTHREAD_H: u32 = 1;
pub const _BITS_TYPES_STRUCT___PTHREAD_ATTR: u32 = 1;
pub const _BITS_TYPES_STRUCT_SCHED_PARAM: u32 = 1;
pub const _BITS_TYPES_STRUCT___PTHREAD_MUTEXATTR_H: u32 = 1;
pub const _BITS_TYPES_STRUCT___PTHREAD_MUTEX_H: u32 = 1;
pub const _BITS_TYPES_STRUCT___PTHREAD_CONDATTR: u32 = 1;
pub const _BITS_TYPES_STRUCT___PTHREAD_COND_H: u32 = 1;
pub const _BITS_TYPES___PTHREAD_SPINLOCK_T_H: u32 = 1;
pub const __PTHREAD_SPIN_LOCK_INITIALIZER: u32 = 0;
pub const _BITS_TYPES_STRUCT___PTHREAD_RWLOCKATTR_H: u32 = 1;
pub const _BITS_TYPES_STRUCT___PTHREAD_BARRIERATTR_H: u32 = 1;
pub const _BITS_TYPES_STRUCT___PTHREAD_BARRIER_H: u32 = 1;
pub const _BITS_TYPES___PTHREAD_KEY_H: u32 = 1;
pub const _BITS_TYPES_STRUCT___PTHREAD_ONCE_H: u32 = 1;
pub const __sigevent_t_defined: u32 = 1;
pub const _BITS_SIGEVENT_CONSTS_H: u32 = 1;
pub const _ASSERT_H: u32 = 1;
pub const _COMPLEX_H: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const __HAVE_FLOAT128: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const _CPIO_H: u32 = 1;
pub const MAGIC: &'static [u8; 7usize] = b"070707\0";
pub const C_IRUSR: u32 = 256;
pub const C_IWUSR: u32 = 128;
pub const C_IXUSR: u32 = 64;
pub const C_IRGRP: u32 = 32;
pub const C_IWGRP: u32 = 16;
pub const C_IXGRP: u32 = 8;
pub const C_IROTH: u32 = 4;
pub const C_IWOTH: u32 = 2;
pub const C_IXOTH: u32 = 1;
pub const C_ISUID: u32 = 2048;
pub const C_ISGID: u32 = 1024;
pub const C_ISVTX: u32 = 512;
pub const C_ISBLK: u32 = 24576;
pub const C_ISCHR: u32 = 8192;
pub const C_ISDIR: u32 = 16384;
pub const C_ISFIFO: u32 = 4096;
pub const C_ISSOCK: u32 = 49152;
pub const C_ISLNK: u32 = 40960;
pub const C_ISCTG: u32 = 36864;
pub const C_ISREG: u32 = 32768;
pub const _CTYPE_H: u32 = 1;
pub const _BITS_TYPES_LOCALE_T_H: u32 = 1;
pub const _BITS_TYPES___LOCALE_T_H: u32 = 1;
pub const _DIRENT_H: u32 = 1;
pub const _DIRENT_HAVE_D_RECLEN: u32 = 1;
pub const _DIRENT_HAVE_D_NAMLEN: u32 = 1;
pub const _DIRENT_HAVE_D_TYPE: u32 = 1;
pub const _DIRENT_MATCHES_DIRENT64: u32 = 0;
pub const _BITS_POSIX1_LIM_H: u32 = 1;
pub const _POSIX_AIO_LISTIO_MAX: u32 = 2;
pub const _POSIX_AIO_MAX: u32 = 1;
pub const _POSIX_ARG_MAX: u32 = 4096;
pub const _POSIX_CHILD_MAX: u32 = 25;
pub const _POSIX_DELAYTIMER_MAX: u32 = 32;
pub const _POSIX_HOST_NAME_MAX: u32 = 255;
pub const _POSIX_LINK_MAX: u32 = 8;
pub const _POSIX_LOGIN_NAME_MAX: u32 = 9;
pub const _POSIX_MAX_CANON: u32 = 255;
pub const _POSIX_MAX_INPUT: u32 = 255;
pub const _POSIX_MQ_OPEN_MAX: u32 = 8;
pub const _POSIX_MQ_PRIO_MAX: u32 = 32;
pub const _POSIX_NAME_MAX: u32 = 14;
pub const _POSIX_NGROUPS_MAX: u32 = 8;
pub const _POSIX_OPEN_MAX: u32 = 20;
pub const _POSIX_PATH_MAX: u32 = 256;
pub const _POSIX_PIPE_BUF: u32 = 512;
pub const _POSIX_RE_DUP_MAX: u32 = 255;
pub const _POSIX_RTSIG_MAX: u32 = 8;
pub const _POSIX_SEM_NSEMS_MAX: u32 = 256;
pub const _POSIX_SEM_VALUE_MAX: u32 = 32767;
pub const _POSIX_SIGQUEUE_MAX: u32 = 32;
pub const _POSIX_SSIZE_MAX: u32 = 32767;
pub const _POSIX_STREAM_MAX: u32 = 8;
pub const _POSIX_SYMLINK_MAX: u32 = 255;
pub const _POSIX_SYMLOOP_MAX: u32 = 8;
pub const _POSIX_TIMER_MAX: u32 = 32;
pub const _POSIX_TTY_NAME_MAX: u32 = 9;
pub const _POSIX_TZNAME_MAX: u32 = 6;
pub const _POSIX_CLOCKRES_MIN: u32 = 20000000;
pub const NAME_MAX: u32 = 255;
pub const NGROUPS_MAX: u32 = 256;
pub const _POSIX_THREAD_KEYS_MAX: u32 = 128;
pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const _POSIX_THREAD_THREADS_MAX: u32 = 64;
pub const SEM_VALUE_MAX: u32 = 2147483647;
pub const MAXNAMLEN: u32 = 255;
pub const _DLFCN_H: u32 = 1;
pub const RTLD_LAZY: u32 = 1;
pub const RTLD_NOW: u32 = 2;
pub const RTLD_BINDING_MASK: u32 = 3;
pub const RTLD_NOLOAD: u32 = 4;
pub const RTLD_DEEPBIND: u32 = 8;
pub const RTLD_GLOBAL: u32 = 256;
pub const RTLD_LOCAL: u32 = 0;
pub const RTLD_NODELETE: u32 = 4096;
pub const _ERRNO_H: u32 = 1;
pub const _BITS_ERRNO_H: u32 = 1;
pub const EPERM: u32 = 1073741825;
pub const ENOENT: u32 = 1073741826;
pub const ESRCH: u32 = 1073741827;
pub const EINTR: u32 = 1073741828;
pub const EIO: u32 = 1073741829;
pub const ENXIO: u32 = 1073741830;
pub const E2BIG: u32 = 1073741831;
pub const ENOEXEC: u32 = 1073741832;
pub const EBADF: u32 = 1073741833;
pub const ECHILD: u32 = 1073741834;
pub const EDEADLK: u32 = 1073741835;
pub const ENOMEM: u32 = 1073741836;
pub const EACCES: u32 = 1073741837;
pub const EFAULT: u32 = 1073741838;
pub const ENOTBLK: u32 = 1073741839;
pub const EBUSY: u32 = 1073741840;
pub const EEXIST: u32 = 1073741841;
pub const EXDEV: u32 = 1073741842;
pub const ENODEV: u32 = 1073741843;
pub const ENOTDIR: u32 = 1073741844;
pub const EISDIR: u32 = 1073741845;
pub const EINVAL: u32 = 1073741846;
pub const EMFILE: u32 = 1073741848;
pub const ENFILE: u32 = 1073741847;
pub const ENOTTY: u32 = 1073741849;
pub const ETXTBSY: u32 = 1073741850;
pub const EFBIG: u32 = 1073741851;
pub const ENOSPC: u32 = 1073741852;
pub const ESPIPE: u32 = 1073741853;
pub const EROFS: u32 = 1073741854;
pub const EMLINK: u32 = 1073741855;
pub const EPIPE: u32 = 1073741856;
pub const EDOM: u32 = 1073741857;
pub const ERANGE: u32 = 1073741858;
pub const EAGAIN: u32 = 1073741859;
pub const EWOULDBLOCK: u32 = 1073741859;
pub const EINPROGRESS: u32 = 1073741860;
pub const EALREADY: u32 = 1073741861;
pub const ENOTSOCK: u32 = 1073741862;
pub const EMSGSIZE: u32 = 1073741864;
pub const EPROTOTYPE: u32 = 1073741865;
pub const ENOPROTOOPT: u32 = 1073741866;
pub const EPROTONOSUPPORT: u32 = 1073741867;
pub const ESOCKTNOSUPPORT: u32 = 1073741868;
pub const EOPNOTSUPP: u32 = 1073741869;
pub const EPFNOSUPPORT: u32 = 1073741870;
pub const EAFNOSUPPORT: u32 = 1073741871;
pub const EADDRINUSE: u32 = 1073741872;
pub const EADDRNOTAVAIL: u32 = 1073741873;
pub const ENETDOWN: u32 = 1073741874;
pub const ENETUNREACH: u32 = 1073741875;
pub const ENETRESET: u32 = 1073741876;
pub const ECONNABORTED: u32 = 1073741877;
pub const ECONNRESET: u32 = 1073741878;
pub const ENOBUFS: u32 = 1073741879;
pub const EISCONN: u32 = 1073741880;
pub const ENOTCONN: u32 = 1073741881;
pub const EDESTADDRREQ: u32 = 1073741863;
pub const ESHUTDOWN: u32 = 1073741882;
pub const ETOOMANYREFS: u32 = 1073741883;
pub const ETIMEDOUT: u32 = 1073741884;
pub const ECONNREFUSED: u32 = 1073741885;
pub const ELOOP: u32 = 1073741886;
pub const ENAMETOOLONG: u32 = 1073741887;
pub const EHOSTDOWN: u32 = 1073741888;
pub const EHOSTUNREACH: u32 = 1073741889;
pub const ENOTEMPTY: u32 = 1073741890;
pub const EPROCLIM: u32 = 1073741891;
pub const EUSERS: u32 = 1073741892;
pub const EDQUOT: u32 = 1073741893;
pub const ESTALE: u32 = 1073741894;
pub const EREMOTE: u32 = 1073741895;
pub const EBADRPC: u32 = 1073741896;
pub const ERPCMISMATCH: u32 = 1073741897;
pub const EPROGUNAVAIL: u32 = 1073741898;
pub const EPROGMISMATCH: u32 = 1073741899;
pub const EPROCUNAVAIL: u32 = 1073741900;
pub const ENOLCK: u32 = 1073741901;
pub const EFTYPE: u32 = 1073741903;
pub const EAUTH: u32 = 1073741904;
pub const ENEEDAUTH: u32 = 1073741905;
pub const ENOSYS: u32 = 1073741902;
pub const ENOTSUP: u32 = 1073741942;
pub const EILSEQ: u32 = 1073741930;
pub const EBACKGROUND: u32 = 1073741924;
pub const EDIED: u32 = 1073741925;
pub const EGREGIOUS: u32 = 1073741927;
pub const EIEIO: u32 = 1073741928;
pub const EGRATUITOUS: u32 = 1073741929;
pub const EBADMSG: u32 = 1073741931;
pub const EIDRM: u32 = 1073741932;
pub const EMULTIHOP: u32 = 1073741933;
pub const ENODATA: u32 = 1073741934;
pub const ENOLINK: u32 = 1073741935;
pub const ENOMSG: u32 = 1073741936;
pub const ENOSR: u32 = 1073741937;
pub const ENOSTR: u32 = 1073741938;
pub const EOVERFLOW: u32 = 1073741939;
pub const EPROTO: u32 = 1073741940;
pub const ETIME: u32 = 1073741941;
pub const ECANCELED: u32 = 1073741943;
pub const EOWNERDEAD: u32 = 1073741944;
pub const ENOTRECOVERABLE: u32 = 1073741945;
pub const EMACH_SEND_IN_PROGRESS: u32 = 268435457;
pub const EMACH_SEND_INVALID_DATA: u32 = 268435458;
pub const EMACH_SEND_INVALID_DEST: u32 = 268435459;
pub const EMACH_SEND_TIMED_OUT: u32 = 268435460;
pub const EMACH_SEND_WILL_NOTIFY: u32 = 268435461;
pub const EMACH_SEND_NOTIFY_IN_PROGRESS: u32 = 268435462;
pub const EMACH_SEND_INTERRUPTED: u32 = 268435463;
pub const EMACH_SEND_MSG_TOO_SMALL: u32 = 268435464;
pub const EMACH_SEND_INVALID_REPLY: u32 = 268435465;
pub const EMACH_SEND_INVALID_RIGHT: u32 = 268435466;
pub const EMACH_SEND_INVALID_NOTIFY: u32 = 268435467;
pub const EMACH_SEND_INVALID_MEMORY: u32 = 268435468;
pub const EMACH_SEND_NO_BUFFER: u32 = 268435469;
pub const EMACH_SEND_NO_NOTIFY: u32 = 268435470;
pub const EMACH_SEND_INVALID_TYPE: u32 = 268435471;
pub const EMACH_SEND_INVALID_HEADER: u32 = 268435472;
pub const EMACH_RCV_IN_PROGRESS: u32 = 268451841;
pub const EMACH_RCV_INVALID_NAME: u32 = 268451842;
pub const EMACH_RCV_TIMED_OUT: u32 = 268451843;
pub const EMACH_RCV_TOO_LARGE: u32 = 268451844;
pub const EMACH_RCV_INTERRUPTED: u32 = 268451845;
pub const EMACH_RCV_PORT_CHANGED: u32 = 268451846;
pub const EMACH_RCV_INVALID_NOTIFY: u32 = 268451847;
pub const EMACH_RCV_INVALID_DATA: u32 = 268451848;
pub const EMACH_RCV_PORT_DIED: u32 = 268451849;
pub const EMACH_RCV_IN_SET: u32 = 268451850;
pub const EMACH_RCV_HEADER_ERROR: u32 = 268451851;
pub const EMACH_RCV_BODY_ERROR: u32 = 268451852;
pub const EKERN_INVALID_ADDRESS: u32 = 1;
pub const EKERN_PROTECTION_FAILURE: u32 = 2;
pub const EKERN_NO_SPACE: u32 = 3;
pub const EKERN_INVALID_ARGUMENT: u32 = 4;
pub const EKERN_FAILURE: u32 = 5;
pub const EKERN_RESOURCE_SHORTAGE: u32 = 6;
pub const EKERN_NOT_RECEIVER: u32 = 7;
pub const EKERN_NO_ACCESS: u32 = 8;
pub const EKERN_MEMORY_FAILURE: u32 = 9;
pub const EKERN_MEMORY_ERROR: u32 = 10;
pub const EKERN_NOT_IN_SET: u32 = 12;
pub const EKERN_NAME_EXISTS: u32 = 13;
pub const EKERN_ABORTED: u32 = 14;
pub const EKERN_INVALID_NAME: u32 = 15;
pub const EKERN_INVALID_TASK: u32 = 16;
pub const EKERN_INVALID_RIGHT: u32 = 17;
pub const EKERN_INVALID_VALUE: u32 = 18;
pub const EKERN_UREFS_OVERFLOW: u32 = 19;
pub const EKERN_INVALID_CAPABILITY: u32 = 20;
pub const EKERN_RIGHT_EXISTS: u32 = 21;
pub const EKERN_INVALID_HOST: u32 = 22;
pub const EKERN_MEMORY_PRESENT: u32 = 23;
pub const EKERN_WRITE_PROTECTION_FAILURE: u32 = 24;
pub const EKERN_TERMINATED: u32 = 26;
pub const EKERN_TIMEDOUT: u32 = 27;
pub const EKERN_INTERRUPTED: u32 = 28;
pub const EMIG_TYPE_ERROR: i32 = -300;
pub const EMIG_REPLY_MISMATCH: i32 = -301;
pub const EMIG_REMOTE_ERROR: i32 = -302;
pub const EMIG_BAD_ID: i32 = -303;
pub const EMIG_BAD_ARGUMENTS: i32 = -304;
pub const EMIG_NO_REPLY: i32 = -305;
pub const EMIG_EXCEPTION: i32 = -306;
pub const EMIG_ARRAY_TOO_LARGE: i32 = -307;
pub const EMIG_SERVER_DIED: i32 = -308;
pub const EMIG_DESTROY_REQUEST: i32 = -309;
pub const ED_IO_ERROR: u32 = 2500;
pub const ED_WOULD_BLOCK: u32 = 2501;
pub const ED_NO_SUCH_DEVICE: u32 = 2502;
pub const ED_ALREADY_OPEN: u32 = 2503;
pub const ED_DEVICE_DOWN: u32 = 2504;
pub const ED_INVALID_OPERATION: u32 = 2505;
pub const ED_INVALID_RECNUM: u32 = 2506;
pub const ED_INVALID_SIZE: u32 = 2507;
pub const ED_NO_MEMORY: u32 = 2508;
pub const ED_READ_ONLY: u32 = 2509;
pub const _HURD_ERRNOS: u32 = 122;
pub const _FCNTL_H: u32 = 1;
pub const O_RDONLY: u32 = 1;
pub const O_WRONLY: u32 = 2;
pub const O_RDWR: u32 = 3;
pub const O_ACCMODE: u32 = 3;
pub const O_LARGEFILE: u32 = 0;
pub const O_CREAT: u32 = 16;
pub const O_EXCL: u32 = 32;
pub const O_NOFOLLOW: u32 = 1048576;
pub const O_DIRECTORY: u32 = 2097152;
pub const O_APPEND: u32 = 256;
pub const O_ASYNC: u32 = 512;
pub const O_FSYNC: u32 = 1024;
pub const O_SYNC: u32 = 1024;
pub const O_SHLOCK: u32 = 131072;
pub const O_EXLOCK: u32 = 262144;
pub const O_DSYNC: u32 = 1024;
pub const O_RSYNC: u32 = 1024;
pub const O_NONBLOCK: u32 = 8;
pub const O_NDELAY: u32 = 8;
pub const O_TRUNC: u32 = 65536;
pub const O_CLOEXEC: u32 = 4194304;
pub const O_NOCTTY: u32 = 0;
pub const FREAD: u32 = 1;
pub const FWRITE: u32 = 2;
pub const FASYNC: u32 = 512;
pub const FCREAT: u32 = 16;
pub const FEXCL: u32 = 32;
pub const FTRUNC: u32 = 65536;
pub const FNOCTTY: u32 = 0;
pub const FFSYNC: u32 = 1024;
pub const FSYNC: u32 = 1024;
pub const FAPPEND: u32 = 256;
pub const FNONBLOCK: u32 = 8;
pub const FNDELAY: u32 = 8;
pub const F_DUPFD: u32 = 0;
pub const F_GETFD: u32 = 1;
pub const F_SETFD: u32 = 2;
pub const F_GETFL: u32 = 3;
pub const F_SETFL: u32 = 4;
pub const F_GETOWN: u32 = 5;
pub const F_SETOWN: u32 = 6;
pub const F_GETLK: u32 = 7;
pub const F_SETLK: u32 = 8;
pub const F_SETLKW: u32 = 9;
pub const F_GETLK64: u32 = 10;
pub const F_SETLK64: u32 = 11;
pub const F_SETLKW64: u32 = 12;
pub const F_DUPFD_CLOEXEC: u32 = 1030;
pub const F_RDLCK: u32 = 1;
pub const F_WRLCK: u32 = 2;
pub const F_UNLCK: u32 = 3;
pub const POSIX_FADV_NORMAL: u32 = 0;
pub const POSIX_FADV_RANDOM: u32 = 1;
pub const POSIX_FADV_SEQUENTIAL: u32 = 2;
pub const POSIX_FADV_WILLNEED: u32 = 3;
pub const POSIX_FADV_DONTNEED: u32 = 4;
pub const POSIX_FADV_NOREUSE: u32 = 5;
pub const AT_FDCWD: i32 = -100;
pub const AT_SYMLINK_NOFOLLOW: u32 = 256;
pub const AT_REMOVEDIR: u32 = 512;
pub const AT_SYMLINK_FOLLOW: u32 = 1024;
pub const AT_EACCESS: u32 = 512;
pub const _BITS_STAT_H: u32 = 1;
pub const __S_IFMT: u32 = 61440;
pub const __S_IFDIR: u32 = 16384;
pub const __S_IFCHR: u32 = 8192;
pub const __S_IFBLK: u32 = 24576;
pub const __S_IFREG: u32 = 32768;
pub const __S_IFLNK: u32 = 40960;
pub const __S_IFSOCK: u32 = 49152;
pub const __S_IFIFO: u32 = 4096;
//pub const __S_ISUID: u32 = 2048;
//pub const __S_ISGID: u32 = 1024;
//pub const __S_ISVTX: u32 = 512;
pub const __S_IREAD: u32 = 256;
pub const __S_IWRITE: u32 = 128;
pub const __S_IEXEC: u32 = 64;
pub const CMASK: u32 = 18;
pub const UF_SETTABLE: u32 = 65535;
pub const UF_NODUMP: u32 = 1;
pub const UF_IMMUTABLE: u32 = 2;
pub const UF_APPEND: u32 = 4;
pub const UF_OPAQUE: u32 = 8;
pub const UF_NOUNLINK: u32 = 16;
pub const SF_SETTABLE: u32 = 4294901760;
pub const SF_ARCHIVED: u32 = 65536;
pub const SF_IMMUTABLE: u32 = 131072;
pub const SF_APPEND: u32 = 262144;
pub const SF_NOUNLINK: u32 = 1048576;
pub const SF_SNAPSHOT: u32 = 2097152;
pub const UTIME_NOW: i32 = -1;
pub const UTIME_OMIT: i32 = -2;
pub const S_IFMT: u32 = 61440;
pub const S_IFDIR: u32 = 16384;
pub const S_IFCHR: u32 = 8192;
pub const S_IFBLK: u32 = 24576;
pub const S_IFREG: u32 = 32768;
pub const S_IFIFO: u32 = 4096;
pub const S_IFLNK: u32 = 40960;
pub const S_IFSOCK: u32 = 49152;
//pub const S_ISUID: u32 = 2048;
//pub const S_ISGID: u32 = 1024;
//pub const S_ISVTX: u32 = 512;
pub const S_IRUSR: u32 = 256;
pub const S_IWUSR: u32 = 128;
pub const S_IXUSR: u32 = 64;
pub const S_IRWXU: u32 = 448;
pub const S_IRGRP: u32 = 32;
pub const S_IWGRP: u32 = 16;
pub const S_IXGRP: u32 = 8;
pub const S_IRWXG: u32 = 56;
pub const S_IROTH: u32 = 4;
pub const S_IWOTH: u32 = 2;
pub const S_IXOTH: u32 = 1;
pub const S_IRWXO: u32 = 7;
pub const R_OK: u32 = 4;
pub const W_OK: u32 = 2;
pub const X_OK: u32 = 1;
pub const F_OK: u32 = 0;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const F_ULOCK: u32 = 0;
pub const F_LOCK: u32 = 1;
pub const F_TLOCK: u32 = 2;
pub const F_TEST: u32 = 3;
pub const _FENV_H: u32 = 1;
pub const FE_INVALID: u32 = 1;
pub const FE_DIVBYZERO: u32 = 4;
pub const FE_OVERFLOW: u32 = 8;
pub const FE_UNDERFLOW: u32 = 16;
pub const FE_INEXACT: u32 = 32;
pub const FE_ALL_EXCEPT: u32 = 61;
pub const FE_TONEAREST: u32 = 0;
pub const FE_DOWNWARD: u32 = 1024;
pub const FE_UPWARD: u32 = 2048;
pub const FE_TOWARDZERO: u32 = 3072;
pub const __FMTMSG_H: u32 = 1;
pub const MM_NULLSEV: u32 = 0;
pub const _FNMATCH_H: u32 = 1;
pub const FNM_PATHNAME: u32 = 1;
pub const FNM_NOESCAPE: u32 = 2;
pub const FNM_PERIOD: u32 = 4;
pub const FNM_NOMATCH: u32 = 1;
pub const _FTW_H: u32 = 1;
pub const _SYS_STAT_H: u32 = 1;
pub const S_IREAD: u32 = 256;
pub const S_IWRITE: u32 = 128;
pub const S_IEXEC: u32 = 64;
pub const ACCESSPERMS: u32 = 511;
pub const ALLPERMS: u32 = 4095;
pub const DEFFILEMODE: u32 = 438;
pub const S_BLKSIZE: u32 = 512;
pub const _STAT_VER: u32 = 0;
pub const _MKNOD_VER: u32 = 0;
pub const _GLOB_H: u32 = 1;
pub const GLOB_ERR: u32 = 1;
pub const GLOB_MARK: u32 = 2;
pub const GLOB_NOSORT: u32 = 4;
pub const GLOB_DOOFFS: u32 = 8;
pub const GLOB_NOCHECK: u32 = 16;
pub const GLOB_APPEND: u32 = 32;
pub const GLOB_NOESCAPE: u32 = 64;
pub const GLOB_PERIOD: u32 = 128;
pub const GLOB_MAGCHAR: u32 = 256;
pub const GLOB_ALTDIRFUNC: u32 = 512;
pub const GLOB_BRACE: u32 = 1024;
pub const GLOB_NOMAGIC: u32 = 2048;
pub const GLOB_TILDE: u32 = 4096;
pub const GLOB_ONLYDIR: u32 = 8192;
pub const GLOB_TILDE_CHECK: u32 = 16384;
pub const __GLOB_FLAGS: u32 = 32511;
pub const GLOB_NOSPACE: u32 = 1;
pub const GLOB_ABORTED: u32 = 2;
pub const GLOB_NOMATCH: u32 = 3;
pub const GLOB_NOSYS: u32 = 4;
pub const _GRP_H: u32 = 1;
pub const __FILE_defined: u32 = 1;
pub const NSS_BUFLEN_GROUP: u32 = 1024;
pub const _ICONV_H: u32 = 1;
pub const _INTTYPES_H: u32 = 1;
pub const _STDINT_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -2147483648;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 2147483647;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 4294967295;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const INTPTR_MIN: i32 = -2147483648;
pub const INTPTR_MAX: u32 = 2147483647;
pub const UINTPTR_MAX: u32 = 4294967295;
pub const PTRDIFF_MIN: i32 = -2147483648;
pub const PTRDIFF_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: u32 = 4294967295;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const ____gwchar_t_defined: u32 = 1;
pub const __PRI64_PREFIX: &'static [u8; 3usize] = b"ll\0";
pub const PRId8: &'static [u8; 2usize] = b"d\0";
pub const PRId16: &'static [u8; 2usize] = b"d\0";
pub const PRId32: &'static [u8; 2usize] = b"d\0";
pub const PRId64: &'static [u8; 4usize] = b"lld\0";
pub const PRIdLEAST8: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST16: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST32: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST64: &'static [u8; 4usize] = b"lld\0";
pub const PRIdFAST8: &'static [u8; 2usize] = b"d\0";
pub const PRIdFAST64: &'static [u8; 4usize] = b"lld\0";
pub const PRIi8: &'static [u8; 2usize] = b"i\0";
pub const PRIi16: &'static [u8; 2usize] = b"i\0";
pub const PRIi32: &'static [u8; 2usize] = b"i\0";
pub const PRIi64: &'static [u8; 4usize] = b"lli\0";
pub const PRIiLEAST8: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST16: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST32: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST64: &'static [u8; 4usize] = b"lli\0";
pub const PRIiFAST8: &'static [u8; 2usize] = b"i\0";
pub const PRIiFAST64: &'static [u8; 4usize] = b"lli\0";
pub const PRIo8: &'static [u8; 2usize] = b"o\0";
pub const PRIo16: &'static [u8; 2usize] = b"o\0";
pub const PRIo32: &'static [u8; 2usize] = b"o\0";
pub const PRIo64: &'static [u8; 4usize] = b"llo\0";
pub const PRIoLEAST8: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST16: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST32: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST64: &'static [u8; 4usize] = b"llo\0";
pub const PRIoFAST8: &'static [u8; 2usize] = b"o\0";
pub const PRIoFAST64: &'static [u8; 4usize] = b"llo\0";
pub const PRIu8: &'static [u8; 2usize] = b"u\0";
pub const PRIu16: &'static [u8; 2usize] = b"u\0";
pub const PRIu32: &'static [u8; 2usize] = b"u\0";
pub const PRIu64: &'static [u8; 4usize] = b"llu\0";
pub const PRIuLEAST8: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST16: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST32: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST64: &'static [u8; 4usize] = b"llu\0";
pub const PRIuFAST8: &'static [u8; 2usize] = b"u\0";
pub const PRIuFAST64: &'static [u8; 4usize] = b"llu\0";
pub const PRIx8: &'static [u8; 2usize] = b"x\0";
pub const PRIx16: &'static [u8; 2usize] = b"x\0";
pub const PRIx32: &'static [u8; 2usize] = b"x\0";
pub const PRIx64: &'static [u8; 4usize] = b"llx\0";
pub const PRIxLEAST8: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST16: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST32: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST64: &'static [u8; 4usize] = b"llx\0";
pub const PRIxFAST8: &'static [u8; 2usize] = b"x\0";
pub const PRIxFAST64: &'static [u8; 4usize] = b"llx\0";
pub const PRIX8: &'static [u8; 2usize] = b"X\0";
pub const PRIX16: &'static [u8; 2usize] = b"X\0";
pub const PRIX32: &'static [u8; 2usize] = b"X\0";
pub const PRIX64: &'static [u8; 4usize] = b"llX\0";
pub const PRIXLEAST8: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST16: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST32: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST64: &'static [u8; 4usize] = b"llX\0";
pub const PRIXFAST8: &'static [u8; 2usize] = b"X\0";
pub const PRIXFAST64: &'static [u8; 4usize] = b"llX\0";
pub const PRIdMAX: &'static [u8; 4usize] = b"lld\0";
pub const PRIiMAX: &'static [u8; 4usize] = b"lli\0";
pub const PRIoMAX: &'static [u8; 4usize] = b"llo\0";
pub const PRIuMAX: &'static [u8; 4usize] = b"llu\0";
pub const PRIxMAX: &'static [u8; 4usize] = b"llx\0";
pub const PRIXMAX: &'static [u8; 4usize] = b"llX\0";
pub const SCNd8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNd16: &'static [u8; 3usize] = b"hd\0";
pub const SCNd32: &'static [u8; 2usize] = b"d\0";
pub const SCNd64: &'static [u8; 4usize] = b"lld\0";
pub const SCNdLEAST8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNdLEAST16: &'static [u8; 3usize] = b"hd\0";
pub const SCNdLEAST32: &'static [u8; 2usize] = b"d\0";
pub const SCNdLEAST64: &'static [u8; 4usize] = b"lld\0";
pub const SCNdFAST8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNdFAST64: &'static [u8; 4usize] = b"lld\0";
pub const SCNi8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNi16: &'static [u8; 3usize] = b"hi\0";
pub const SCNi32: &'static [u8; 2usize] = b"i\0";
pub const SCNi64: &'static [u8; 4usize] = b"lli\0";
pub const SCNiLEAST8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNiLEAST16: &'static [u8; 3usize] = b"hi\0";
pub const SCNiLEAST32: &'static [u8; 2usize] = b"i\0";
pub const SCNiLEAST64: &'static [u8; 4usize] = b"lli\0";
pub const SCNiFAST8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNiFAST64: &'static [u8; 4usize] = b"lli\0";
pub const SCNu8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNu16: &'static [u8; 3usize] = b"hu\0";
pub const SCNu32: &'static [u8; 2usize] = b"u\0";
pub const SCNu64: &'static [u8; 4usize] = b"llu\0";
pub const SCNuLEAST8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNuLEAST16: &'static [u8; 3usize] = b"hu\0";
pub const SCNuLEAST32: &'static [u8; 2usize] = b"u\0";
pub const SCNuLEAST64: &'static [u8; 4usize] = b"llu\0";
pub const SCNuFAST8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNuFAST64: &'static [u8; 4usize] = b"llu\0";
pub const SCNo8: &'static [u8; 4usize] = b"hho\0";
pub const SCNo16: &'static [u8; 3usize] = b"ho\0";
pub const SCNo32: &'static [u8; 2usize] = b"o\0";
pub const SCNo64: &'static [u8; 4usize] = b"llo\0";
pub const SCNoLEAST8: &'static [u8; 4usize] = b"hho\0";
pub const SCNoLEAST16: &'static [u8; 3usize] = b"ho\0";
pub const SCNoLEAST32: &'static [u8; 2usize] = b"o\0";
pub const SCNoLEAST64: &'static [u8; 4usize] = b"llo\0";
pub const SCNoFAST8: &'static [u8; 4usize] = b"hho\0";
pub const SCNoFAST64: &'static [u8; 4usize] = b"llo\0";
pub const SCNx8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNx16: &'static [u8; 3usize] = b"hx\0";
pub const SCNx32: &'static [u8; 2usize] = b"x\0";
pub const SCNx64: &'static [u8; 4usize] = b"llx\0";
pub const SCNxLEAST8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNxLEAST16: &'static [u8; 3usize] = b"hx\0";
pub const SCNxLEAST32: &'static [u8; 2usize] = b"x\0";
pub const SCNxLEAST64: &'static [u8; 4usize] = b"llx\0";
pub const SCNxFAST8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNxFAST64: &'static [u8; 4usize] = b"llx\0";
pub const SCNdMAX: &'static [u8; 4usize] = b"lld\0";
pub const SCNiMAX: &'static [u8; 4usize] = b"lli\0";
pub const SCNoMAX: &'static [u8; 4usize] = b"llo\0";
pub const SCNuMAX: &'static [u8; 4usize] = b"llu\0";
pub const SCNxMAX: &'static [u8; 4usize] = b"llx\0";
pub const _LANGINFO_H: u32 = 1;
pub const _NL_TYPES_H: u32 = 1;
pub const NL_SETD: u32 = 1;
pub const NL_CAT_LOCALE: u32 = 1;
pub const _BITS_LOCALE_H: u32 = 1;
pub const __LC_CTYPE: u32 = 0;
pub const __LC_NUMERIC: u32 = 1;
pub const __LC_TIME: u32 = 2;
pub const __LC_COLLATE: u32 = 3;
pub const __LC_MONETARY: u32 = 4;
pub const __LC_MESSAGES: u32 = 5;
pub const __LC_ALL: u32 = 6;
pub const __LC_PAPER: u32 = 7;
pub const __LC_NAME: u32 = 8;
pub const __LC_ADDRESS: u32 = 9;
pub const __LC_TELEPHONE: u32 = 10;
pub const __LC_MEASUREMENT: u32 = 11;
pub const __LC_IDENTIFICATION: u32 = 12;
pub const _LIBGEN_H: u32 = 1;
pub const _LIBC_LIMITS_H_: u32 = 1;
pub const MB_LEN_MAX: u32 = 16;
pub const _BITS_POSIX2_LIM_H: u32 = 1;
pub const _POSIX2_BC_BASE_MAX: u32 = 99;
pub const _POSIX2_BC_DIM_MAX: u32 = 2048;
pub const _POSIX2_BC_SCALE_MAX: u32 = 99;
pub const _POSIX2_BC_STRING_MAX: u32 = 1000;
pub const _POSIX2_COLL_WEIGHTS_MAX: u32 = 2;
pub const _POSIX2_EXPR_NEST_MAX: u32 = 32;
pub const _POSIX2_LINE_MAX: u32 = 2048;
pub const _POSIX2_RE_DUP_MAX: u32 = 255;
pub const _POSIX2_CHARCLASS_NAME_MAX: u32 = 14;
pub const BC_BASE_MAX: u32 = 99;
pub const BC_DIM_MAX: u32 = 2048;
pub const BC_SCALE_MAX: u32 = 99;
pub const BC_STRING_MAX: u32 = 1000;
pub const COLL_WEIGHTS_MAX: u32 = 255;
pub const EXPR_NEST_MAX: u32 = 32;
pub const LINE_MAX: u32 = 2048;
pub const CHARCLASS_NAME_MAX: u32 = 2048;
pub const RE_DUP_MAX: u32 = 32767;
pub const _LOCALE_H: u32 = 1;
pub const LC_CTYPE: u32 = 0;
pub const LC_NUMERIC: u32 = 1;
pub const LC_TIME: u32 = 2;
pub const LC_COLLATE: u32 = 3;
pub const LC_MONETARY: u32 = 4;
pub const LC_MESSAGES: u32 = 5;
pub const LC_ALL: u32 = 6;
pub const LC_PAPER: u32 = 7;
pub const LC_NAME: u32 = 8;
pub const LC_ADDRESS: u32 = 9;
pub const LC_TELEPHONE: u32 = 10;
pub const LC_MEASUREMENT: u32 = 11;
pub const LC_IDENTIFICATION: u32 = 12;
pub const LC_CTYPE_MASK: u32 = 1;
pub const LC_NUMERIC_MASK: u32 = 2;
pub const LC_TIME_MASK: u32 = 4;
pub const LC_COLLATE_MASK: u32 = 8;
pub const LC_MONETARY_MASK: u32 = 16;
pub const LC_MESSAGES_MASK: u32 = 32;
pub const LC_PAPER_MASK: u32 = 128;
pub const LC_NAME_MASK: u32 = 256;
pub const LC_ADDRESS_MASK: u32 = 512;
pub const LC_TELEPHONE_MASK: u32 = 1024;
pub const LC_MEASUREMENT_MASK: u32 = 2048;
pub const LC_IDENTIFICATION_MASK: u32 = 4096;
pub const LC_ALL_MASK: u32 = 8127;
pub const _MATH_H: u32 = 1;
pub const _BITS_LIBM_SIMD_DECL_STUBS_H: u32 = 1;
pub const __FP_LOGB0_IS_MIN: u32 = 1;
pub const __FP_LOGBNAN_IS_MIN: u32 = 1;
pub const FP_ILOGB0: i32 = -2147483648;
pub const FP_ILOGBNAN: i32 = -2147483648;
pub const __MATH_DECLARING_DOUBLE: u32 = 1;
pub const __MATH_DECLARING_FLOATN: u32 = 0;
pub const __MATH_DECLARE_LDOUBLE: u32 = 1;
pub const FP_NAN: u32 = 0;
pub const FP_INFINITE: u32 = 1;
pub const FP_ZERO: u32 = 2;
pub const FP_SUBNORMAL: u32 = 3;
pub const FP_NORMAL: u32 = 4;
pub const MATH_ERRNO: u32 = 1;
pub const MATH_ERREXCEPT: u32 = 2;
pub const math_errhandling: u32 = 3;
pub const M_E: f64 = 2.718281828459045;
pub const M_LOG2E: f64 = 1.4426950408889634;
pub const M_LOG10E: f64 = 0.4342944819032518;
pub const M_LN2: f64 = 0.6931471805599453;
pub const M_LN10: f64 = 2.302585092994046;
pub const M_PI: f64 = 3.141592653589793;
pub const M_PI_2: f64 = 1.5707963267948966;
pub const M_PI_4: f64 = 0.7853981633974483;
pub const M_1_PI: f64 = 0.3183098861837907;
pub const M_2_PI: f64 = 0.6366197723675814;
pub const M_2_SQRTPI: f64 = 1.1283791670955126;
pub const M_SQRT2: f64 = 1.4142135623730951;
pub const M_SQRT1_2: f64 = 0.7071067811865476;
pub const _MONETARY_H: u32 = 1;
pub const _MQUEUE_H: u32 = 1;
pub const _NETDB_H: u32 = 1;
pub const _NETINET_IN_H: u32 = 1;
pub const _SYS_SOCKET_H: u32 = 1;
pub const __iovec_defined: u32 = 1;
pub const __BITS_SOCKET_H: u32 = 1;
pub const SOCK_TYPE_MASK: u32 = 15;
pub const PF_UNSPEC: u32 = 0;
pub const PF_LOCAL: u32 = 1;
pub const PF_UNIX: u32 = 1;
pub const PF_FILE: u32 = 1;
pub const PF_INET: u32 = 2;
pub const PF_IMPLINK: u32 = 3;
pub const PF_PUP: u32 = 4;
pub const PF_CHAOS: u32 = 5;
pub const PF_NS: u32 = 6;
pub const PF_ISO: u32 = 7;
pub const PF_OSI: u32 = 7;
pub const PF_ECMA: u32 = 8;
pub const PF_DATAKIT: u32 = 9;
pub const PF_CCITT: u32 = 10;
pub const PF_SNA: u32 = 11;
pub const PF_DECnet: u32 = 12;
pub const PF_DLI: u32 = 13;
pub const PF_LAT: u32 = 14;
pub const PF_HYLINK: u32 = 15;
pub const PF_APPLETALK: u32 = 16;
pub const PF_ROUTE: u32 = 17;
pub const PF_XTP: u32 = 19;
pub const PF_COIP: u32 = 20;
pub const PF_CNT: u32 = 21;
pub const PF_RTIP: u32 = 22;
pub const PF_IPX: u32 = 23;
pub const PF_SIP: u32 = 24;
pub const PF_PIP: u32 = 25;
pub const PF_INET6: u32 = 26;
pub const PF_MAX: u32 = 27;
pub const AF_UNSPEC: u32 = 0;
pub const AF_LOCAL: u32 = 1;
pub const AF_UNIX: u32 = 1;
pub const AF_FILE: u32 = 1;
pub const AF_INET: u32 = 2;
pub const AF_IMPLINK: u32 = 3;
pub const AF_PUP: u32 = 4;
pub const AF_CHAOS: u32 = 5;
pub const AF_NS: u32 = 6;
pub const AF_ISO: u32 = 7;
pub const AF_OSI: u32 = 7;
pub const AF_ECMA: u32 = 8;
pub const AF_DATAKIT: u32 = 9;
pub const AF_CCITT: u32 = 10;
pub const AF_SNA: u32 = 11;
pub const AF_DECnet: u32 = 12;
pub const AF_DLI: u32 = 13;
pub const AF_LAT: u32 = 14;
pub const AF_HYLINK: u32 = 15;
pub const AF_APPLETALK: u32 = 16;
pub const AF_ROUTE: u32 = 17;
pub const pseudo_AF_XTP: u32 = 19;
pub const AF_COIP: u32 = 20;
pub const AF_CNT: u32 = 21;
pub const pseudo_AF_RTIP: u32 = 22;
pub const AF_IPX: u32 = 23;
pub const AF_SIP: u32 = 24;
pub const pseudo_AF_PIP: u32 = 25;
pub const AF_INET6: u32 = 26;
pub const AF_MAX: u32 = 27;
pub const SOMAXCONN: u32 = 128;
pub const _BITS_SOCKADDR_H: u32 = 1;
pub const _HAVE_SA_LEN: u32 = 1;
pub const _SS_SIZE: u32 = 128;
pub const CMGROUP_MAX: u32 = 16;
pub const SOL_SOCKET: u32 = 65535;
pub const __osockaddr_defined: u32 = 1;
pub const __USE_KERNEL_IPV6_DEFS: u32 = 0;
pub const SOL_IP: u32 = 0;
pub const IP_OPTIONS: u32 = 1;
pub const IP_HDRINCL: u32 = 2;
pub const IP_TOS: u32 = 3;
pub const IP_TTL: u32 = 4;
pub const IP_RECVOPTS: u32 = 5;
pub const IP_RECVRETOPTS: u32 = 6;
pub const IP_RECVDSTADDR: u32 = 7;
pub const IP_RETOPTS: u32 = 8;
pub const IP_MULTICAST_IF: u32 = 9;
pub const IP_MULTICAST_TTL: u32 = 10;
pub const IP_MULTICAST_LOOP: u32 = 11;
pub const IP_ADD_MEMBERSHIP: u32 = 12;
pub const IP_DROP_MEMBERSHIP: u32 = 13;
pub const SOL_IPV6: u32 = 41;
pub const SOL_ICMPV6: u32 = 58;
pub const IPV6_ADDRFORM: u32 = 1;
pub const IPV6_2292PKTINFO: u32 = 2;
pub const IPV6_2292HOPOPTS: u32 = 3;
pub const IPV6_2292DSTOPTS: u32 = 4;
pub const IPV6_2292RTHDR: u32 = 5;
pub const IPV6_2292PKTOPTIONS: u32 = 6;
pub const IPV6_CHECKSUM: u32 = 7;
pub const IPV6_2292HOPLIMIT: u32 = 8;
pub const IPV6_RXINFO: u32 = 2;
pub const IPV6_TXINFO: u32 = 2;
pub const SCM_SRCINFO: u32 = 2;
pub const IPV6_UNICAST_HOPS: u32 = 16;
pub const IPV6_MULTICAST_IF: u32 = 17;
pub const IPV6_MULTICAST_HOPS: u32 = 18;
pub const IPV6_MULTICAST_LOOP: u32 = 19;
pub const IPV6_JOIN_GROUP: u32 = 20;
pub const IPV6_LEAVE_GROUP: u32 = 21;
pub const IPV6_ROUTER_ALERT: u32 = 22;
pub const IPV6_MTU_DISCOVER: u32 = 23;
pub const IPV6_MTU: u32 = 24;
pub const IPV6_RECVERR: u32 = 25;
pub const IPV6_V6ONLY: u32 = 26;
pub const IPV6_JOIN_ANYCAST: u32 = 27;
pub const IPV6_LEAVE_ANYCAST: u32 = 28;
pub const IPV6_RECVPKTINFO: u32 = 49;
pub const IPV6_PKTINFO: u32 = 50;
pub const IPV6_RECVHOPLIMIT: u32 = 51;
pub const IPV6_HOPLIMIT: u32 = 52;
pub const IPV6_RECVHOPOPTS: u32 = 53;
pub const IPV6_HOPOPTS: u32 = 54;
pub const IPV6_RTHDRDSTOPTS: u32 = 55;
pub const IPV6_RECVRTHDR: u32 = 56;
pub const IPV6_RTHDR: u32 = 57;
pub const IPV6_RECVDSTOPTS: u32 = 58;
pub const IPV6_DSTOPTS: u32 = 59;
pub const IPV6_RECVPATHMTU: u32 = 60;
pub const IPV6_PATHMTU: u32 = 61;
pub const IPV6_DONTFRAG: u32 = 62;
pub const IPV6_ADD_MEMBERSHIP: u32 = 20;
pub const IPV6_DROP_MEMBERSHIP: u32 = 21;
pub const IPV6_RXHOPOPTS: u32 = 3;
pub const IPV6_RXDSTOPTS: u32 = 4;
pub const IPV6_RTHDR_LOOSE: u32 = 0;
pub const IPV6_RTHDR_STRICT: u32 = 1;
pub const IPV6_RTHDR_TYPE_0: u32 = 0;
pub const IN_CLASSA_NET: u32 = 4278190080;
pub const IN_CLASSA_NSHIFT: u32 = 24;
pub const IN_CLASSA_HOST: u32 = 16777215;
pub const IN_CLASSA_MAX: u32 = 128;
pub const IN_CLASSB_NET: u32 = 4294901760;
pub const IN_CLASSB_NSHIFT: u32 = 16;
pub const IN_CLASSB_HOST: u32 = 65535;
pub const IN_CLASSB_MAX: u32 = 65536;
pub const IN_CLASSC_NET: u32 = 4294967040;
pub const IN_CLASSC_NSHIFT: u32 = 8;
pub const IN_CLASSC_HOST: u32 = 255;
pub const IN_LOOPBACKNET: u32 = 127;
pub const INET_ADDRSTRLEN: u32 = 16;
pub const INET6_ADDRSTRLEN: u32 = 46;
pub const _RPC_NETDB_H: u32 = 1;
pub const _PATH_HEQUIV: &'static [u8; 17usize] = b"/etc/hosts.equiv\0";
pub const _PATH_HOSTS: &'static [u8; 11usize] = b"/etc/hosts\0";
pub const _PATH_NETWORKS: &'static [u8; 14usize] = b"/etc/networks\0";
pub const _PATH_NSSWITCH_CONF: &'static [u8; 19usize] = b"/etc/nsswitch.conf\0";
pub const _PATH_PROTOCOLS: &'static [u8; 15usize] = b"/etc/protocols\0";
pub const _PATH_SERVICES: &'static [u8; 14usize] = b"/etc/services\0";
pub const HOST_NOT_FOUND: u32 = 1;
pub const TRY_AGAIN: u32 = 2;
pub const NO_RECOVERY: u32 = 3;
pub const NO_DATA: u32 = 4;
pub const NETDB_INTERNAL: i32 = -1;
pub const NETDB_SUCCESS: u32 = 0;
pub const NO_ADDRESS: u32 = 4;
pub const AI_PASSIVE: u32 = 1;
pub const AI_CANONNAME: u32 = 2;
pub const AI_NUMERICHOST: u32 = 4;
pub const AI_V4MAPPED: u32 = 8;
pub const AI_ALL: u32 = 16;
pub const AI_ADDRCONFIG: u32 = 32;
pub const AI_NUMERICSERV: u32 = 1024;
pub const EAI_BADFLAGS: i32 = -1;
pub const EAI_NONAME: i32 = -2;
pub const EAI_AGAIN: i32 = -3;
pub const EAI_FAIL: i32 = -4;
pub const EAI_FAMILY: i32 = -6;
pub const EAI_SOCKTYPE: i32 = -7;
pub const EAI_SERVICE: i32 = -8;
pub const EAI_MEMORY: i32 = -10;
pub const EAI_SYSTEM: i32 = -11;
pub const EAI_OVERFLOW: i32 = -12;
pub const NI_MAXHOST: u32 = 1025;
pub const NI_MAXSERV: u32 = 32;
pub const NI_NUMERICHOST: u32 = 1;
pub const NI_NUMERICSERV: u32 = 2;
pub const NI_NOFQDN: u32 = 4;
pub const NI_NAMEREQD: u32 = 8;
pub const NI_DGRAM: u32 = 16;
pub const _SYS_POLL_H: u32 = 1;
pub const POLLIN: u32 = 1;
pub const POLLPRI: u32 = 2;
pub const POLLOUT: u32 = 4;
pub const POLLRDNORM: u32 = 1;
pub const POLLRDBAND: u32 = 2;
pub const POLLWRNORM: u32 = 4;
pub const POLLWRBAND: u32 = 4;
pub const POLLERR: u32 = 8;
pub const POLLHUP: u32 = 16;
pub const POLLNVAL: u32 = 32;
pub const _PTHREAD_H: u32 = 1;
pub const _SCHED_H: u32 = 1;
pub const _BITS_SCHED_H: u32 = 1;
pub const SCHED_OTHER: u32 = 0;
pub const SCHED_FIFO: u32 = 1;
pub const SCHED_RR: u32 = 2;
pub const _BITS_CPU_SET_H: u32 = 1;
pub const __CPU_SETSIZE: u32 = 1024;
pub const _TIME_H: u32 = 1;
pub const _BITS_TIME_H: u32 = 1;
pub const CLOCK_REALTIME: u32 = 0;
pub const CLOCK_MONOTONIC: u32 = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: u32 = 2;
pub const CLOCK_THREAD_CPUTIME_ID: u32 = 3;
pub const CLOCK_MONOTONIC_RAW: u32 = 4;
pub const CLOCK_REALTIME_COARSE: u32 = 5;
pub const CLOCK_MONOTONIC_COARSE: u32 = 6;
pub const TIMER_ABSTIME: u32 = 1;
pub const __struct_tm_defined: u32 = 1;
pub const __itimerspec_defined: u32 = 1;
pub const TIME_UTC: u32 = 1;
pub const PTHREAD_SPINLOCK_INITIALIZER: u32 = 0;
pub const _BITS_CANCELATION_H: u32 = 1;
pub const PTHREAD_CANCEL_DISABLE: u32 = 0;
pub const PTHREAD_CANCEL_ENABLE: u32 = 1;
pub const PTHREAD_CANCEL_DEFERRED: u32 = 0;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: u32 = 1;
pub const PTHREAD_BARRIER_SERIAL_THREAD: i32 = -1;
pub const _BITS_PTHREAD_NP_H: u32 = 1;
pub const _PWD_H: u32 = 1;
pub const NSS_BUFLEN_PASSWD: u32 = 1024;
pub const _REGEX_H: u32 = 1;
pub const REG_EXTENDED: u32 = 1;
pub const REG_ICASE: u32 = 2;
pub const REG_NEWLINE: u32 = 4;
pub const REG_NOSUB: u32 = 8;
pub const REG_NOTBOL: u32 = 1;
pub const REG_NOTEOL: u32 = 2;
pub const REG_STARTEND: u32 = 4;
pub const _SEARCH_H: u32 = 1;
pub const _SEMAPHORE_H: u32 = 1;
pub const _BITS_SEMAPHORE_H: u32 = 1;
pub const __SIZEOF_SEM_T: u32 = 20;
pub const _SETJMP_H: u32 = 1;
pub const _BITS_SETJMP_H: u32 = 1;
pub const _BITS_SIGNUM_H: u32 = 1;
pub const _BITS_SIGNUM_GENERIC_H: u32 = 1;
pub const SIGINT: u32 = 2;
pub const SIGILL: u32 = 4;
pub const SIGABRT: u32 = 6;
pub const SIGFPE: u32 = 8;
pub const SIGSEGV: u32 = 11;
pub const SIGTERM: u32 = 15;
pub const SIGHUP: u32 = 1;
pub const SIGQUIT: u32 = 3;
pub const SIGTRAP: u32 = 5;
pub const SIGKILL: u32 = 9;
pub const SIGBUS: u32 = 10;
pub const SIGSYS: u32 = 12;
pub const SIGPIPE: u32 = 13;
pub const SIGALRM: u32 = 14;
pub const SIGURG: u32 = 16;
pub const SIGSTOP: u32 = 17;
pub const SIGTSTP: u32 = 18;
pub const SIGCONT: u32 = 19;
pub const SIGCHLD: u32 = 20;
pub const SIGTTIN: u32 = 21;
pub const SIGTTOU: u32 = 22;
pub const SIGPOLL: u32 = 23;
pub const SIGXCPU: u32 = 24;
pub const SIGXFSZ: u32 = 25;
pub const SIGVTALRM: u32 = 26;
pub const SIGPROF: u32 = 27;
pub const SIGUSR1: u32 = 30;
pub const SIGUSR2: u32 = 31;
pub const SIGWINCH: u32 = 28;
pub const SIGIO: u32 = 23;
//pub const SIGIOT: u32 = 6;
pub const SIGCLD: u32 = 20;
pub const __SIGRTMIN: u32 = 32;
pub const __SIGRTMAX: u32 = 32;
pub const _NSIG: u32 = 33;
pub const SIGEMT: u32 = 7;
pub const SIGINFO: u32 = 29;
pub const SIGLOST: u32 = 32;
pub const __sig_atomic_t_defined: u32 = 1;
pub const __siginfo_t_defined: u32 = 1;
pub const _BITS_SIGINFO_CONSTS_H: u32 = 1;
pub const NSIG: u32 = 33;
pub const _BITS_SIGACTION_H: u32 = 1;
pub const SA_ONSTACK: u32 = 1;
pub const SA_RESTART: u32 = 2;
pub const SA_NODEFER: u32 = 16;
pub const SA_RESETHAND: u32 = 4;
pub const SA_NOCLDSTOP: u32 = 8;
pub const SA_SIGINFO: u32 = 64;
pub const SA_INTERRUPT: u32 = 0;
pub const SA_NOMASK: u32 = 16;
pub const SA_ONESHOT: u32 = 4;
pub const SA_STACK: u32 = 1;
pub const SIG_BLOCK: u32 = 1;
pub const SIG_UNBLOCK: u32 = 2;
pub const SIG_SETMASK: u32 = 3;
pub const _BITS_SIGCONTEXT_H: u32 = 1;
pub const FPC_IE: u32 = 1;
pub const FPC_IM: u32 = 1;
pub const FPC_DE: u32 = 2;
pub const FPC_DM: u32 = 2;
pub const FPC_ZE: u32 = 4;
pub const FPC_ZM: u32 = 4;
pub const FPC_OE: u32 = 8;
pub const FPC_OM: u32 = 8;
pub const FPC_UE: u32 = 16;
pub const FPC_PE: u32 = 32;
pub const FPC_PC: u32 = 768;
pub const FPC_PC_24: u32 = 0;
pub const FPC_PC_53: u32 = 512;
pub const FPC_PC_64: u32 = 768;
pub const FPC_RC: u32 = 3072;
pub const FPC_RC_RN: u32 = 0;
pub const FPC_RC_RD: u32 = 1024;
pub const FPC_RC_RU: u32 = 2048;
pub const FPC_RC_CHOP: u32 = 3072;
pub const FPC_IC: u32 = 4096;
pub const FPC_IC_PROJ: u32 = 0;
pub const FPC_IC_AFF: u32 = 4096;
pub const FPS_IE: u32 = 1;
pub const FPS_DE: u32 = 2;
pub const FPS_ZE: u32 = 4;
pub const FPS_OE: u32 = 8;
pub const FPS_UE: u32 = 16;
pub const FPS_PE: u32 = 32;
pub const FPS_SF: u32 = 64;
pub const FPS_ES: u32 = 128;
pub const FPS_C0: u32 = 256;
pub const FPS_C1: u32 = 512;
pub const FPS_C2: u32 = 1024;
pub const FPS_TOS: u32 = 14336;
pub const FPS_TOS_SHIFT: u32 = 11;
pub const FPS_C3: u32 = 16384;
pub const FPS_BUSY: u32 = 32768;
pub const FPE_INTOVF_TRAP: u32 = 1;
pub const FPE_INTDIV_FAULT: u32 = 2;
pub const FPE_FLTOVF_FAULT: u32 = 3;
pub const FPE_FLTDIV_FAULT: u32 = 4;
pub const FPE_FLTUND_FAULT: u32 = 5;
pub const FPE_SUBRNG_FAULT: u32 = 7;
pub const FPE_FLTDNR_FAULT: u32 = 8;
pub const FPE_FLTINX_FAULT: u32 = 9;
pub const FPE_EMERR_FAULT: u32 = 10;
pub const FPE_EMBND_FAULT: u32 = 11;
pub const ILL_INVOPR_FAULT: u32 = 1;
pub const ILL_STACK_FAULT: u32 = 2;
pub const ILL_FPEOPR_FAULT: u32 = 3;
pub const DBG_SINGLE_TRAP: u32 = 1;
pub const DBG_BRKPNT_FAULT: u32 = 2;
pub const __stack_t_defined: u32 = 1;
pub const _SYS_UCONTEXT_H: u32 = 1;
pub const __NGREG: u32 = 19;
pub const NGREG: u32 = 19;
pub const _BITS_SIGSTACK_H: u32 = 1;
pub const MINSIGSTKSZ: u32 = 8192;
pub const SIGSTKSZ: u32 = 40960;
pub const _BITS_SS_FLAGS_H: u32 = 1;
pub const __sigstack_defined: u32 = 1;
pub const _BITS_SIGTHREAD_H: u32 = 1;
pub const _SPAWN_H: u32 = 1;
pub const POSIX_SPAWN_RESETIDS: u32 = 1;
pub const POSIX_SPAWN_SETPGROUP: u32 = 2;
pub const POSIX_SPAWN_SETSIGDEF: u32 = 4;
pub const POSIX_SPAWN_SETSIGMASK: u32 = 8;
pub const POSIX_SPAWN_SETSCHEDPARAM: u32 = 16;
pub const POSIX_SPAWN_SETSCHEDULER: u32 = 32;
pub const __GNUC_VA_LIST: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub const _STDIO_H: u32 = 1;
pub const _____fpos_t_defined: u32 = 1;
pub const ____mbstate_t_defined: u32 = 1;
pub const _____fpos64_t_defined: u32 = 1;
pub const ____FILE_defined: u32 = 1;
pub const __struct_FILE_defined: u32 = 1;
pub const _IO_EOF_SEEN: u32 = 16;
pub const _IO_ERR_SEEN: u32 = 32;
pub const _IO_USER_LOCK: u32 = 32768;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 8192;
pub const EOF: i32 = -1;
pub const P_tmpdir: &'static [u8; 5usize] = b"/tmp\0";
pub const _BITS_STDIO_LIM_H: u32 = 1;
pub const L_tmpnam: u32 = 20;
pub const TMP_MAX: u32 = 238328;
pub const FILENAME_MAX: u32 = 1024;
pub const L_ctermid: u32 = 9;
pub const FOPEN_MAX: u32 = 16;
pub const _STDLIB_H: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WSTOPPED: u32 = 2;
pub const WCONTINUED: u32 = 4;
pub const WNOWAIT: u32 = 8;
pub const WEXITED: u32 = 16;
pub const __ENUM_IDTYPE_T: u32 = 1;
pub const __W_CONTINUED: u32 = 65535;
pub const __WCOREFLAG: u32 = 128;
pub const __ldiv_t_defined: u32 = 1;
pub const __lldiv_t_defined: u32 = 1;
pub const RAND_MAX: u32 = 2147483647;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const _ALLOCA_H: u32 = 1;
pub const _STRING_H: u32 = 1;
pub const _STRINGS_H: u32 = 1;
pub const _SYS_IPC_H: u32 = 1;
pub const _BITS_IPCTYPES_H: u32 = 1;
pub const IPC_CREAT: u32 = 512;
pub const IPC_EXCL: u32 = 1024;
pub const IPC_NOWAIT: u32 = 2048;
pub const IPC_RMID: u32 = 0;
pub const IPC_SET: u32 = 1;
pub const IPC_STAT: u32 = 2;
pub const _SYS_MMAN_H: u32 = 1;
pub const PROT_NONE: u32 = 0;
pub const PROT_READ: u32 = 4;
pub const PROT_WRITE: u32 = 2;
pub const PROT_EXEC: u32 = 1;
pub const MAP_FILE: u32 = 1;
pub const MAP_ANON: u32 = 2;
pub const MAP_TYPE: u32 = 15;
pub const MAP_ANONYMOUS: u32 = 2;
pub const MAP_COPY: u32 = 32;
pub const MAP_SHARED: u32 = 16;
pub const MAP_PRIVATE: u32 = 0;
pub const MAP_FIXED: u32 = 256;
pub const MAP_NOEXTEND: u32 = 512;
pub const MAP_HASSEMPHORE: u32 = 1024;
pub const MAP_INHERIT: u32 = 2048;
pub const MADV_NORMAL: u32 = 0;
pub const MADV_RANDOM: u32 = 1;
pub const MADV_SEQUENTIAL: u32 = 2;
pub const MADV_WILLNEED: u32 = 3;
pub const MADV_DONTNEED: u32 = 4;
pub const POSIX_MADV_NORMAL: u32 = 0;
pub const POSIX_MADV_RANDOM: u32 = 1;
pub const POSIX_MADV_SEQUENTIAL: u32 = 2;
pub const POSIX_MADV_WILLNEED: u32 = 3;
pub const POSIX_MADV_DONTNEED: u32 = 4;
pub const MS_ASYNC: u32 = 1;
pub const MS_SYNC: u32 = 0;
pub const MS_INVALIDATE: u32 = 2;
pub const MCL_CURRENT: u32 = 1;
pub const MCL_FUTURE: u32 = 2;
pub const MSG_NOERROR: u32 = 4096;
pub const MSG_STAT: u32 = 11;
pub const MSG_INFO: u32 = 12;
pub const _SYS_RESOURCE_H: u32 = 1;
pub const RLIM_INFINITY: u32 = 2147483647;
pub const RLIM_SAVED_MAX: u32 = 2147483647;
pub const RLIM_SAVED_CUR: u32 = 2147483647;
pub const __rusage_defined: u32 = 1;
//pub const PRIO_MIN: i32 = -20;
//pub const PRIO_MAX: u32 = 20;
pub const _SYS_SEM_H: u32 = 1;
pub const SEM_UNDO: u32 = 4096;
pub const GETPID: u32 = 11;
pub const GETVAL: u32 = 12;
pub const GETALL: u32 = 13;
pub const GETNCNT: u32 = 14;
pub const GETZCNT: u32 = 15;
pub const SETVAL: u32 = 16;
pub const SETALL: u32 = 17;
pub const _SEM_SEMUN_UNDEFINED: u32 = 1;
pub const SEM_STAT: u32 = 18;
pub const SEM_INFO: u32 = 19;
pub const _SYS_SHM_H: u32 = 1;
pub const SHM_R: u32 = 256;
pub const SHM_W: u32 = 128;
pub const SHM_RDONLY: u32 = 4096;
pub const SHM_RND: u32 = 8192;
pub const SHM_REMAP: u32 = 16384;
pub const SHM_LOCK: u32 = 11;
pub const SHM_UNLOCK: u32 = 12;
pub const SHM_STAT: u32 = 13;
pub const SHM_INFO: u32 = 14;
pub const SHM_DEST: u32 = 512;
pub const SHM_LOCKED: u32 = 1024;
pub const _SYS_STATVFS_H: u32 = 1;
pub const _SYS_TIME_H: u32 = 1;
pub const _SYS_TIMES_H: u32 = 1;
pub const _SYS_UIO_H: u32 = 1;
pub const _BITS_UIO_LIM_H: u32 = 1;
pub const _SYS_UN_H: u32 = 1;
pub const _SYS_UTSNAME_H: u32 = 1;
pub const _UTSNAME_LENGTH: u32 = 1024;
pub const _UTSNAME_DOMAIN_LENGTH: u32 = 0;
pub const _UTSNAME_SYSNAME_LENGTH: u32 = 1024;
pub const _UTSNAME_NODENAME_LENGTH: u32 = 1024;
pub const _UTSNAME_RELEASE_LENGTH: u32 = 1024;
pub const _UTSNAME_VERSION_LENGTH: u32 = 1024;
pub const _UTSNAME_MACHINE_LENGTH: u32 = 1024;
pub const SYS_NMLN: u32 = 1024;
pub const _SYS_WAIT_H: u32 = 1;
pub const WCOREFLAG: u32 = 128;
pub const WAIT_ANY: i32 = -1;
pub const WAIT_MYPGRP: u32 = 0;
pub const _SYS_SYSLOG_H: u32 = 1;
pub const _BITS_SYSLOG_PATH_H: u32 = 1;
pub const _PATH_LOG: &'static [u8; 9usize] = b"/dev/log\0";
//pub const LOG_EMERG: u32 = 0;
//pub const LOG_ALERT: u32 = 1;
//pub const LOG_CRIT: u32 = 2;
//pub const LOG_ERR: u32 = 3;
//pub const LOG_WARNING: u32 = 4;
//pub const LOG_NOTICE: u32 = 5;
//pub const LOG_INFO: u32 = 6;
//pub const LOG_DEBUG: u32 = 7;
//pub const LOG_PRIMASK: u32 = 7;
//pub const LOG_KERN: u32 = 0;
//pub const LOG_USER: u32 = 8;
//pub const LOG_MAIL: u32 = 16;
//pub const LOG_DAEMON: u32 = 24;
//pub const LOG_AUTH: u32 = 32;
//pub const LOG_SYSLOG: u32 = 40;
//pub const LOG_LPR: u32 = 48;
//pub const LOG_NEWS: u32 = 56;
//pub const LOG_UUCP: u32 = 64;
//pub const LOG_CRON: u32 = 72;
//pub const LOG_AUTHPRIV: u32 = 80;
//pub const LOG_FTP: u32 = 88;
//pub const LOG_LOCAL0: u32 = 128;
//pub const LOG_LOCAL1: u32 = 136;
//pub const LOG_LOCAL2: u32 = 144;
//pub const LOG_LOCAL3: u32 = 152;
//pub const LOG_LOCAL4: u32 = 160;
//pub const LOG_LOCAL5: u32 = 168;
//pub const LOG_LOCAL6: u32 = 176;
//pub const LOG_LOCAL7: u32 = 184;
//pub const LOG_NFACILITIES: u32 = 24;
//pub const LOG_FACMASK: u32 = 1016;
//pub const LOG_PID: u32 = 1;
//pub const LOG_CONS: u32 = 2;
//pub const LOG_ODELAY: u32 = 4;
//pub const LOG_NDELAY: u32 = 8;
//pub const LOG_NOWAIT: u32 = 16;
pub const LOG_PERROR: u32 = 32;
pub const _TAR_H: u32 = 1;
pub const TSUID: u32 = 2048;
pub const TSGID: u32 = 1024;
pub const TUREAD: u32 = 256;
pub const TUWRITE: u32 = 128;
pub const TUEXEC: u32 = 64;
pub const TGREAD: u32 = 32;
pub const TGWRITE: u32 = 16;
pub const TGEXEC: u32 = 8;
pub const TOREAD: u32 = 4;
pub const TOWRITE: u32 = 2;
pub const TOEXEC: u32 = 1;
pub const REGTYPE: u8 = 48u8;
pub const AREGTYPE: u8 = 0u8;
pub const LNKTYPE: u8 = 49u8;
pub const SYMTYPE: u8 = 50u8;
pub const CHRTYPE: u8 = 51u8;
pub const BLKTYPE: u8 = 52u8;
pub const DIRTYPE: u8 = 53u8;
pub const FIFOTYPE: u8 = 54u8;
pub const CONTTYPE: u8 = 55u8;
pub const TMAGIC: &'static [u8; 6usize] = b"ustar\0";
pub const TMAGLEN: u32 = 6;
pub const TVERSION: &'static [u8; 3usize] = b"00\0";
pub const TVERSLEN: u32 = 2;
pub const _TERMIOS_H: u32 = 1;
pub const IGNBRK: u32 = 1;
pub const BRKINT: u32 = 2;
pub const IGNPAR: u32 = 4;
pub const PARMRK: u32 = 8;
pub const INPCK: u32 = 16;
pub const ISTRIP: u32 = 32;
pub const INLCR: u32 = 64;
pub const IGNCR: u32 = 128;
pub const ICRNL: u32 = 256;
pub const IXON: u32 = 512;
pub const IXOFF: u32 = 1024;
pub const IXANY: u32 = 2048;
pub const IMAXBEL: u32 = 8192;
pub const OPOST: u32 = 1;
pub const ONLCR: u32 = 2;
pub const ONOEOT: u32 = 8;
pub const OCRNL: u32 = 16;
pub const ONOCR: u32 = 32;
pub const ONLRET: u32 = 64;
pub const NLDLY: u32 = 768;
pub const NL0: u32 = 0;
pub const NL1: u32 = 256;
pub const TABDLY: u32 = 3076;
pub const TAB0: u32 = 0;
pub const TAB1: u32 = 1024;
pub const TAB2: u32 = 2048;
pub const TAB3: u32 = 4;
pub const CRDLY: u32 = 12288;
pub const CR0: u32 = 0;
pub const CR1: u32 = 4096;
pub const CR2: u32 = 8192;
pub const CR3: u32 = 12288;
pub const FFDLY: u32 = 16384;
pub const FF0: u32 = 0;
pub const FF1: u32 = 16384;
pub const BSDLY: u32 = 32768;
pub const BS0: u32 = 0;
pub const BS1: u32 = 32768;
pub const VTDLY: u32 = 65536;
pub const VT0: u32 = 0;
pub const VT1: u32 = 65536;
pub const CIGNORE: u32 = 1;
pub const CS5: u32 = 0;
pub const CS6: u32 = 256;
pub const CS7: u32 = 512;
pub const CS8: u32 = 768;
pub const CSTOPB: u32 = 1024;
pub const CREAD: u32 = 2048;
pub const PARENB: u32 = 4096;
pub const PARODD: u32 = 8192;
pub const HUPCL: u32 = 16384;
pub const CLOCAL: u32 = 32768;
pub const CRTSCTS: u32 = 65536;
pub const CRTS_IFLOW: u32 = 65536;
pub const CCTS_OFLOW: u32 = 65536;
pub const CDTRCTS: u32 = 131072;
pub const MDMBUF: u32 = 1048576;
pub const CHWFLOW: u32 = 1245184;
pub const ECHOKE: u32 = 1;
pub const _ECHOE: u32 = 2;
pub const ECHOE: u32 = 2;
pub const _ECHOK: u32 = 4;
pub const ECHOK: u32 = 4;
pub const _ECHO: u32 = 8;
pub const ECHO: u32 = 8;
pub const _ECHONL: u32 = 16;
pub const ECHONL: u32 = 16;
pub const ECHOPRT: u32 = 32;
pub const ECHOCTL: u32 = 64;
pub const _ISIG: u32 = 128;
pub const ISIG: u32 = 128;
pub const _ICANON: u32 = 256;
pub const ICANON: u32 = 256;
pub const ALTWERASE: u32 = 512;
pub const _IEXTEN: u32 = 1024;
pub const IEXTEN: u32 = 1024;
pub const EXTPROC: u32 = 2048;
pub const _TOSTOP: u32 = 4194304;
pub const TOSTOP: u32 = 4194304;
pub const FLUSHO: u32 = 8388608;
pub const NOKERNINFO: u32 = 33554432;
pub const PENDIN: u32 = 536870912;
pub const _NOFLSH: u32 = 2147483648;
pub const NOFLSH: u32 = 2147483648;
pub const VEOF: u32 = 0;
pub const VEOL: u32 = 1;
pub const VEOL2: u32 = 2;
pub const VERASE: u32 = 3;
pub const VWERASE: u32 = 4;
pub const VKILL: u32 = 5;
pub const VREPRINT: u32 = 6;
pub const VINTR: u32 = 8;
pub const VQUIT: u32 = 9;
pub const VSUSP: u32 = 10;
pub const VDSUSP: u32 = 11;
pub const VSTART: u32 = 12;
pub const VSTOP: u32 = 13;
pub const VLNEXT: u32 = 14;
pub const VDISCARD: u32 = 15;
pub const VMIN: u32 = 16;
pub const VTIME: u32 = 17;
pub const VSTATUS: u32 = 18;
pub const NCCS: u32 = 20;
pub const B0: u32 = 0;
pub const B50: u32 = 50;
pub const B75: u32 = 75;
pub const B110: u32 = 110;
pub const B134: u32 = 134;
pub const B150: u32 = 150;
pub const B200: u32 = 200;
pub const B300: u32 = 300;
pub const B600: u32 = 600;
pub const B1200: u32 = 1200;
pub const B1800: u32 = 1800;
pub const B2400: u32 = 2400;
pub const B4800: u32 = 4800;
pub const B9600: u32 = 9600;
pub const B7200: u32 = 7200;
pub const B14400: u32 = 14400;
pub const B19200: u32 = 19200;
pub const B28800: u32 = 28800;
pub const B38400: u32 = 38400;
pub const EXTA: u32 = 19200;
pub const EXTB: u32 = 38400;
pub const B57600: u32 = 57600;
pub const B76800: u32 = 76800;
pub const B115200: u32 = 115200;
pub const B230400: u32 = 230400;
pub const B460800: u32 = 460800;
pub const B500000: u32 = 500000;
pub const B576000: u32 = 576000;
pub const B921600: u32 = 921600;
pub const B1000000: u32 = 1000000;
pub const B1152000: u32 = 1152000;
pub const B1500000: u32 = 1500000;
pub const B2000000: u32 = 2000000;
pub const B2500000: u32 = 2500000;
pub const B3000000: u32 = 3000000;
pub const B3500000: u32 = 3500000;
pub const B4000000: u32 = 4000000;
pub const TCSANOW: u32 = 0;
pub const TCSADRAIN: u32 = 1;
pub const TCSAFLUSH: u32 = 2;
pub const TCSASOFT: u32 = 16;
pub const TCIFLUSH: u32 = 1;
pub const TCOFLUSH: u32 = 2;
pub const TCIOFLUSH: u32 = 3;
pub const TCOOFF: u32 = 1;
pub const TCOON: u32 = 2;
pub const TCIOFF: u32 = 3;
pub const TCION: u32 = 4;
pub const TTYDEF_IFLAG: u32 = 11042;
pub const TTYDEF_LFLAG: u32 = 1483;
pub const TTYDEF_CFLAG: u32 = 23040;
pub const TTYDEF_SPEED: u32 = 9600;
pub const CERASE: u32 = 127;
pub const CMIN: u32 = 1;
pub const CQUIT: u32 = 28;
pub const CTIME: u32 = 0;
pub const _ULIMIT_H: u32 = 1;
pub const _UNISTD_H: u32 = 1;
pub const _POSIX_VERSION: u32 = 200809;
pub const __POSIX2_THIS_VERSION: u32 = 200809;
pub const _POSIX2_VERSION: u32 = 200809;
pub const _POSIX2_C_VERSION: u32 = 200809;
pub const _POSIX2_C_BIND: u32 = 200809;
pub const _POSIX2_C_DEV: u32 = 200809;
pub const _POSIX2_SW_DEV: u32 = 200809;
pub const _POSIX2_LOCALEDEF: u32 = 200809;
pub const _XOPEN_VERSION: u32 = 700;
pub const _XOPEN_XCU_VERSION: u32 = 4;
pub const _XOPEN_XPG2: u32 = 1;
pub const _XOPEN_XPG3: u32 = 1;
pub const _XOPEN_XPG4: u32 = 1;
pub const _XOPEN_UNIX: u32 = 1;
pub const _XOPEN_ENH_I18N: u32 = 1;
pub const _XOPEN_LEGACY: u32 = 1;
pub const _BITS_POSIX_OPT_H: u32 = 1;
pub const _POSIX_JOB_CONTROL: u32 = 1;
pub const _POSIX_SAVED_IDS: u32 = 1;
pub const _POSIX_FSYNC: u32 = 200809;
pub const _POSIX_MAPPED_FILES: u32 = 200809;
pub const _POSIX_MEMLOCK: u32 = 200809;
pub const _POSIX_MEMLOCK_RANGE: u32 = 200809;
pub const _POSIX_MEMORY_PROTECTION: u32 = 200809;
pub const _POSIX_VDISABLE: u8 = 0u8;
pub const _POSIX_CHOWN_RESTRICTED: u32 = 0;
pub const _POSIX_NO_TRUNC: u32 = 0;
pub const _XOPEN_SHM: u32 = 1;
pub const _POSIX_THREADS: u32 = 200809;
pub const _POSIX_REENTRANT_FUNCTIONS: u32 = 1;
pub const _POSIX_THREAD_SAFE_FUNCTIONS: u32 = 200809;
pub const _POSIX_THREAD_PRIORITY_SCHEDULING: i32 = -1;
pub const _POSIX_THREAD_ATTR_STACKSIZE: u32 = 200809;
pub const _POSIX_THREAD_ATTR_STACKADDR: u32 = 200809;
pub const _POSIX_THREAD_PRIO_INHERIT: i32 = -1;
pub const _POSIX_THREAD_PRIO_PROTECT: i32 = -1;
pub const _POSIX_THREAD_ROBUST_PRIO_INHERIT: i32 = -1;
pub const _POSIX_THREAD_ROBUST_PRIO_PROTECT: i32 = -1;
pub const _POSIX_SEMAPHORES: u32 = 200809;
pub const _POSIX_REALTIME_SIGNALS: i32 = -1;
pub const _POSIX_ASYNCHRONOUS_IO: u32 = 0;
pub const _LFS_ASYNCHRONOUS_IO: u32 = 0;
pub const _LFS64_ASYNCHRONOUS_IO: u32 = 0;
pub const _LFS_LARGEFILE: u32 = 1;
pub const _LFS64_LARGEFILE: u32 = 1;
pub const _LFS64_STDIO: u32 = 1;
pub const _POSIX_SHARED_MEMORY_OBJECTS: u32 = 200809;
pub const _POSIX_CPUTIME: u32 = 0;
pub const _POSIX_THREAD_CPUTIME: u32 = 0;
pub const _POSIX_REGEXP: u32 = 1;
pub const _POSIX_READER_WRITER_LOCKS: u32 = 200809;
pub const _POSIX_SHELL: u32 = 1;
pub const _POSIX_TIMEOUTS: u32 = 200809;
pub const _POSIX_SPIN_LOCKS: u32 = 200809;
pub const _POSIX_SPAWN: u32 = 200809;
pub const _POSIX_TIMERS: u32 = 0;
pub const _POSIX_BARRIERS: u32 = 200809;
pub const _POSIX_MESSAGE_PASSING: u32 = 0;
pub const _POSIX_THREAD_PROCESS_SHARED: i32 = -1;
pub const _POSIX_MONOTONIC_CLOCK: u32 = 200809;
pub const _POSIX_CLOCK_SELECTION: i32 = -1;
pub const _POSIX_ADVISORY_INFO: u32 = 0;
pub const _POSIX_IPV6: u32 = 200809;
pub const _POSIX_RAW_SOCKETS: u32 = 200809;
pub const _POSIX2_CHAR_TERM: u32 = 200809;
pub const _POSIX_SPORADIC_SERVER: i32 = -1;
pub const _POSIX_THREAD_SPORADIC_SERVER: i32 = -1;
pub const _POSIX_TRACE: i32 = -1;
pub const _POSIX_TRACE_EVENT_FILTER: i32 = -1;
pub const _POSIX_TRACE_INHERIT: i32 = -1;
pub const _POSIX_TRACE_LOG: i32 = -1;
pub const _POSIX_TYPED_MEMORY_OBJECTS: i32 = -1;
pub const _POSIX_V7_ILP32_OFF32: u32 = 1;
pub const _POSIX_V7_ILP32_OFFBIG: u32 = 1;
pub const _POSIX_V6_ILP32_OFF32: u32 = 1;
pub const _POSIX_V6_ILP32_OFFBIG: u32 = 1;
pub const _XBS5_ILP32_OFF32: u32 = 1;
pub const _XBS5_ILP32_OFFBIG: u32 = 1;
pub const _POSIX_V7_LP64_OFF64: i32 = -1;
pub const _POSIX_V7_LPBIG_OFFBIG: i32 = -1;
pub const _POSIX_V6_LP64_OFF64: i32 = -1;
pub const _POSIX_V6_LPBIG_OFFBIG: i32 = -1;
pub const _XBS5_LP64_OFF64: i32 = -1;
pub const _XBS5_LPBIG_OFFBIG: i32 = -1;
pub const __ILP32_OFFBIG_CFLAGS: &'static [u8; 43usize] =
    b"-D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64\0";
pub const STDIN_FILENO: u32 = 0;
pub const STDOUT_FILENO: u32 = 1;
pub const STDERR_FILENO: u32 = 2;
pub const L_SET: u32 = 0;
pub const L_INCR: u32 = 1;
pub const L_XTND: u32 = 2;
pub const _GETOPT_POSIX_H: u32 = 1;
pub const _GETOPT_CORE_H: u32 = 1;
pub const _UTIME_H: u32 = 1;
pub const _UTMPX_H: u32 = 1;
pub const __UT_LINESIZE: u32 = 32;
pub const __UT_NAMESIZE: u32 = 32;
pub const __UT_HOSTSIZE: u32 = 256;
pub const EMPTY: u32 = 0;
pub const BOOT_TIME: u32 = 2;
pub const NEW_TIME: u32 = 3;
pub const OLD_TIME: u32 = 4;
pub const INIT_PROCESS: u32 = 5;
pub const LOGIN_PROCESS: u32 = 6;
pub const USER_PROCESS: u32 = 7;
pub const DEAD_PROCESS: u32 = 8;
pub const _WCHAR_H: u32 = 1;
pub const __wint_t_defined: u32 = 1;
pub const _WINT_T: u32 = 1;
pub const __mbstate_t_defined: u32 = 1;
pub const WEOF: u32 = 4294967295;
pub const _WCTYPE_H: u32 = 1;
pub const _BITS_WCTYPE_WCHAR_H: u32 = 1;
pub const _WORDEXP_H: u32 = 1;

pub const MSG_OOB: ::c_uint = 1;
pub const MSG_PEEK: ::c_uint = 2;
pub const MSG_DONTROUTE: ::c_uint = 4;
pub const MSG_EOR: ::c_uint = 8;
pub const MSG_TRUNC: ::c_uint = 16;
pub const MSG_CTRUNC: ::c_uint = 32;
pub const MSG_WAITALL: ::c_uint = 64;
pub const MSG_DONTWAIT: ::c_uint = 128;
pub const MSG_NOSIGNAL: ::c_uint = 1024;

pub const IPPROTO_IP: ::c_uint = 0;
pub const IPPROTO_ICMP: ::c_uint = 1;
pub const IPPROTO_IGMP: ::c_uint = 2;
pub const IPPROTO_IPIP: ::c_uint = 4;
pub const IPPROTO_TCP: ::c_uint = 6;
pub const IPPROTO_EGP: ::c_uint = 8;
pub const IPPROTO_PUP: ::c_uint = 12;
pub const IPPROTO_UDP: ::c_uint = 17;
pub const IPPROTO_IDP: ::c_uint = 22;
pub const IPPROTO_TP: ::c_uint = 29;
pub const IPPROTO_DCCP: ::c_uint = 33;
pub const IPPROTO_IPV6: ::c_uint = 41;
pub const IPPROTO_RSVP: ::c_uint = 46;
pub const IPPROTO_GRE: ::c_uint = 47;
pub const IPPROTO_ESP: ::c_uint = 50;
pub const IPPROTO_AH: ::c_uint = 51;
pub const IPPROTO_MTP: ::c_uint = 92;
pub const IPPROTO_BEETPH: ::c_uint = 94;
pub const IPPROTO_ENCAP: ::c_uint = 98;
pub const IPPROTO_PIM: ::c_uint = 103;
pub const IPPROTO_COMP: ::c_uint = 108;
pub const IPPROTO_SCTP: ::c_uint = 132;
pub const IPPROTO_UDPLITE: ::c_uint = 136;
pub const IPPROTO_MPLS: ::c_uint = 137;
pub const IPPROTO_RAW: ::c_uint = 255;
pub const IPPROTO_MAX: ::c_uint = 256;
pub type _bindgen_ty_19 = ::c_uint;
pub const IPPROTO_HOPOPTS: ::c_uint = 0;
pub const IPPROTO_ROUTING: ::c_uint = 43;
pub const IPPROTO_FRAGMENT: ::c_uint = 44;
pub const IPPROTO_ICMPV6: ::c_uint = 58;
pub const IPPROTO_NONE: ::c_uint = 59;
pub const IPPROTO_DSTOPTS: ::c_uint = 60;
pub const IPPROTO_MH: ::c_uint = 135;
pub type _bindgen_ty_20 = ::c_uint;
pub type in_port_t = u16;
pub const IPPORT_ECHO: ::c_uint = 7;
pub const IPPORT_DISCARD: ::c_uint = 9;
pub const IPPORT_SYSTAT: ::c_uint = 11;
pub const IPPORT_DAYTIME: ::c_uint = 13;
pub const IPPORT_NETSTAT: ::c_uint = 15;
pub const IPPORT_FTP: ::c_uint = 21;
pub const IPPORT_TELNET: ::c_uint = 23;
pub const IPPORT_SMTP: ::c_uint = 25;
pub const IPPORT_TIMESERVER: ::c_uint = 37;
pub const IPPORT_NAMESERVER: ::c_uint = 42;
pub const IPPORT_WHOIS: ::c_uint = 43;
pub const IPPORT_MTP: ::c_uint = 57;
pub const IPPORT_TFTP: ::c_uint = 69;
pub const IPPORT_RJE: ::c_uint = 77;
pub const IPPORT_FINGER: ::c_uint = 79;
pub const IPPORT_TTYLINK: ::c_uint = 87;
pub const IPPORT_SUPDUP: ::c_uint = 95;
pub const IPPORT_EXECSERVER: ::c_uint = 512;
pub const IPPORT_LOGINSERVER: ::c_uint = 513;
pub const IPPORT_CMDSERVER: ::c_uint = 514;
pub const IPPORT_EFSSERVER: ::c_uint = 520;
pub const IPPORT_BIFFUDP: ::c_uint = 512;
pub const IPPORT_WHOSERVER: ::c_uint = 513;
pub const IPPORT_ROUTESERVER: ::c_uint = 520;
pub const IPPORT_RESERVED: ::c_uint = 1024;
pub const IPPORT_USERRESERVED: ::c_uint = 5000;
pub type _bindgen_ty_21 = ::c_uint;
pub const RTLD_DEFAULT: *mut ::c_void = 0i64 as *mut ::c_void;
pub const SO_DEBUG: ::c_uint = 1;
pub const SO_ACCEPTCONN: ::c_uint = 2;
pub const SO_REUSEADDR: ::c_uint = 4;
pub const SO_KEEPALIVE: ::c_uint = 8;
pub const SO_DONTROUTE: ::c_uint = 16;
pub const SO_BROADCAST: ::c_uint = 32;
pub const SO_USELOOPBACK: ::c_uint = 64;
pub const SO_LINGER: ::c_uint = 128;
pub const SO_OOBINLINE: ::c_uint = 256;
pub const SO_REUSEPORT: ::c_uint = 512;
pub const SO_SNDBUF: ::c_uint = 4097;
pub const SO_RCVBUF: ::c_uint = 4098;
pub const SO_SNDLOWAT: ::c_uint = 4099;
pub const SO_RCVLOWAT: ::c_uint = 4100;
pub const SO_SNDTIMEO: ::c_uint = 4101;
pub const SO_RCVTIMEO: ::c_uint = 4102;
pub const SO_ERROR: ::c_uint = 4103;
pub const SO_STYLE: ::c_uint = 4104;
pub const SO_TYPE: ::c_uint = 4104;
pub const SHUT_RD: ::c_uint = 0;
pub const SHUT_WR: ::c_uint = 1;
pub const SHUT_RDWR: ::c_uint = 2;
pub const TCP_NODELAY: ::c_int = 1;
pub const _PC_LINK_MAX: ::c_uint = 0;
pub const _PC_MAX_CANON: ::c_uint = 1;
pub const _PC_MAX_INPUT: ::c_uint = 2;
pub const _PC_NAME_MAX: ::c_uint = 3;
pub const _PC_PATH_MAX: ::c_uint = 4;
pub const _PC_PIPE_BUF: ::c_uint = 5;
pub const _PC_CHOWN_RESTRICTED: ::c_uint = 6;
pub const _PC_NO_TRUNC: ::c_uint = 7;
pub const _PC_VDISABLE: ::c_uint = 8;
pub const _PC_SYNC_IO: ::c_uint = 9;
pub const _PC_ASYNC_IO: ::c_uint = 10;
pub const _PC_PRIO_IO: ::c_uint = 11;
pub const _PC_SOCK_MAXBUF: ::c_uint = 12;
pub const _PC_FILESIZEBITS: ::c_uint = 13;
pub const _PC_REC_INCR_XFER_SIZE: ::c_uint = 14;
pub const _PC_REC_MAX_XFER_SIZE: ::c_uint = 15;
pub const _PC_REC_MIN_XFER_SIZE: ::c_uint = 16;
pub const _PC_REC_XFER_ALIGN: ::c_uint = 17;
pub const _PC_ALLOC_SIZE_MIN: ::c_uint = 18;
pub const _PC_SYMLINK_MAX: ::c_uint = 19;
pub const _PC_2_SYMLINKS: ::c_uint = 20;
pub const _SC_ARG_MAX: ::c_uint = 0;
pub const _SC_CHILD_MAX: ::c_uint = 1;
pub const _SC_CLK_TCK: ::c_uint = 2;
pub const _SC_NGROUPS_MAX: ::c_uint = 3;
pub const _SC_OPEN_MAX: ::c_uint = 4;
pub const _SC_STREAM_MAX: ::c_uint = 5;
pub const _SC_TZNAME_MAX: ::c_uint = 6;
pub const _SC_JOB_CONTROL: ::c_uint = 7;
pub const _SC_SAVED_IDS: ::c_uint = 8;
pub const _SC_REALTIME_SIGNALS: ::c_uint = 9;
pub const _SC_PRIORITY_SCHEDULING: ::c_uint = 10;
pub const _SC_TIMERS: ::c_uint = 11;
pub const _SC_ASYNCHRONOUS_IO: ::c_uint = 12;
pub const _SC_PRIORITIZED_IO: ::c_uint = 13;
pub const _SC_SYNCHRONIZED_IO: ::c_uint = 14;
pub const _SC_FSYNC: ::c_uint = 15;
pub const _SC_MAPPED_FILES: ::c_uint = 16;
pub const _SC_MEMLOCK: ::c_uint = 17;
pub const _SC_MEMLOCK_RANGE: ::c_uint = 18;
pub const _SC_MEMORY_PROTECTION: ::c_uint = 19;
pub const _SC_MESSAGE_PASSING: ::c_uint = 20;
pub const _SC_SEMAPHORES: ::c_uint = 21;
pub const _SC_SHARED_MEMORY_OBJECTS: ::c_uint = 22;
pub const _SC_AIO_LISTIO_MAX: ::c_uint = 23;
pub const _SC_AIO_MAX: ::c_uint = 24;
pub const _SC_AIO_PRIO_DELTA_MAX: ::c_uint = 25;
pub const _SC_DELAYTIMER_MAX: ::c_uint = 26;
pub const _SC_MQ_OPEN_MAX: ::c_uint = 27;
pub const _SC_MQ_PRIO_MAX: ::c_uint = 28;
pub const _SC_VERSION: ::c_uint = 29;
pub const _SC_PAGESIZE: ::c_uint = 30;
pub const _SC_RTSIG_MAX: ::c_uint = 31;
pub const _SC_SEM_NSEMS_MAX: ::c_uint = 32;
pub const _SC_SEM_VALUE_MAX: ::c_uint = 33;
pub const _SC_SIGQUEUE_MAX: ::c_uint = 34;
pub const _SC_TIMER_MAX: ::c_uint = 35;
pub const _SC_BC_BASE_MAX: ::c_uint = 36;
pub const _SC_BC_DIM_MAX: ::c_uint = 37;
pub const _SC_BC_SCALE_MAX: ::c_uint = 38;
pub const _SC_BC_STRING_MAX: ::c_uint = 39;
pub const _SC_COLL_WEIGHTS_MAX: ::c_uint = 40;
pub const _SC_EQUIV_CLASS_MAX: ::c_uint = 41;
pub const _SC_EXPR_NEST_MAX: ::c_uint = 42;
pub const _SC_LINE_MAX: ::c_uint = 43;
pub const _SC_RE_DUP_MAX: ::c_uint = 44;
pub const _SC_CHARCLASS_NAME_MAX: ::c_uint = 45;
pub const _SC_2_VERSION: ::c_uint = 46;
pub const _SC_2_C_BIND: ::c_uint = 47;
pub const _SC_2_C_DEV: ::c_uint = 48;
pub const _SC_2_FORT_DEV: ::c_uint = 49;
pub const _SC_2_FORT_RUN: ::c_uint = 50;
pub const _SC_2_SW_DEV: ::c_uint = 51;
pub const _SC_2_LOCALEDEF: ::c_uint = 52;
pub const _SC_PII: ::c_uint = 53;
pub const _SC_PII_XTI: ::c_uint = 54;
pub const _SC_PII_SOCKET: ::c_uint = 55;
pub const _SC_PII_INTERNET: ::c_uint = 56;
pub const _SC_PII_OSI: ::c_uint = 57;
pub const _SC_POLL: ::c_uint = 58;
pub const _SC_SELECT: ::c_uint = 59;
pub const _SC_UIO_MAXIOV: ::c_uint = 60;
pub const _SC_IOV_MAX: ::c_uint = 60;
pub const _SC_PII_INTERNET_STREAM: ::c_uint = 61;
pub const _SC_PII_INTERNET_DGRAM: ::c_uint = 62;
pub const _SC_PII_OSI_COTS: ::c_uint = 63;
pub const _SC_PII_OSI_CLTS: ::c_uint = 64;
pub const _SC_PII_OSI_M: ::c_uint = 65;
pub const _SC_T_IOV_MAX: ::c_uint = 66;
pub const _SC_THREADS: ::c_uint = 67;
pub const _SC_THREAD_SAFE_FUNCTIONS: ::c_uint = 68;
pub const _SC_GETGR_R_SIZE_MAX: ::c_uint = 69;
pub const _SC_GETPW_R_SIZE_MAX: ::c_uint = 70;
pub const _SC_LOGIN_NAME_MAX: ::c_uint = 71;
pub const _SC_TTY_NAME_MAX: ::c_uint = 72;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: ::c_uint = 73;
pub const _SC_THREAD_KEYS_MAX: ::c_uint = 74;
pub const _SC_THREAD_STACK_MIN: ::c_uint = 75;
pub const _SC_THREAD_THREADS_MAX: ::c_uint = 76;
pub const _SC_THREAD_ATTR_STACKADDR: ::c_uint = 77;
pub const _SC_THREAD_ATTR_STACKSIZE: ::c_uint = 78;
pub const _SC_THREAD_PRIORITY_SCHEDULING: ::c_uint = 79;
pub const _SC_THREAD_PRIO_INHERIT: ::c_uint = 80;
pub const _SC_THREAD_PRIO_PROTECT: ::c_uint = 81;
pub const _SC_THREAD_PROCESS_SHARED: ::c_uint = 82;
pub const _SC_NPROCESSORS_CONF: ::c_uint = 83;
pub const _SC_NPROCESSORS_ONLN: ::c_uint = 84;
pub const _SC_PHYS_PAGES: ::c_uint = 85;
pub const _SC_AVPHYS_PAGES: ::c_uint = 86;
pub const _SC_ATEXIT_MAX: ::c_uint = 87;
pub const _SC_PASS_MAX: ::c_uint = 88;
pub const _SC_XOPEN_VERSION: ::c_uint = 89;
pub const _SC_XOPEN_XCU_VERSION: ::c_uint = 90;
pub const _SC_XOPEN_UNIX: ::c_uint = 91;
pub const _SC_XOPEN_CRYPT: ::c_uint = 92;
pub const _SC_XOPEN_ENH_I18N: ::c_uint = 93;
pub const _SC_XOPEN_SHM: ::c_uint = 94;
pub const _SC_2_CHAR_TERM: ::c_uint = 95;
pub const _SC_2_C_VERSION: ::c_uint = 96;
pub const _SC_2_UPE: ::c_uint = 97;
pub const _SC_XOPEN_XPG2: ::c_uint = 98;
pub const _SC_XOPEN_XPG3: ::c_uint = 99;
pub const _SC_XOPEN_XPG4: ::c_uint = 100;
pub const _SC_CHAR_BIT: ::c_uint = 101;
pub const _SC_CHAR_MAX: ::c_uint = 102;
pub const _SC_CHAR_MIN: ::c_uint = 103;
pub const _SC_INT_MAX: ::c_uint = 104;
pub const _SC_INT_MIN: ::c_uint = 105;
pub const _SC_LONG_BIT: ::c_uint = 106;
pub const _SC_WORD_BIT: ::c_uint = 107;
pub const _SC_MB_LEN_MAX: ::c_uint = 108;
pub const _SC_NZERO: ::c_uint = 109;
pub const _SC_SSIZE_MAX: ::c_uint = 110;
pub const _SC_SCHAR_MAX: ::c_uint = 111;
pub const _SC_SCHAR_MIN: ::c_uint = 112;
pub const _SC_SHRT_MAX: ::c_uint = 113;
pub const _SC_SHRT_MIN: ::c_uint = 114;
pub const _SC_UCHAR_MAX: ::c_uint = 115;
pub const _SC_UINT_MAX: ::c_uint = 116;
pub const _SC_ULONG_MAX: ::c_uint = 117;
pub const _SC_USHRT_MAX: ::c_uint = 118;
pub const _SC_NL_ARGMAX: ::c_uint = 119;
pub const _SC_NL_LANGMAX: ::c_uint = 120;
pub const _SC_NL_MSGMAX: ::c_uint = 121;
pub const _SC_NL_NMAX: ::c_uint = 122;
pub const _SC_NL_SETMAX: ::c_uint = 123;
pub const _SC_NL_TEXTMAX: ::c_uint = 124;
pub const _SC_XBS5_ILP32_OFF32: ::c_uint = 125;
pub const _SC_XBS5_ILP32_OFFBIG: ::c_uint = 126;
pub const _SC_XBS5_LP64_OFF64: ::c_uint = 127;
pub const _SC_XBS5_LPBIG_OFFBIG: ::c_uint = 128;
pub const _SC_XOPEN_LEGACY: ::c_uint = 129;
pub const _SC_XOPEN_REALTIME: ::c_uint = 130;
pub const _SC_XOPEN_REALTIME_THREADS: ::c_uint = 131;
pub const _SC_ADVISORY_INFO: ::c_uint = 132;
pub const _SC_BARRIERS: ::c_uint = 133;
pub const _SC_BASE: ::c_uint = 134;
pub const _SC_C_LANG_SUPPORT: ::c_uint = 135;
pub const _SC_C_LANG_SUPPORT_R: ::c_uint = 136;
pub const _SC_CLOCK_SELECTION: ::c_uint = 137;
pub const _SC_CPUTIME: ::c_uint = 138;
pub const _SC_THREAD_CPUTIME: ::c_uint = 139;
pub const _SC_DEVICE_IO: ::c_uint = 140;
pub const _SC_DEVICE_SPECIFIC: ::c_uint = 141;
pub const _SC_DEVICE_SPECIFIC_R: ::c_uint = 142;
pub const _SC_FD_MGMT: ::c_uint = 143;
pub const _SC_FIFO: ::c_uint = 144;
pub const _SC_PIPE: ::c_uint = 145;
pub const _SC_FILE_ATTRIBUTES: ::c_uint = 146;
pub const _SC_FILE_LOCKING: ::c_uint = 147;
pub const _SC_FILE_SYSTEM: ::c_uint = 148;
pub const _SC_MONOTONIC_CLOCK: ::c_uint = 149;
pub const _SC_MULTI_PROCESS: ::c_uint = 150;
pub const _SC_SINGLE_PROCESS: ::c_uint = 151;
pub const _SC_NETWORKING: ::c_uint = 152;
pub const _SC_READER_WRITER_LOCKS: ::c_uint = 153;
pub const _SC_SPIN_LOCKS: ::c_uint = 154;
pub const _SC_REGEXP: ::c_uint = 155;
pub const _SC_REGEX_VERSION: ::c_uint = 156;
pub const _SC_SHELL: ::c_uint = 157;
pub const _SC_SIGNALS: ::c_uint = 158;
pub const _SC_SPAWN: ::c_uint = 159;
pub const _SC_SPORADIC_SERVER: ::c_uint = 160;
pub const _SC_THREAD_SPORADIC_SERVER: ::c_uint = 161;
pub const _SC_SYSTEM_DATABASE: ::c_uint = 162;
pub const _SC_SYSTEM_DATABASE_R: ::c_uint = 163;
pub const _SC_TIMEOUTS: ::c_uint = 164;
pub const _SC_TYPED_MEMORY_OBJECTS: ::c_uint = 165;
pub const _SC_USER_GROUPS: ::c_uint = 166;
pub const _SC_USER_GROUPS_R: ::c_uint = 167;
pub const _SC_2_PBS: ::c_uint = 168;
pub const _SC_2_PBS_ACCOUNTING: ::c_uint = 169;
pub const _SC_2_PBS_LOCATE: ::c_uint = 170;
pub const _SC_2_PBS_MESSAGE: ::c_uint = 171;
pub const _SC_2_PBS_TRACK: ::c_uint = 172;
pub const _SC_SYMLOOP_MAX: ::c_uint = 173;
pub const _SC_STREAMS: ::c_uint = 174;
pub const _SC_2_PBS_CHECKPOINT: ::c_uint = 175;
pub const _SC_V6_ILP32_OFF32: ::c_uint = 176;
pub const _SC_V6_ILP32_OFFBIG: ::c_uint = 177;
pub const _SC_V6_LP64_OFF64: ::c_uint = 178;
pub const _SC_V6_LPBIG_OFFBIG: ::c_uint = 179;
pub const _SC_HOST_NAME_MAX: ::c_uint = 180;
pub const _SC_TRACE: ::c_uint = 181;
pub const _SC_TRACE_EVENT_FILTER: ::c_uint = 182;
pub const _SC_TRACE_INHERIT: ::c_uint = 183;
pub const _SC_TRACE_LOG: ::c_uint = 184;
pub const _SC_LEVEL1_ICACHE_SIZE: ::c_uint = 185;
pub const _SC_LEVEL1_ICACHE_ASSOC: ::c_uint = 186;
pub const _SC_LEVEL1_ICACHE_LINESIZE: ::c_uint = 187;
pub const _SC_LEVEL1_DCACHE_SIZE: ::c_uint = 188;
pub const _SC_LEVEL1_DCACHE_ASSOC: ::c_uint = 189;
pub const _SC_LEVEL1_DCACHE_LINESIZE: ::c_uint = 190;
pub const _SC_LEVEL2_CACHE_SIZE: ::c_uint = 191;
pub const _SC_LEVEL2_CACHE_ASSOC: ::c_uint = 192;
pub const _SC_LEVEL2_CACHE_LINESIZE: ::c_uint = 193;
pub const _SC_LEVEL3_CACHE_SIZE: ::c_uint = 194;
pub const _SC_LEVEL3_CACHE_ASSOC: ::c_uint = 195;
pub const _SC_LEVEL3_CACHE_LINESIZE: ::c_uint = 196;
pub const _SC_LEVEL4_CACHE_SIZE: ::c_uint = 197;
pub const _SC_LEVEL4_CACHE_ASSOC: ::c_uint = 198;
pub const _SC_LEVEL4_CACHE_LINESIZE: ::c_uint = 199;
pub const _SC_IPV6: ::c_uint = 235;
pub const _SC_RAW_SOCKETS: ::c_uint = 236;
pub const _SC_V7_ILP32_OFF32: ::c_uint = 237;
pub const _SC_V7_ILP32_OFFBIG: ::c_uint = 238;
pub const _SC_V7_LP64_OFF64: ::c_uint = 239;
pub const _SC_V7_LPBIG_OFFBIG: ::c_uint = 240;
pub const _SC_SS_REPL_MAX: ::c_uint = 241;
pub const _SC_TRACE_EVENT_NAME_MAX: ::c_uint = 242;
pub const _SC_TRACE_NAME_MAX: ::c_uint = 243;
pub const _SC_TRACE_SYS_MAX: ::c_uint = 244;
pub const _SC_TRACE_USER_EVENT_MAX: ::c_uint = 245;
pub const _SC_XOPEN_STREAMS: ::c_uint = 246;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: ::c_uint = 247;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: ::c_uint = 248;
pub const _CS_PATH: ::c_uint = 0;
pub const _CS_V6_WIDTH_RESTRICTED_ENVS: ::c_uint = 1;
pub const _CS_GNU_LIBC_VERSION: ::c_uint = 2;
pub const _CS_GNU_LIBPTHREAD_VERSION: ::c_uint = 3;
pub const _CS_V5_WIDTH_RESTRICTED_ENVS: ::c_uint = 4;
pub const _CS_V7_WIDTH_RESTRICTED_ENVS: ::c_uint = 5;
pub const _CS_LFS_CFLAGS: ::c_uint = 1000;
pub const _CS_LFS_LDFLAGS: ::c_uint = 1001;
pub const _CS_LFS_LIBS: ::c_uint = 1002;
pub const _CS_LFS_LINTFLAGS: ::c_uint = 1003;
pub const _CS_LFS64_CFLAGS: ::c_uint = 1004;
pub const _CS_LFS64_LDFLAGS: ::c_uint = 1005;
pub const _CS_LFS64_LIBS: ::c_uint = 1006;
pub const _CS_LFS64_LINTFLAGS: ::c_uint = 1007;
pub const _CS_XBS5_ILP32_OFF32_CFLAGS: ::c_uint = 1100;
pub const _CS_XBS5_ILP32_OFF32_LDFLAGS: ::c_uint = 1101;
pub const _CS_XBS5_ILP32_OFF32_LIBS: ::c_uint = 1102;
pub const _CS_XBS5_ILP32_OFF32_LINTFLAGS: ::c_uint = 1103;
pub const _CS_XBS5_ILP32_OFFBIG_CFLAGS: ::c_uint = 1104;
pub const _CS_XBS5_ILP32_OFFBIG_LDFLAGS: ::c_uint = 1105;
pub const _CS_XBS5_ILP32_OFFBIG_LIBS: ::c_uint = 1106;
pub const _CS_XBS5_ILP32_OFFBIG_LINTFLAGS: ::c_uint = 1107;
pub const _CS_XBS5_LP64_OFF64_CFLAGS: ::c_uint = 1108;
pub const _CS_XBS5_LP64_OFF64_LDFLAGS: ::c_uint = 1109;
pub const _CS_XBS5_LP64_OFF64_LIBS: ::c_uint = 1110;
pub const _CS_XBS5_LP64_OFF64_LINTFLAGS: ::c_uint = 1111;
pub const _CS_XBS5_LPBIG_OFFBIG_CFLAGS: ::c_uint = 1112;
pub const _CS_XBS5_LPBIG_OFFBIG_LDFLAGS: ::c_uint = 1113;
pub const _CS_XBS5_LPBIG_OFFBIG_LIBS: ::c_uint = 1114;
pub const _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS: ::c_uint = 1115;
pub const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: ::c_uint = 1116;
pub const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: ::c_uint = 1117;
pub const _CS_POSIX_V6_ILP32_OFF32_LIBS: ::c_uint = 1118;
pub const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: ::c_uint = 1119;
pub const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: ::c_uint = 1120;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: ::c_uint = 1121;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: ::c_uint = 1122;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: ::c_uint = 1123;
pub const _CS_POSIX_V6_LP64_OFF64_CFLAGS: ::c_uint = 1124;
pub const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: ::c_uint = 1125;
pub const _CS_POSIX_V6_LP64_OFF64_LIBS: ::c_uint = 1126;
pub const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: ::c_uint = 1127;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: ::c_uint = 1128;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: ::c_uint = 1129;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: ::c_uint = 1130;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: ::c_uint = 1131;
pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: ::c_uint = 1132;
pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: ::c_uint = 1133;
pub const _CS_POSIX_V7_ILP32_OFF32_LIBS: ::c_uint = 1134;
pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: ::c_uint = 1135;
pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: ::c_uint = 1136;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: ::c_uint = 1137;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: ::c_uint = 1138;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: ::c_uint = 1139;
pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS: ::c_uint = 1140;
pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: ::c_uint = 1141;
pub const _CS_POSIX_V7_LP64_OFF64_LIBS: ::c_uint = 1142;
pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: ::c_uint = 1143;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: ::c_uint = 1144;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: ::c_uint = 1145;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: ::c_uint = 1146;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: ::c_uint = 1147;
pub const _CS_V6_ENV: ::c_uint = 1148;
pub const _CS_V7_ENV: ::c_uint = 1149;

pub type __u_char = ::c_uchar;
pub type __u_short = ::c_ushort;
pub type __u_int = ::c_uint;
pub type __u_long = ::c_ulong;
pub type __int8_t = ::c_schar;
pub type __uint8_t = ::c_uchar;
pub type __int16_t = ::c_short;
pub type __uint16_t = ::c_ushort;
pub type __int32_t = ::c_int;
pub type __uint32_t = ::c_uint;
pub type __int64_t = ::c_longlong;
pub type __uint64_t = ::c_ulonglong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::c_longlong;
pub type __u_quad_t = ::c_ulonglong;
pub type __intmax_t = ::c_longlong;
pub type __uintmax_t = ::c_ulonglong;
pub type __dev_t = ::c_uint;
pub type __uid_t = ::c_uint;
pub type __gid_t = ::c_uint;
pub type __ino_t = ::c_ulong;
pub type __ino64_t = __uint64_t;
pub type __mode_t = ::c_uint;
pub type __nlink_t = ::c_uint;
pub type __off_t = ::c_long;
pub type __off64_t = __int64_t;
pub type __pid_t = ::c_int;
pub type __fsid_t = __uint64_t;
pub type __clock_t = ::c_long;
pub type __rlim_t = ::c_ulong;
pub type __rlim64_t = __uint64_t;
pub type __id_t = ::c_uint;
pub type __time_t = ::c_long;
pub type __useconds_t = ::c_uint;
pub type __suseconds_t = ::c_long;
pub type __daddr_t = ::c_int;
pub type __key_t = ::c_int;
pub type __clockid_t = ::c_int;
pub type __timer_t = ::c_int;
pub type __blksize_t = ::c_long;
pub type __blkcnt_t = ::c_long;
pub type __blkcnt64_t = __int64_t;
pub type __fsblkcnt_t = ::c_ulong;
pub type __fsblkcnt64_t = __uint64_t;
pub type __fsfilcnt_t = ::c_ulong;
pub type __fsfilcnt64_t = __uint64_t;
pub type __fsword_t = ::c_int;
pub type __ssize_t = ::c_int;
pub type __syscall_slong_t = ::c_long;
pub type __syscall_ulong_t = ::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::c_char;
pub type __intptr_t = ::c_int;
pub type __socklen_t = ::c_uint;
pub type __sig_atomic_t = ::c_int;
pub type __time64_t = __int64_t;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
//pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
//pub type uid_t = __uid_t;
pub type off_t = __off_t;
//pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type ssize_t = __ssize_t;
pub type suseconds_t = __suseconds_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type size_t = ::c_uint;
pub type ulong = ::c_ulong;
pub type ushort = ::c_ushort;
pub type uint = ::c_uint;
pub type u_int8_t = __uint8_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
pub type register_t = ::c_int;
pub type __sigset_t = ::c_ulong;
pub type sigset_t = __sigset_t;
pub type rlim_t = __rlim_t;

pub type socklen_t = __socklen_t;
pub type __socket_type = ::c_uint;
pub const SOCK_STREAM: __socket_type = 1;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_CLOEXEC: __socket_type = 4194304;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub type sa_family_t = ::c_uchar;
pub type __pthread_t = ::c_int;
pub type pthread_t = __pthread_t;
pub const PTHREAD_PROCESS_PRIVATE : __pthread_process_shared = 0 ;
pub const PTHREAD_PROCESS_SHARED : __pthread_process_shared = 1 ;
pub type __pthread_process_shared = ::c_uint;
pub const PTHREAD_EXPLICIT_SCHED : __pthread_inheritsched = 0 ;   
pub const PTHREAD_INHERIT_SCHED : __pthread_inheritsched = 1 ;
pub type __pthread_inheritsched = ::c_uint;
pub const PTHREAD_SCOPE_SYSTEM : __pthread_contentionscope = 0 ;
pub const PTHREAD_SCOPE_PROCESS : __pthread_contentionscope = 1 ;
pub type __pthread_contentionscope = ::c_uint;
pub const PTHREAD_CREATE_JOINABLE : __pthread_detachstate = 0 ;
pub const PTHREAD_CREATE_DETACHED : __pthread_detachstate = 1 ;
pub type __pthread_detachstate = ::c_uint;
pub type pthread_attr_t = __pthread_attr;
pub const PTHREAD_PRIO_NONE : __pthread_mutex_protocol = 0 ;
pub const PTHREAD_PRIO_INHERIT : __pthread_mutex_protocol = 1 ;
pub const PTHREAD_PRIO_PROTECT : __pthread_mutex_protocol = 2 ;
pub type __pthread_mutex_protocol = ::c_uint;
pub const PTHREAD_MUTEX_TIMED: __pthread_mutex_type = 0;
pub const PTHREAD_MUTEX_ERRORCHECK : __pthread_mutex_type = 1 ;
pub const PTHREAD_MUTEX_RECURSIVE : __pthread_mutex_type = 2 ;
pub type __pthread_mutex_type = ::c_uint;
pub const PTHREAD_MUTEX_STALLED : __pthread_mutex_robustness = 0 ;
pub const PTHREAD_MUTEX_ROBUST : __pthread_mutex_robustness = 256 ;
pub type __pthread_mutex_robustness = ::c_uint;
extern "C" {
    pub static __pthread_errorcheck_mutexattr: __pthread_mutexattr;
}
extern "C" {
    pub static __pthread_recursive_mutexattr: __pthread_mutexattr;
}
pub type pthread_mutexattr_t = __pthread_mutexattr;
pub type pthread_mutex_t = __pthread_mutex;
pub type pthread_condattr_t = __pthread_condattr;
pub type __pthread_spinlock_t = ::c_int;
pub type pthread_cond_t = __pthread_cond;
pub type pthread_spinlock_t = __pthread_spinlock_t;
pub type pthread_rwlockattr_t = __pthread_rwlockattr;
pub type pthread_rwlock_t = __pthread_rwlock;
pub type pthread_barrierattr_t = __pthread_barrierattr;
pub type pthread_barrier_t = __pthread_barrier;
pub type __pthread_key = ::c_int;
pub type pthread_key_t = __pthread_key;
pub type pthread_once_t = __pthread_once;
pub type __sigval_t = sigval;
pub type __sighandler_t = unsafe extern "C" fn(arg1: ::c_int);
pub type sigevent_t = sigevent;
pub const SIGEV_SIGNAL: ::c_uint = 0;
pub const SIGEV_NONE: ::c_uint = 1;
pub const SIGEV_THREAD: ::c_uint = 2;
pub type tcflag_t = ::c_ulong;
pub type cc_t = ::c_uchar;
pub type speed_t = ::c_long;
pub type __fd_mask = ::c_long;
pub type nfds_t = ::c_ulong;
pub type in_addr_t = u32;

/*
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
*/

s_no_extra_traits! {

    pub union sem_t {
        pub __size: [::c_char; 20usize],
        pub __align: ::c_long,
    }
   
    pub union sigval { pub sival_int : :: c_int , pub sival_ptr : * mut :: c_void , }

    pub union sigaction__bindgen_ty_1 { pub sa_handler : __sighandler_t , pub sa_sigaction : unsafe extern "C" fn (arg1 : :: c_int , arg2 : * mut siginfo_t , arg3 : * mut :: c_void) , }

}

s! {

    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }

    pub struct passwd {
        pub pw_name: *mut ::c_char,
        pub pw_passwd: *mut ::c_char,
        pub pw_uid: __uid_t,
        pub pw_gid: __gid_t,
        pub pw_gecos: *mut ::c_char,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char,
    }
   
    pub struct sockaddr {
        pub sa_len: ::c_uchar,
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14usize],
    }

    pub struct sockaddr_storage {
        pub ss_len: ::c_uchar,
        pub ss_family: sa_family_t,
        pub __ss_padding: [::c_char; 122usize],
        pub __ss_align: __uint32_t,
    }

    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
   
    pub struct ip_opts {
        pub ip_dst: in_addr,
        pub ip_opts: [::c_char; 40usize],
    }

    pub struct stat {
        pub st_fstype: ::c_int,
        pub st_fsid: __fsid_t,
        pub st_ino: __ino_t,
        pub st_gen: ::c_uint,
        pub st_rdev: __dev_t,
        pub st_mode: __mode_t,
        pub st_nlink: __nlink_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub st_size: __off_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_author: __uid_t,
        pub st_flags: ::c_uint,
        pub st_spare: [::c_int; 11usize],
    }

    pub struct dirent {
        pub d_ino: __ino_t,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_namlen: ::c_uchar,
        pub d_name: [::c_char; 1usize],
    }
   
    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *const ::c_void,
    }
   
    pub struct lconv {
        pub decimal_point: *mut ::c_char,
        pub thousands_sep: *mut ::c_char,
        pub grouping: *mut ::c_char,
        pub int_curr_symbol: *mut ::c_char,
        pub currency_symbol: *mut ::c_char,
        pub mon_decimal_point: *mut ::c_char,
        pub mon_thousands_sep: *mut ::c_char,
        pub mon_grouping: *mut ::c_char,
        pub positive_sign: *mut ::c_char,
        pub negative_sign: *mut ::c_char,
        pub int_frac_digits: ::c_char,
        pub frac_digits: ::c_char,
        pub p_cs_precedes: ::c_char,
        pub p_sep_by_space: ::c_char,
        pub n_cs_precedes: ::c_char,
        pub n_sep_by_space: ::c_char,
        pub p_sign_posn: ::c_char,
        pub n_sign_posn: ::c_char,
        pub int_p_cs_precedes: ::c_char,
        pub int_p_sep_by_space: ::c_char,
        pub int_n_cs_precedes: ::c_char,
        pub int_n_sep_by_space: ::c_char,
        pub int_p_sign_posn: ::c_char,
        pub int_n_sign_posn: ::c_char,
    }
   
    pub struct tm {
        pub tm_sec: ::c_int,
        pub tm_min: ::c_int,
        pub tm_hour: ::c_int,
        pub tm_mday: ::c_int,
        pub tm_mon: ::c_int,
        pub tm_year: ::c_int,
        pub tm_wday: ::c_int,
        pub tm_yday: ::c_int,
        pub tm_isdst: ::c_int,
        pub tm_gmtoff: ::c_long,
        pub tm_zone: *const ::c_char,
    }   
   
    pub struct __pthread {
        pub _address: u8,
    }
     
    pub struct sched_param {
        pub sched_priority: ::c_int,
    }
   
    pub struct __pthread_attr {
        pub __schedparam: sched_param,
        pub __stackaddr: *mut ::c_void,
        pub __stacksize: size_t,
        pub __guardsize: size_t,
        pub __detachstate: __pthread_detachstate,
        pub __inheritsched: __pthread_inheritsched,
        pub __contentionscope: __pthread_contentionscope,
        pub __schedpolicy: ::c_int,
    }
   
    pub struct __pthread_mutexattr {
        pub __prioceiling: ::c_int,
        pub __protocol: __pthread_mutex_protocol,
        pub __pshared: __pthread_process_shared,
        pub __mutex_type: __pthread_mutex_type,
    }
   
    pub struct __pthread_mutex {
        pub __lock: ::c_uint,
        pub __owner_id: ::c_uint,
        pub __cnt: ::c_uint,
        pub __shpid: ::c_int,
        pub __type: ::c_int,
        pub __flags: ::c_int,
        pub __reserved1: ::c_uint,
        pub __reserved2: ::c_uint,
    }
   
    pub struct __pthread_condattr {
        pub __pshared: __pthread_process_shared,
        pub __clock: __clockid_t,
    }
   
    pub struct __pthread_cond {
        pub __lock: __pthread_spinlock_t,
        pub __queue: *mut __pthread,
        pub __attr: *mut __pthread_condattr,
        pub __wrefs: ::c_uint,
        pub __data: *mut ::c_void,
    }
   
    pub struct __pthread_rwlockattr {
        pub __pshared: __pthread_process_shared,
    }
   
    pub struct __pthread_rwlock {
        pub __held: __pthread_spinlock_t,
        pub __lock: __pthread_spinlock_t,
        pub __readers: ::c_int,
        pub __readerqueue: *mut __pthread,
        pub __writerqueue: *mut __pthread,
        pub __attr: *mut __pthread_rwlockattr,
        pub __data: *mut ::c_void,
    }
   
    pub struct __pthread_barrierattr {
        pub __pshared: __pthread_process_shared,
    }
   
    pub struct __pthread_barrier {
        pub __lock: __pthread_spinlock_t,
        pub __queue: *mut __pthread,
        pub __pending: ::c_uint,
        pub __count: ::c_uint,
        pub __attr: *mut __pthread_barrierattr,
        pub __data: *mut ::c_void,
    }
   
    pub struct __pthread_once {
        pub __run: ::c_int,
        pub __lock: __pthread_spinlock_t,
    }
     
    pub struct termios {
        pub c_iflag: tcflag_t,
        pub c_oflag: tcflag_t,
        pub c_cflag: tcflag_t,
        pub c_lflag: tcflag_t,
        pub c_cc: [cc_t; 20usize],
        pub __ispeed: speed_t,
        pub __ospeed: speed_t,
    }
   
    pub struct fd_set {
        pub __fds_bits: [__fd_mask; 8usize],
    }
   
    pub struct statvfs {
        pub __f_type: ::c_uint,
        pub f_bsize: ::c_ulong,
        pub f_blocks: __fsblkcnt_t,
        pub f_bfree: __fsblkcnt_t,
        pub f_bavail: __fsblkcnt_t,
        pub f_files: __fsfilcnt_t,
        pub f_ffree: __fsfilcnt_t,
        pub f_fsid: __fsid_t,
        pub f_namemax: ::c_ulong,
        pub f_favail: __fsfilcnt_t,
        pub f_frsize: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_spare: [::c_uint; 3usize],
    }
   
    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_addr: *mut sockaddr,
        pub ai_canonname: *mut ::c_char,
        pub ai_next: *mut addrinfo,
    }

    pub struct sigevent {
        pub sigev_value: __sigval_t,
        pub sigev_signo: ::c_int,
        pub sigev_notify: ::c_int,
        pub sigev_notify_function:
            unsafe extern "C" fn(arg1: __sigval_t),
        pub sigev_notify_attributes: *mut pthread_attr_t,
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_errno: ::c_int,
        pub si_code: ::c_int,
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_addr: *mut ::c_void,
        pub si_status: ::c_int,
        pub si_band: ::c_long,
        pub si_value: __sigval_t,
    }

    pub struct sigaction {
        pub __sigaction_handler: sigaction__bindgen_ty_1,
        pub sa_mask: __sigset_t,
        pub sa_flags: ::c_int,
    }

    pub struct iovec {
        pub iov_base: *mut ::c_void,
        pub iov_len: size_t,
    }

    pub struct msghdr {
        pub msg_name: *mut ::c_void,
        pub msg_namelen: socklen_t,
        pub msg_iov: *mut iovec,
        pub msg_iovlen: ::c_int,
        pub msg_control: *mut ::c_void,
        pub msg_controllen: socklen_t,
        pub msg_flags: ::c_int,
    }
}

extern "C" {
    pub fn __pthread_equal(
        __t1: __pthread_t,
        __t2: __pthread_t,
    ) -> ::c_int;

    pub fn getsockopt(
        __fd: ::c_int,
        __level: ::c_int,
        __optname: ::c_int,
        __optval: *mut ::c_void,
        __optlen: *mut socklen_t,
    ) -> ::c_int;

    pub fn setsockopt(
        __fd: ::c_int,
        __level: ::c_int,
        __optname: ::c_int,
        __optval: *const ::c_void,
        __optlen: socklen_t,
    ) -> ::c_int;

    pub fn bind(
        __fd: ::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> ::c_int;

    pub fn recvfrom(
        __fd: ::c_int,
        __buf: *mut ::c_void,
        __n: size_t,
        __flags: ::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;

    pub fn clock_gettime(
        __clock_id: clockid_t,
        __tp: *mut timespec,
    ) -> ::c_int;

    pub fn recvmsg(
        __fd: ::c_int,
        __message: *mut msghdr,
        __flags: ::c_int,
    ) -> ssize_t;

    pub fn sendmsg(
        __fd: ::c_int,
        __message: *const msghdr,
        __flags: ::c_int,
    ) -> ssize_t;

}