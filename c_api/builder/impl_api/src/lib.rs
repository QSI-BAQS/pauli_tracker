use proc_macro::TokenStream;
use proc_macro2::Span;
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

fn concat<L: AsRef<str>, R: AsRef<str>>(left: L, right: R, span: Span) -> Ident {
    let mut s = String::new();
    s.push_str(left.as_ref());
    s.push_str(right.as_ref());
    Ident::new(&s, span)
}

struct Gen {
    typ: Ident,
    pre: Pre,
}

struct Pre {
    prefix: String,
    span: Span,
}

impl Parse for Gen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let typ = input.parse()?;
        input.parse::<Token![,]>()?;
        let pre: Ident = input.parse()?;
        Ok(Self {
            typ,
            pre: Pre {
                span: pre.span(),
                prefix: pre.to_string(),
            },
        })
    }
}

impl Pre {
    fn name(&self, name: &str) -> Ident {
        concat(&self.prefix, name, self.span)
    }
}

struct GenWithAdditional {
    gen: Gen,
    additional: Vec<Ident>,
}

impl Parse for GenWithAdditional {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let gen = input.parse()?;
        let mut additional = Vec::new();
        while input.parse::<Token![,]>().is_ok() {
            additional.push(input.parse()?);
        }
        Ok(Self { gen, additional })
    }
}

const MUST_FREE: &str =
    " The instance has to be freed manually with the according `*_free` function.";
const FREES: &str = " Frees the instance.";

#[proc_macro]
pub fn raw_vec(input: TokenStream) -> TokenStream {
    let GenWithAdditional {
        gen: Gen { typ, pre },
        mut additional,
    } = parse_macro_input!(input as GenWithAdditional);
    let get_raw = pre.name("get_raw");
    let ret = additional.pop().unwrap();
    quote! {
        #[no_mangle]
        pub extern "C" fn #get_raw(x: &mut #typ) -> #ret {
            #ret {
                data: x.as_mut_ptr(),
                len: x.len(),
            }
        }
    }
    .into()
}

#[proc_macro]
pub fn raw_vec_newtyped(input: TokenStream) -> TokenStream {
    let GenWithAdditional {
        gen: Gen { typ, pre },
        mut additional,
    } = parse_macro_input!(input as GenWithAdditional);
    let get_raw = pre.name("get_raw");
    let ret = additional.pop().unwrap();
    quote! {
        #[no_mangle]
        pub extern "C" fn #get_raw(x: &mut #typ) -> #ret {
            #ret {
                data: x.0.as_mut_ptr(),
                len: x.0.len(),
            }
        }
    }
    .into()
}

#[proc_macro]
pub fn basic(input: TokenStream) -> TokenStream {
    let Gen { typ, pre } = parse_macro_input!(input as Gen);

    let new = pre.name("new");
    let free = pre.name("free");
    let serialize = pre.name("serialize");
    let deserialize = pre.name("deserialize");

    quote! {
        #[doc = #MUST_FREE]
        #[no_mangle]
        pub extern "C" fn #new() -> *mut #typ {
            std::mem::ManuallyDrop::new(Box::new(#typ::default())).as_mut() as *mut #typ
        }

        #[doc = #FREES]
        #[no_mangle]
        pub unsafe extern "C" fn #free(x: *mut #typ) {
            unsafe { Box::from_raw(x) };
        }

        /// Serialize into json.
        #[no_mangle]
        pub unsafe extern "C"
        fn #serialize(x: &#typ, file: *const std::ffi::c_char) {
            let file = unsafe {
                std::ffi::CStr::from_ptr(file as *const i8)
            }.to_str().expect("invalid file name");
            let output = serde_json::to_string(x).expect("serialize error");
            std::fs::write(file, output).unwrap();
        }

        /// Deserialize from json.
        ///
        #[doc = #MUST_FREE]
        #[no_mangle]
        pub extern "C"
        fn #deserialize(file: *const std::ffi::c_char) -> *mut #typ {
            let file = unsafe {
                std::ffi::CStr::from_ptr(file as *const i8)
            }.to_str().expect("invalid file name");
            let contents = std::fs::read_to_string(file).expect("cannot read file");
            let x: #typ = serde_json::from_str(&contents).expect("deserialize error");
            std::mem::ManuallyDrop::new(Box::new(x)).as_mut() as *mut #typ
        }
    }
    .into()
}

#[proc_macro]
pub fn pauli(input: TokenStream) -> TokenStream {
    let Gen { typ, pre } = parse_macro_input!(input as Gen);
    let tableau_encoding = pre.name("tableau_encoding");
    quote! {
        #[no_mangle]
        pub extern "C" fn #tableau_encoding(x: &mut #typ) -> u8 {
            <#typ as Pauli>::tableau_encoding(x)
        }
    }
    .into()
}

#[proc_macro]
pub fn pauli_stack(input: TokenStream) -> TokenStream {
    let GenWithAdditional {
        gen: Gen { typ, pre },
        mut additional,
    } = parse_macro_input!(input as GenWithAdditional);

    let left = pre.name("left");
    let right = pre.name("right");
    let inner_type = additional.pop().unwrap();

    quote! {
        #[no_mangle]
        pub extern "C" fn #left(x: &mut #typ) -> *mut #inner_type
        {
            &mut x.left as *mut #inner_type
        }

        #[no_mangle]
        pub extern "C" fn #right(x: &mut #typ) -> *mut #inner_type
        {
            &mut x.right as *mut #inner_type
        }
    }
    .into()
}

#[proc_macro]
pub fn boolean_vector(input: TokenStream) -> TokenStream {
    let Gen { typ, pre } = parse_macro_input!(input as Gen);

    let get = pre.name("get");
    let len = pre.name("len");
    let is_empty = pre.name("is_empty");

    quote! {
        #[no_mangle]
        pub extern "C" fn #get(x: &mut #typ, key: usize)
            -> bool {
            <#typ as BooleanVector>::get(x, key).expect("missing key")
        }

        #[no_mangle]
        pub extern "C" fn #len(x: &#typ) -> usize {
            <#typ as BooleanVector>::len(x)
        }

        #[no_mangle]
        pub extern "C" fn #is_empty(x: &#typ) -> bool {
            <#typ as BooleanVector>::is_empty(x)
        }
    }
    .into()
}

/// Don't know how to make associated types work with cbindgen, so we just pass it in as
/// additional argument.
#[proc_macro]
pub fn base(input: TokenStream) -> TokenStream {
    let GenWithAdditional {
        gen: Gen { typ, pre },
        mut additional,
    } = parse_macro_input!(input as GenWithAdditional);

    let tb = additional.pop().unwrap();

    let get = pre.name("get");
    let len = pre.name("len");
    let is_empty = pre.name("is_empty");

    quote! {
        #[no_mangle]
        pub extern "C" fn #get(x: &mut #typ, key: usize)
            -> &mut #tb {
            <#typ as Base>::get_mut(x, key).expect("missing key")
        }

        #[no_mangle]
        pub extern "C" fn #len(x: &#typ) -> usize {
            <#typ as Base>::len(x)
        }

        #[no_mangle]
        pub extern "C" fn #is_empty(x: &#typ) -> bool {
            <#typ as Base>::is_empty(x)
        }
    }
    .into()
}

#[proc_macro]
pub fn init(input: TokenStream) -> TokenStream {
    let Gen { typ, pre } = parse_macro_input!(input as Gen);
    let init = pre.name("init");
    quote! {
        #[doc = #MUST_FREE]
        #[no_mangle]
        pub extern "C" fn #init(num_qubits: usize) -> *mut #typ {
            std::mem::ManuallyDrop::new(Box::new(<#typ as Init>::init(num_qubits)))
                .as_mut() as *mut #typ
        }
    }
    .into()
}

#[proc_macro]
pub fn tracker(input: TokenStream) -> TokenStream {
    let GenWithAdditional {
        gen: Gen { typ, pre },
        mut additional,
    } = parse_macro_input!(input as GenWithAdditional);

    let is_frames = additional.pop().unwrap();
    let stack = additional.pop().unwrap();

    let track_x = pre.name("track_x");
    let track_y = pre.name("track_y");
    let track_z = pre.name("track_z");

    let h = pre.name("h");
    let s = pre.name("s");
    let cz = pre.name("cz");

    let x = pre.name("x");
    let y = pre.name("y");
    let z = pre.name("z");
    let sdg = pre.name("sdg");
    let sx = pre.name("sx");
    let sxdg = pre.name("sxdg");
    let sy = pre.name("sy");
    let sydg = pre.name("sydg");
    let sz = pre.name("sz");
    let szdg = pre.name("szdg");

    let cx = pre.name("cx");
    let swap = pre.name("swap");

    let move_x_to_x = pre.name("move_x_to_x");
    let move_x_to_z = pre.name("move_x_to_z");
    let move_z_to_x = pre.name("move_z_to_x");
    let move_z_to_z = pre.name("move_z_to_z");

    let new_qubit = pre.name("new_qubit");
    let measure = pre.name("measure");

    #[allow(clippy::cmp_owned)]
    let measure_fn = if is_frames.to_string() == "is_frames" {
        quote! {
            #[doc = #MUST_FREE]
            #[no_mangle]
            pub extern "C" fn #measure(tracker: &mut #typ, qubit: usize)
                -> *mut #stack {
            std::mem::ManuallyDrop::new(
                Box::new(<#typ as Tracker>::measure(tracker, qubit).unwrap()))
                .as_mut() as *mut #stack

            }
        }
    } else {
        quote! {
            #[no_mangle]
            pub extern "C"
            fn #measure(tracker: &mut #typ, qubit: usize) -> #stack {
                <#typ as Tracker>::measure(tracker, qubit).unwrap()
            }
        }
    };

    quote! {
        #[no_mangle]
        pub extern "C" fn #track_x(tracker: &mut #typ, qubit: usize) {
            <#typ as Tracker>::track_x(tracker, qubit);
        }

        #[no_mangle]
        pub extern "C" fn #track_y(tracker: &mut #typ, qubit: usize) {
            <#typ as Tracker>::track_y(tracker, qubit);
        }

        #[no_mangle]
        pub extern "C" fn #track_z(tracker: &mut #typ, qubit: usize) {
            <#typ as Tracker>::track_z(tracker, qubit);
        }

        #[no_mangle]
        pub extern "C" fn #h(tracker: &mut #typ, qubit: usize) {
            <#typ as Tracker>::h(tracker, qubit);
        }
        #[no_mangle]
        pub extern "C" fn #s(tracker: &mut #typ, qubit: usize) {
            <#typ as Tracker>::s(tracker, qubit);
        }
        #[no_mangle]
        pub extern "C"
        fn #cz(tracker: &mut #typ, qubit_a: usize, qubit_b: usize) {
            <#typ as Tracker>::cz(tracker, qubit_a, qubit_b);
        }

        #[no_mangle]
        pub extern "C" fn #x(_: &mut #typ, _: usize) {}
        #[no_mangle]
        pub extern "C" fn #y(_: &mut #typ, _: usize) {}
        #[no_mangle]
        pub extern "C" fn #z(_: &mut #typ, _: usize) {}
        #[no_mangle]
        pub extern "C" fn #sdg(tracker: &mut #typ, qubit: usize) {
            <#typ as Tracker>::sdg(tracker, qubit);
        }
        #[no_mangle]
        pub extern "C" fn #sx(tracker: &mut #typ, qubit: usize) {
            <#typ as Tracker>::sx(tracker, qubit);
        }
        #[no_mangle]
        pub extern "C" fn #sxdg(tracker: &mut #typ, qubit: usize) {
            <#typ as Tracker>::sxdg(tracker, qubit);
        }
        #[no_mangle]
        pub extern "C" fn #sy(tracker: &mut #typ, qubit: usize) {
            <#typ as Tracker>::sy(tracker, qubit);
        }
        #[no_mangle]
        pub extern "C" fn #sydg(tracker: &mut #typ, qubit: usize) {
            <#typ as Tracker>::sydg(tracker, qubit);
        }
        #[no_mangle]
        pub extern "C" fn #sz(tracker: &mut #typ, qubit: usize) {
            <#typ as Tracker>::sz(tracker, qubit);
        }
        #[no_mangle]
        pub extern "C" fn #szdg(tracker: &mut #typ, qubit: usize) {
            <#typ as Tracker>::szdg(tracker, qubit);
        }

        #[no_mangle]
        pub extern "C"
        fn #cx(tracker: &mut #typ, control: usize, target: usize) {
            <#typ as Tracker>::cx(tracker, control, target);
        }
        #[no_mangle]
        pub extern "C"
        fn #swap(tracker: &mut #typ, qubit_a: usize, qubit_b: usize) {
            <#typ as Tracker>::swap(tracker, qubit_b, qubit_a);
        }

        #[no_mangle]
        pub extern "C"
        fn #move_x_to_x(tracker: &mut #typ, source: usize, destination: usize) {
            <#typ as Tracker>::move_x_to_x(tracker, source, destination);
        }
        #[no_mangle]
        pub extern "C"
        fn #move_x_to_z(tracker: &mut #typ, source: usize, destination: usize) {
            <#typ as Tracker>::move_x_to_z(tracker, source, destination);
        }
        #[no_mangle]
        pub extern "C"
        fn #move_z_to_x(tracker: &mut #typ, source: usize, destination: usize) {
            <#typ as Tracker>::move_z_to_x(tracker, source, destination);
        }
        #[no_mangle]
        pub extern "C"
        fn #move_z_to_z(tracker: &mut #typ, source: usize, destination: usize) {
            <#typ as Tracker>::move_z_to_z(tracker, source, destination);
        }


        #[no_mangle]
        pub extern "C" fn #new_qubit(tracker: &mut #typ, qubit: usize) {
            <#typ as Tracker>::new_qubit(tracker, qubit);
        }

        #measure_fn
    }
    .into()
}

#[proc_macro]
pub fn measure_and_store(input: TokenStream) -> TokenStream {
    let GenWithAdditional {
        gen: Gen { typ, pre },
        mut additional,
    } = parse_macro_input!(input as GenWithAdditional);

    let measure_and_store = pre.name("measure_and_store");
    let measure_and_store_all = pre.name("measure_and_store_all");
    let storage = additional.pop().unwrap();

    quote! {
        #[no_mangle]
        pub extern "C" fn #measure_and_store(
            tracker: &mut #typ,
            bit: usize,
            storage: &mut #storage,
        ) {
            tracker.measure_and_store(bit, storage).unwrap();
        }

        #[no_mangle]
        pub extern "C" fn #measure_and_store_all(
            tracker: &mut #typ,
            storage: &mut #storage,
        ) {
            tracker.measure_and_store_all(storage);
        }
    }
    .into()
}
