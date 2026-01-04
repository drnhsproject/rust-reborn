use crate::features::product::application::dto::create_product_result::CreateProductResult;
use crate::features::product::application::CreateProductRequest;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::features::product::presentation::product_controller::create_product_handler
    ),
    components(
        schemas(CreateProductRequest, CreateProductResult),
    ),
    tags(
        (name = "Products", description = "Product management APIs")
    )
)]
pub struct ProductApiDoc;
