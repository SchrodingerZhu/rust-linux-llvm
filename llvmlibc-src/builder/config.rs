
pub trait AddToCMake {
    fn add_to_cmake(&self, config: &mut cmake::Config);
}

#[derive(Debug, Clone)]
pub struct CodegenOpts {
    pub strong_stack_protector: bool,
    pub keep_frame_pointer: bool
}

fn bool_to_str(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}

impl Default for CodegenOpts {
    fn default() -> Self {
        CodegenOpts {
            strong_stack_protector: false,
            keep_frame_pointer: false
        }
    }
}

impl AddToCMake for CodegenOpts {
    fn add_to_cmake(&self, config: &mut cmake::Config) {
        config.define("LIBC_CONF_ENABLE_STRONG_STACK_PROTECTOR", bool_to_str(self.strong_stack_protector));
        config.define("LIBC_CONF_KEEP_FRAME_POINTER", bool_to_str(self.keep_frame_pointer));
    }
}

#[derive(Debug, Clone)]
pub enum ErrnoMode {
    Default,
    Undefined,
    ThreadLocal,
    Shared,
    External,
    System
}

impl Default for ErrnoMode {
    fn default() -> Self {
        ErrnoMode::Default
    }
}

impl AddToCMake for ErrnoMode {
    fn add_to_cmake(&self, config: &mut cmake::Config) {
        let value = match self {
            ErrnoMode::Default => "LIBC_ERRNO_MODE_DEFAULT",
            ErrnoMode::Undefined => "LIBC_ERRNO_MODE_UNDEFINED",
            ErrnoMode::ThreadLocal => "LIBC_ERRNO_MODE_THREAD_LOCAL",
            ErrnoMode::Shared => "LIBC_ERRNO_MODE_SHARED",
            ErrnoMode::External => "LIBC_ERRNO_MODE_EXTERNAL",
            ErrnoMode::System => "LIBC_ERRNO_MODE_SYSTEM",
        };
        config.define("LIBC_CONF_ERRNO_MODE", value);
    }
}

#[derive(Debug, Clone)]
pub enum MathOptimization {
    SkipAccuratePass,
    SmallTables,
    NoErrno,
    NoExcept,
    Fast
}

impl MathOptimization {
    fn to_str(&self) -> &'static str {
        match self {
            MathOptimization::SkipAccuratePass => "LIBC_MATH_SKIP_ACCURATE_PASS",
            MathOptimization::SmallTables => "LIBC_MATH_SMALL_TABLES",
            MathOptimization::NoErrno => "LIBC_MATH_NO_ERRNO",
            MathOptimization::NoExcept => "LIBC_MATH_NO_EXCEPT",
            MathOptimization::Fast => "LIBC_MATH_FAST",
        }
    }
}

#[derive(Debug, Clone)]
pub struct MathOpts {
    frexp_inf_nan_exponent: Option<String>,
    optimizations: Vec<MathOptimization>,
}

impl AddToCMake for MathOpts {
    fn add_to_cmake(&self, config: &mut cmake::Config) {
        if let Some(frexp_inf_nan_exponent) = &self.frexp_inf_nan_exponent {
            config.define("LIBC_CONF_FREXP_INF_NAN_EXPONENT", frexp_inf_nan_exponent);
        }
        // ; separated list of optimizations
        let optimizations = self.optimizations.iter().map(|o| o.to_str()).collect::<Vec<_>>().join(";");
        config.define("LIBC_CONF_MATH_OPTIMIZATIONS", &optimizations);
    }
}

impl Default for MathOpts {
    fn default() -> Self {
        MathOpts {
            frexp_inf_nan_exponent: None,
            optimizations: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrintfOpts {
    pub disable_fixed_point: bool,
    pub disable_float: bool,
    pub disable_index_mode: bool,
    pub disable_strerror: bool,
    pub disable_write_int: bool,
    pub float_to_str_no_specialize_ld: bool,
    pub float_to_str_use_dyadic_float: bool,
    pub float_to_str_use_mega_long_double_table: bool,
}

impl AddToCMake for PrintfOpts {
    fn add_to_cmake(&self, config: &mut cmake::Config) {
        config.define("LIBC_CONF_PRINTF_DISABLE_FIXED_POINT", bool_to_str(self.disable_fixed_point));
        config.define("LIBC_CONF_PRINTF_DISABLE_FLOAT", bool_to_str(self.disable_float));
        config.define("LIBC_CONF_PRINTF_DISABLE_INDEX_MODE", bool_to_str(self.disable_index_mode));
        config.define("LIBC_CONF_PRINTF_DISABLE_STRERROR", bool_to_str(self.disable_strerror));
        config.define("LIBC_CONF_PRINTF_DISABLE_WRITE_INT", bool_to_str(self.disable_write_int));
        config.define("LIBC_CONF_PRINTF_FLOAT_TO_STR_NO_SPECIALIZE_LD", bool_to_str(self.float_to_str_no_specialize_ld));
        config.define("LIBC_CONF_PRINTF_FLOAT_TO_STR_USE_DYADIC_FLOAT", bool_to_str(self.float_to_str_use_dyadic_float));
        config.define("LIBC_CONF_PRINTF_FLOAT_TO_STR_USE_MEGA_LONG_DOUBLE_TABLE", bool_to_str(self.float_to_str_use_mega_long_double_table));
    }
}

impl Default for PrintfOpts {
    fn default() -> Self {
        PrintfOpts {
            disable_fixed_point: false,
            disable_float: false,
            disable_index_mode: false,
            disable_strerror: false,
            disable_write_int: false,
            float_to_str_no_specialize_ld: false,
            float_to_str_use_dyadic_float: false,
            float_to_str_use_mega_long_double_table: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PThreadOpts {
    pub raw_mutex_default_spin_count: usize,
    pub rwlock_default_spin_count: usize,
    pub timeout_ensure_monotonicity: bool,
}

impl Default for PThreadOpts {
    fn default() -> Self {
        PThreadOpts {
            raw_mutex_default_spin_count: 100,
            rwlock_default_spin_count: 100,
            timeout_ensure_monotonicity: true,
        }
    }
}

impl AddToCMake for PThreadOpts {
    fn add_to_cmake(&self, config: &mut cmake::Config) {
        config.define("LIBC_CONF_PTHREAD_RAW_MUTEX_DEFAULT_SPIN_COUNT", self.raw_mutex_default_spin_count.to_string());
        config.define("LIBC_CONF_PTHREAD_RWLOCK_DEFAULT_SPIN_COUNT", self.rwlock_default_spin_count.to_string());
        config.define("LIBC_CONF_PTHREAD_TIMEOUT_ENSURE_MONOTONICITY", bool_to_str(self.timeout_ensure_monotonicity));
    }
}

#[derive(Debug, Clone)]
pub enum QSortImpl {
    QuickSort,
    HeapSort,
}

impl AddToCMake for QSortImpl {
    fn add_to_cmake(&self, config: &mut cmake::Config) {
        let value = match self {
            QSortImpl::QuickSort => "LIBC_QSORT_QUICK_SORT",
            QSortImpl::HeapSort => "LIBC_QSORT_HEAP_SORT",
        };
        config.define("LIBC_CONF_QSORT_IMPL", value);
    }
}


#[derive(Debug, Clone)]
pub struct Config {
    pub full_build: bool,
    pub enable_scudo: bool,
    pub codegen_opts: CodegenOpts,
    pub errno_mode: ErrnoMode,
    pub null_checks: bool,
    pub math_opts: MathOpts,
    pub printf_opts: PrintfOpts,
    pub pthread_opts: PThreadOpts,
    pub qsort_impl: QSortImpl,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            full_build: true,
            enable_scudo: true,
            codegen_opts: Default::default(),
            errno_mode: Default::default(),
            null_checks: true,
            math_opts: Default::default(),
            printf_opts: Default::default(),
            pthread_opts: Default::default(),
            qsort_impl: QSortImpl::QuickSort,
        }
    }
}