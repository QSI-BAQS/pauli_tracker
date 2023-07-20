use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{
        Parse,
        ParseStream,
    },
    parse_macro_input,
    Ident,
    Token,
};

struct Tracker {
    typ: Ident,
    pre: Ident,
}

impl Parse for Tracker {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let typ = input.parse()?;
        input.parse::<Token![,]>()?;
        let pre = input.parse()?;
        Ok(Self { typ, pre })
    }
}

fn concat(left: &str, right: &str, span: proc_macro2::Span) -> Ident {
    let mut s = String::new();
    s.push_str(left);
    s.push_str(right);
    Ident::new(&s, span)
}

#[proc_macro]
pub fn tracker_boilerplate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Tracker);
    let typ = input.typ;
    let span = input.pre.span();
    let pre = &input.pre.to_string();

    let init = concat(pre, "init", span);
    let free = concat(pre, "free", span);
    let track_x = concat(pre, "track_x", span);
    let track_y = concat(pre, "track_y", span);
    let track_z = concat(pre, "track_z", span);
    let h = concat(pre, "h", span);
    let s = concat(pre, "s", span);
    let cx = concat(pre, "cx", span);
    let cz = concat(pre, "cz", span);
    let new_qubit = concat(pre, "new_qubit", span);

    quote! {
        #[no_mangle]
        pub extern "C" fn #init(num_qubits: usize) -> *mut #typ {
            ManuallyDrop::new(Box::new(#typ::init(num_qubits))).as_mut() as *mut #typ
        }

        /// # Safety
        #[no_mangle]
        pub unsafe extern "C" fn #free(tracker: *mut #typ) {
            unsafe { Box::from_raw(tracker) };
        }

        #[no_mangle]
        pub extern "C" fn #track_x(tracker: &mut #typ, qubit: usize) {
            tracker.track_x(qubit);
        }

        #[no_mangle]
        pub extern "C" fn #track_y(tracker: &mut #typ, qubit: usize) {
            tracker.track_y(qubit);
        }

        #[no_mangle]
        pub extern "C" fn #track_z(tracker: &mut #typ, qubit: usize) {
            tracker.track_z(qubit);
        }

        #[no_mangle]
        pub extern "C" fn #h(tracker: &mut #typ, qubit: usize) {
            tracker.h(qubit);
        }

        #[no_mangle]
        pub extern "C" fn #s(tracker: &mut #typ, qubit: usize) {
            tracker.s(qubit);
        }

        #[no_mangle]
        pub extern "C" fn #cx(tracker: &mut #typ, control: usize, target: usize) {
            tracker.cx(control, target);
        }

        #[no_mangle]
        pub extern "C" fn #cz(tracker: &mut #typ, qubit_a: usize, qubit_b: usize) {
            tracker.cz(qubit_a, qubit_b);
        }

        #[no_mangle]
        pub extern "C" fn #new_qubit(tracker: &mut #typ, qubit: usize) {
            tracker.new_qubit(qubit);
        }
    }
    .into()
}

#[proc_macro]
pub fn serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Tracker);
    let typ = input.typ;
    let span = input.pre.span();
    let pre = &input.pre.to_string();

    let ser = concat(pre, "serialize", span);

    quote! {
        #[no_mangle]
        pub extern "C" fn #ser(storage: &#typ, file: *const std::ffi::c_char) {
            let file = unsafe {
                std::ffi::CStr::from_ptr(file as *const i8)
            }.to_str().unwrap();
            let output = serde_json::to_string(storage).unwrap();
            std::fs::write(file, output).unwrap();
        }
    }
    .into()
}
