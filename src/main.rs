//! main.rs  패킷 분석 및 탐지/차단 프로그램
//!
//! 패킷 분석 및 탐지/차단 프로그램인 RWAF의 메인 실행 소스코드이다.

mod config;
mod log;
mod packet_capture;

use config::*;
use log::*;
use packet_capture::*;

use nfqueue::*;

struct Context {
    dummy: u8,
}

// 동기 처리 됨. 즉, 패킷이 처리되어 verdict 나올 때까지 대기. 패킷들은 queue에 쌓임.
fn callback(msg: &Message, _data: &mut Context) {
    print_log!(LogLevel::Debug, "Callback");

    // let payload = msg.get_payload();

    // if payload.is_empty() {
    //     println!("empty payload");
    //     msg.set_verdict(nfqueue::Verdict::Accept);
    //     return;
    // }

    // for b in payload {
    //     print!("{:02X} ", b);
    // }
    // println!();

    let verdict = parse_packet(msg.get_payload());

    // 커널로 패킷 송신
    msg.set_verdict(verdict);  // or Drop
}

fn main() {
    print_log!(LogLevel::Info, "Start RWAF");

    // init config
    set_config();

    // init logging
    set_log();

    // init nfqueue
    // nfqueue 핸들 생성
    let mut q = Queue::new(Context { dummy: 0 });

    // netlink 소켓 open
    q.open();
    print_log!(LogLevel::Debug, "소켓 Open");

    // bind. IPv4 패킷 처리
    q.unbind(libc::AF_INET);
    q.bind(libc::AF_INET);
    print_log!(LogLevel::Debug, "bind");

    // queue 0번에 callback 등록
    q.create_queue(0, callback);
    print_log!(LogLevel::Debug, "queue set");

    // 전체 패킷 복사하도록 설정
    q.set_mode(nfqueue::CopyMode::CopyPacket, 0xffff);
    print_log!(LogLevel::Debug, "mode set");

    // nfqueue run
    q.run_loop();

    q.close();
}