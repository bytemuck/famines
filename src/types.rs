use crate::{AddrFuncResult, Processor};

pub(crate) type Byte = u8;
pub(crate) type SByte = i8;
pub(crate) type Word = u16;

pub(crate) type ExecFunc = fn(AddrFuncResult, &mut Processor);
pub(crate) type AddrFunc = fn(&mut Processor) -> AddrFuncResult;
