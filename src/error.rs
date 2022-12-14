use concat_idents::concat_idents;

#[repr(u64)]
#[derive(EnumString, Clone, Copy, Display, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorCode {
    #[strum(serialize = "none")]
    None = 0u64,

    #[strum(serialize = "not a file")]
    NotAFile = 0x100u64,
    #[strum(serialize = "not a directory")]
    NotADirectory = 0x101u64,
    #[strum(serialize = "file not found")]
    FileNotFound = 0x102u64,
    #[strum(serialize = "directory unusable")]
    DirectoryUnusable = 0x103u64,
    #[strum(serialize = "file extension not matched")]
    FileExtensionNotMatched = 0x104u64,
    #[strum(serialize = "data not found")]
    DataNotFound = 0x105u64,
    #[strum(serialize = "data empty")]
    DataEmpty = 0x106u64,
    #[strum(disabled)]
    EnumVisitorEmpty = 0x107u64,
    #[strum(disabled)]
    UniqueBoxLocked = 0x108u64,
    #[strum(disabled)]
    UniqueRefLocked = 0x109u64,
    #[strum(serialize = "serialization failed")]
    SerializationFailed = 0x10au64,
    #[strum(serialize = "deserialization failed")]
    DeserializationFailed = 0x10bu64,

    #[strum(serialize = "lack of pipelines")]
    LackOfPipelines = 0x1000u64,
    #[strum(serialize = "solver not found")]
    SolverNotFound = 0x1001u64,
    #[strum(serialize = "OR engine environment lost")]
    OREngineEnvironmentLost = 0x1002u64,
    #[strum(serialize = "OR engine connection overtime")]
    OREngineConnectionOvertime = 0x1003u64,
    #[strum(serialize = "OR engine modeling exception")]
    OREngineModelingException = 0x1004u64,
    #[strum(serialize = "OR engine solving exception")]
    OREngineSolvingException = 0x1005u64,
    #[strum(serialize = "OR engine terminated")]
    OREngineTerminated = 0x1006u64,
    #[strum(serialize = "OR model no solution")]
    ORModelNoSolution = 0x1007u64,
    #[strum(serialize = "OR model unbounded")]
    ORModelUnbounded = 0x1008u64,
    #[strum(serialize = "OR solution invalid")]
    ORSolutionInvalid = 0x1009u64,

    #[strum(serialize = "application failed")]
    ApplicationFailed = 0x10000u64,
    #[strum(serialize = "application error")]
    ApplicationError = 0x10001u64,
    #[strum(serialize = "application exception")]
    ApplicationException = 0x10002u64,
    #[strum(serialize = "application stopped")]
    ApplicationStopped = 0x10003u64,

    #[strum(serialize = "other")]
    Other = u64::MAX - 1,
    #[strum(serialize = "unknown")]
    Unknown = u64::MAX
}

impl From<u64> for ErrorCode {
    fn from(value: u64) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<u64> for ErrorCode {
    fn into(self) -> u64 {
        self as u64
    }
}

trait Error {
    fn what(&self) -> &str;
}

trait ExError<T: Sized>: Error {
    fn arg(&self) -> &Option<T>;
}

trait LogicError: Error {}
trait RuntimeError: Error {
    fn code(&self) -> ErrorCode;
}

trait ExLogicError<T: Sized> : LogicError + ExError<T> {}
trait ExRuntimeError<T: Sized> : RuntimeError + ExError<T> {}

macro_rules! logic_error_template {
    ($($type:ident)*) => ($(
        pub struct $type {
            msg: String
        }

        impl Error for $type {
            fn what(&self) -> &str { &self.msg }
        }

        impl LogicError for $type {}

        concat_idents!(ex_error_name = Ex, $type {
            pub struct ex_error_name<T: Sized> {
                msg: String,
                arg: Option<T>
            }

            impl<T: Sized> Error for ex_error_name<T> {
                fn what(&self) -> &str { &self.msg }
            }

            impl<T: Sized> ExError<T> for ex_error_name<T> {
                fn arg(&self) -> &Option<T> { &self.arg }
            }

            impl<T: Sized> LogicError for ex_error_name<T> {}
            impl<T: Sized> ExLogicError<T> for ex_error_name<T> {}
        });
    )*)
}
logic_error_template! { InvalidArgument DomainError LengthError OutOfRange }

macro_rules! runtime_error_template {
    ($($type:ident)*) => ($(
        pub struct $type {
            code: ErrorCode,
            msg: String
        }

        impl Error for $type {
            fn what(&self) -> &str { &self.msg }
        }

        impl RuntimeError for $type {
            fn code(&self) -> ErrorCode { self.code }
        }

        concat_idents!(ex_error_name = Ex, $type {
            pub struct ex_error_name<T: Sized> {
                code: ErrorCode,
                msg: String,
                arg: Option<T>
            }

            impl<T: Sized> Error for ex_error_name<T> {
                fn what(&self) -> &str { &self.msg }
            }

            impl<T: Sized> ExError<T> for ex_error_name<T> {
                fn arg(&self) -> &Option<T> { &self.arg }
            }

            impl<T: Sized> RuntimeError for ex_error_name<T> {
                fn code(&self) -> ErrorCode { self.code }
            }

            impl<T: Sized> ExRuntimeError<T> for ex_error_name<T> {}
        });
    )*)
}
runtime_error_template! { ApplicationError RangeError OverflowError UnderflowError SystemError FilesystemError }
