pub mod abi_types;
pub mod cairo_struct;
pub use cairo_struct::CairoStruct;
pub mod cairo_enum;
pub use cairo_enum::CairoEnum;
pub mod cairo_function;
pub use cairo_function::CairoFunction;

#[derive(Debug, Clone)]
pub enum CairoAny {
    Struct(CairoStruct),
    Enum(CairoEnum),
    Function(Box<CairoFunction>),
    Basic,
}

impl CairoAny {
    pub fn is_generic(&self) -> bool {
        match self {
            CairoAny::Struct(s) => s.is_generic(),
            CairoAny::Enum(e) => e.is_generic(),
            CairoAny::Function(f) => f.is_generic(),
            CairoAny::Basic => false,
        }
    }

}
