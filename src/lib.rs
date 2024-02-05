use math_lingo_impl::evaluate_math_lingo;

#[proc_macro]
pub fn math(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(evaluate_math_lingo(proc_macro2::TokenStream::from(input)))
}