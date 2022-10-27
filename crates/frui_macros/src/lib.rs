use proc_macro::TokenStream;

mod macros;

/// Implements `WidgetList` for tuples of size up to specified length.
#[proc_macro]
pub fn impl_widget_list(tokens: TokenStream) -> TokenStream {
    macros::impl_tuple_slice::impl_tuple_slice(tokens)
}

/// Duplicates trait definition for each identifier specified.
#[proc_macro_attribute]
pub fn copy_trait_as(args: TokenStream, body: TokenStream) -> TokenStream {
    macros::copy_trait_as::copy_trait_as(body, args)
}

#[proc_macro_derive(Builder)]
pub fn derive_builder(tokens: TokenStream) -> TokenStream {
    macros::builder::derive_builder(tokens)
}

//
// Widget Implementations
//

#[proc_macro_derive(LeafWidget)]
pub fn leaf_widget(tokens: TokenStream) -> TokenStream {
    macros::widget_impl::Leaf(&syn::parse_macro_input!(tokens as syn::ItemStruct)).into()
}

#[proc_macro_derive(SingleChildWidget)]
pub fn single_child_widget(tokens: TokenStream) -> TokenStream {
    macros::widget_impl::SingleChild(&syn::parse_macro_input!(tokens as syn::ItemStruct)).into()
}

#[proc_macro_derive(MultiChildWidget)]
pub fn multi_child_widget(tokens: TokenStream) -> TokenStream {
    macros::widget_impl::MultiChild(&syn::parse_macro_input!(tokens as syn::ItemStruct)).into()
}

#[proc_macro_derive(ViewWidget)]
pub fn view_widget(tokens: TokenStream) -> TokenStream {
    macros::widget_impl::View(&syn::parse_macro_input!(tokens as syn::ItemStruct)).into()
}

#[proc_macro_derive(InheritedWidget)]
pub fn inherited_widget(tokens: TokenStream) -> TokenStream {
    macros::widget_impl::Inherited(&syn::parse_macro_input!(tokens as syn::ItemStruct)).into()
}
