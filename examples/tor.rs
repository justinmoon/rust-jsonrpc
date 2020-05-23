extern crate jsonrpc;
extern crate serde;
extern crate serde_derive;

fn main() {
    let username = "";
    let password = "";
    let socks_url = "";
    let rpc_url = "";

    let client = jsonrpc::client::Client::new(
        rpc_url.to_owned(),
        Some(username.to_owned()),
        Some(password.to_owned()),
        Some(socks_url.to_owned()),
    );

    let command = "getblockchaininfo";
    let params = vec![];
    let request = client.build_request(&command, &params);
    match client.send_request(&request) {
        Ok(r) => println!("ok: {:?}", r),
        Err(e) => println!("err: {:?}", e),
    }
}
