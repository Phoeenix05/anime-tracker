#[macro_export]
macro_rules! generate_impls {
    // ($api_impls:ident, $($api:ident),*) => {
    //     // let mut api_impls = vec![];
    //     $( $api_impls.push(Arc::new($api::default())) )*
    //     api_impls
    // };
    ($($api:ident),*) => {{
        let mut api_impls: Vec<Arc<dyn ApiImpl + Send + Sync>> = Vec::new();
        $( api_impls.push(Arc::new($api::default())); )*
        api_impls
    }};
}
