use candid::Principal;
use ic_web3_macros::{cross_canister_call_func, manage_single_state, manage_vec_state, manage_map_state, setup_func, timer_task_func};

#[ic_cdk::update]
async fn call(canister_id: String, name: String, msg: String) -> Result<String, String> {
    ic_cdk::println!("calling");
    let canister_id = Principal::from_text(canister_id).unwrap();
    call_greet(canister_id, (name, msg)).await
}

#[ic_cdk::query]
fn greet(name: String, msg: String) -> Result<String, String> {
    ic_cdk::println!("called");
    Ok(format!("Hello, {}! {}", name, msg))
}


// states
manage_single_state!("last_timestamp", u64, 100);
manage_single_state!("latest_result", String);
manage_vec_state!("vec_result", String);
manage_map_state!("balance", String, u64);
manage_map_state!("username", u64, String);

// cross-canister call
type CallCanisterArgs = (String, String);
type CallCanisterResponse = Result<String, String>;
cross_canister_call_func!("greet", CallCanisterArgs, CallCanisterResponse);

// setup
manage_single_state!("rpc", String);
manage_single_state!("chain_id", u8);
manage_single_state!("dst_address", String);
setup_func!({
    rpc: String,
    chain_id: u8,
    dst_address: String,
});

// timer task
manage_vec_state!("hello_ts", u64);
manage_map_state!("hello_msg", u64, String);
static hello: fn() -> () = || {
    let current_ts = ic_cdk::api::time();
    let msg = format!("Hello, {}!", current_ts);
    set_hello_ts(current_ts);
    set_hello_msg(current_ts, msg.clone());
    ic_cdk::println!("key={}, value={}", current_ts, msg);
};
timer_task_func!("set_task", "hello");

#[ic_cdk::query]
fn view_get_hello_tss() -> Vec<u64> {
    get_hello_tss()
}
#[ic_cdk::query]
fn view_get_hello_msg(key: u64) -> String {
    get_hello_msg(key)
}

#[cfg(test)]
mod test_lib {
    use super::*;

    #[test]
    fn test_setup() {
        let rpc = String::from("rpc");
        let chain_id = 1;
        let dst_address = String::from("dst_address");
        setup(rpc.clone(), chain_id, dst_address.clone());
        assert_eq!(get_rpc(), rpc);
        assert_eq!(get_chain_id(), chain_id);
        assert_eq!(get_dst_address(), dst_address);
    }

    #[test]
    fn test_last_timestamp() {
        assert_eq!(get_last_timestamp(), 100);
        set_last_timestamp(200);
        assert_eq!(get_last_timestamp(), 200);
    }

    #[test]
    fn test_latest_result() {
        assert_eq!(get_latest_result(), String::from(""));
        set_latest_result(String::from("UPDATED"));
        assert_eq!(get_latest_result(), String::from("UPDATED"));
    }

    #[test]
    fn test_vec_results() {
        assert_eq!(vec_results_len(), 0);
        let datum1 = String::from("RESULT1");
        let datum2 = String::from("RESULT2");
        set_vec_result(datum1.clone());
        set_vec_result(datum2.clone());
        assert_eq!(vec_results_len(), 2);
        assert_eq!(get_vec_results(), vec![datum1.clone(), datum2.clone()]);
        assert_eq!(get_vec_result(0), datum1.clone());
        assert_eq!(get_vec_result(1), datum2.clone());
    }

    #[test]
    fn test_balances() {
        assert_eq!(balances_len(), 0);
        let datum1 = String::from("BALANCE1");
        let datum2 = String::from("BALANCE2");
        set_balance(datum1.clone(), 100);
        set_balance(datum2.clone(), 200);
        assert_eq!(balances_len(), 2);
        assert_eq!(get_balance(datum1.clone()), 100);
        assert_eq!(get_balance(datum2.clone()), 200);
    }

    #[test]
    fn test_usernames() {
        assert_eq!(usernames_len(), 0);
        let datum1 = String::from("USERNAME1");
        let datum2 = String::from("USERNAME2");
        set_username(1, datum1.clone());
        set_username(2, datum2.clone());
        assert_eq!(usernames_len(), 2);
        assert_eq!(get_username(1), datum1.clone());
        assert_eq!(get_username(2), datum2.clone());
    }
}
