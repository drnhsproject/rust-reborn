pub mod features;
pub mod shared;

pub use features::product::domain::ProductRepository;
pub use features::product::infrastructure::product_repository_impl::PostgresProductRepository;
pub use features::product::presentation::product_routes::product_routes_handler;
