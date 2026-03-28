//! main.rs  패킷 분석 및 탐지/차단 프로그램
//!
//! 패킷 분석 및 탐지/차단 프로그램인 RWAF의 메인 실행 소스코드이다.

mod config;
mod log;

use config::*;
use log::*;

use nfqueue::*;

fn init_nfqueue() -> Queue<()> {
    // nfqueue 핸들 생성
    let mut q = Queue::new(());

    // netlink 소켓 open
    q.open();
    print_log!(LogLevel::Debug, "소켓 Open");

    // bind. IPv4 패킷 처리
    q.unbind(libc::AF_INET);
    q.bind(libc::AF_INET);

    // queue 0번에 callback 등록
    q.create_queue(0, callback);

    // 전체 패킷 복사하도록 설정
    q.set_mode(nfqueue::CopyMode::CopyPacket, 0xffff);

    q
}

// 동기 처리 됨. 즉, 패킷이 처리되어 verdict 나올 때까지 대기. 패킷들은 queue에 쌓임.
fn callback(msg: &Message, _data: &mut ()) {
    print_log!(LogLevel::Debug, "Callback");
    let payload_data = msg.get_payload();
    for byte in payload_data {
        print!("{:02X} ", byte);
    }
    println!();

    if let Ok(xml) = msg.as_xml_str(&[nfqueue::XMLFormatFlags::XmlAll]) {
        println!("XML\n{}", xml);
    }

    // 차후 패킷 처리 로직(packet_capture 호출)으로 변경
    // let verdict = packet_capture(msg.get_payload());
    // msg.set_verdict(verdict);

    // 커널로 패킷 송신
    msg.set_verdict(nfqueue::Verdict::Accept);  // or Drop
}

fn main() {
    print_log!(LogLevel::Info, "Start RWAF");

    // init config
    set_config();

    // init logging
    set_log();

    // init nfqueue
    let mut q = init_nfqueue();

    // nfqueue run
    q.run_loop();

    q.close();
}