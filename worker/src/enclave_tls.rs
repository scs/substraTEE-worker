/*
	Copyright 2019 Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

extern crate sgx_types;
extern crate sgx_urts;
extern crate dirs;
use sgx_types::*;

use std::os::unix::io::{AsRawFd};
use std::net::{TcpListener, TcpStream};
use init_enclave::init_enclave;

extern {
	fn run_server(eid: sgx_enclave_id_t, retval: *mut sgx_status_t,
				  socket_fd: c_int, sign_type: sgx_quote_sign_type_t) -> sgx_status_t;
	fn run_client(eid: sgx_enclave_id_t, retval: *mut sgx_status_t,
				  socket_fd: c_int, sign_type: sgx_quote_sign_type_t) -> sgx_status_t;
}

pub enum Mode {
	Client,
	Server,
}

pub fn run(mode: Mode) {
	let sign_type = sgx_quote_sign_type_t::SGX_UNLINKABLE_SIGNATURE;

	let enclave = match init_enclave() {
		Ok(r) => {
			println!("[+] Init Enclave Successful {}!", r.geteid());
			r
		},
		Err(x) => {
			println!("[-] Init Enclave Failed {}!", x.as_str());
			return;
		},
	};

	match mode {
		Mode::Server => run_enclave_server(enclave.geteid(), sign_type),
		Mode::Client => run_enclave_client(enclave.geteid(), sign_type),
	}


	println!("[+] Done!");
	enclave.destroy();
}


pub fn run_enclave_server(eid: sgx_enclave_id_t, sign_type: sgx_quote_sign_type_t) {
	println!("Running as server...");
	let listener = TcpListener::bind("0.0.0.0:3443").unwrap();
	//loop{
	match listener.accept() {
		Ok((socket, addr)) => {
			println!("new client from {:?}", addr);
			let mut retval = sgx_status_t::SGX_SUCCESS;
			let result = unsafe {
				run_server(eid, &mut retval, socket.as_raw_fd(), sign_type)
			};
			match result {
				sgx_status_t::SGX_SUCCESS => {
					println!("ECALL success!");
				},
				_ => {
					println!("[-] ECALL Enclave Failed {}!", result.as_str());
					return;
				}
			}
		}
		Err(e) => println!("couldn't get client: {:?}", e),
	}
}

pub fn run_enclave_client(eid: sgx_enclave_id_t, sign_type: sgx_quote_sign_type_t) {
	println!("Running as client...");
	let socket = TcpStream::connect("localhost:3443").unwrap();
	let mut retval = sgx_status_t::SGX_SUCCESS;
	let result = unsafe {
		run_client(eid, &mut retval, socket.as_raw_fd(), sign_type)
	};
	match result {
		sgx_status_t::SGX_SUCCESS => {
			println!("ECALL success!");
		},
		_ => {
			println!("[-] ECALL Enclave Failed {}!", result.as_str());
			return;
		}
	}
}
