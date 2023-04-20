use sui_sdk::{SuiClient, SuiClientBuilder};
use sui_types::{
    base_types::{ObjectID},
};
use sui_json_rpc_types::SuiObjectDataOptions;

#[tokio::main]
async fn main() {
    let url = "https://fullnode.devnet.sui.io:443";
    let client: SuiClient = SuiClientBuilder::default().build(url).await.unwrap();
    let res = client
        .read_api()
        .get_object_with_options(
         ObjectID::from_hex_literal("0xbd4a8807df7b05c6f4569f3ef92c05ea38ea2e1eaac2455b7412fd8953f82fcd").unwrap(),
         SuiObjectDataOptions { 
            show_type: true, 
            show_owner: true, 
            show_previous_transaction: true, 
            show_display: true, 
            show_content: true, 
            show_bcs: true, 
            show_storage_rebate: true
        })
        .await
        .unwrap();
    println!("{:?}", res);
}