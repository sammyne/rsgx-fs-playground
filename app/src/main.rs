extern crate sgx_types;
extern crate sgx_urts;

use std::ffi::CString;

use sgx_types::*;
use sgx_urts::SgxEnclave;

extern "C" {
    fn bufio_read(eid: sgx_enclave_id_t, path: *const c_char) -> sgx_status_t;
    fn read_file(eid: sgx_enclave_id_t, path: *const c_char) -> sgx_status_t;
    fn read_to_vec(eid: sgx_enclave_id_t, path: *const c_char) -> sgx_status_t;
}

fn init_enclave(enclave_path: &str) -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // [DEPRECATED since v2.6] Step 1: try to retrieve the launch token saved by last transaction
    // if there is no token, then create a new one.
    //

    // Step 2: call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    const DEBUG: i32 = 1;
    let mut misc_attr = sgx_misc_attribute_t {
        secs_attr: sgx_attributes_t { flags: 0, xfrm: 0 },
        misc_select: 0,
    };
    let enclave = SgxEnclave::create(
        enclave_path,
        DEBUG,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr,
    )?;

    // [DEPRECATED since v2.6] Step 3: save the launch token if it is updated

    Ok(enclave)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("[usage] {} enclave-path data-path", &args[0]);
        std::process::exit(-1);
    }

    let (enclave_path, data_path) = (&args[1], &args[2]);
    println!("[+] enclave path: {}", enclave_path);
    println!("[+] data path: {}", data_path);

    let enclave = match init_enclave(enclave_path) {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        }
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        }
    };

    let data_path = CString::new(data_path.as_bytes()).expect("unfallible");

    println!("[-] bufio_read...");
    let status = unsafe { bufio_read(enclave.geteid(), data_path.as_ptr()) };
    match status {
        sgx_status_t::SGX_SUCCESS => {}
        _ => {
            println!("[-] ECALL Enclave Failed {:?}!", status);
            return;
        }
    }
    println!("[+] bufio_read...");

    println!("[-] read_to_vec...");
    //let status = unsafe { read_file(enclave.geteid(), data_path.as_ptr()) };
    let status = unsafe { read_to_vec(enclave.geteid(), data_path.as_ptr()) };
    match status {
        sgx_status_t::SGX_SUCCESS => {}
        _ => {
            println!("[-] ECALL Enclave Failed {:?}!", status);
            return;
        }
    }
    println!("[+] done read_to_vec...");

    enclave.destroy();
}
