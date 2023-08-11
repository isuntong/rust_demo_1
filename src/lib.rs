#![feature(impl_trait_in_assoc_type)]

pub struct S;

#[volo::async_trait]
impl volo_gen::rust_demo_1::ItemService for S {
	async fn get_item(&self, _req: volo_gen::rust_demo_1::GetItemRequest) -> ::core::result::Result<volo_gen::rust_demo_1::GetItemResponse, ::volo_thrift::AnyhowError>{
					Ok(Default::default())
				}
}
