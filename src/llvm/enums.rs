#[repr(C)]
pub enum Linkage {
	External = 0,
	DLLImport = 10,
}

#[repr(C)]
pub enum CallingConvention {
	Win64 = 79,
}

#[repr(C)]
pub enum CodegenFileType {
	Object = 1,
}