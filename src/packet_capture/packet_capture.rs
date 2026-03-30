//! packet_capture.rs  패킷 파싱 및 분석
//!
//! 로그 설정을 세팅하고, 일반 동작/탐지/설정 관련 로그를 출력한다.

use crate::print_log;
use crate::log::LogLevel;

use std::net::Ipv4Addr;
use std::fmt::Write;

/// 파싱 된 패킷을 저장하는 구조체.
struct ParsedPacket {
	//ethernet_type,
	src_ip: Ipv4Addr,
	dst_ip: Ipv4Addr,
	src_port: u16,
	dst_port: u16,
	//seq,
	payload: Vec<u8>,
}

/// 캡쳐한 패킷을 파싱한다.
pub fn parse_packet(packet: &[u8]) -> nfqueue::Verdict {
    print_log!(LogLevel::Debug, "패킷 처리 : ");

    if packet.is_empty() {
        print_log!(LogLevel::Debug, "Empty packet");
        return nfqueue::Verdict::Accept;
    }

    print_packet(packet);

    print_log!(LogLevel::Debug, "\n");

    nfqueue::Verdict::Accept
}

/// 캡쳐한 패킷을 Hex와 ASCII 형태로 출력한다.
pub fn print_packet(packet: &[u8]) {
    let mut offset = 0;

    for chunk in packet.chunks(16) {
        let mut hex_part = String::new();
        let mut ascii_part = String::new();

        for &b in chunk {
            // HEX로 출력
            write!(&mut hex_part, "{:02X} ", b).unwrap();

            // ASCII로 출력
            if b.is_ascii_graphic() || b == b' ' {
                ascii_part.push(b as char);
            } else {
                ascii_part.push('.');
            }
        }

        // 정렬 맞추기
        while hex_part.len() < 16 * 3 {
            hex_part.push(' ');
        }

        print_log!(
            LogLevel::Debug,
            "{:04X} {} {}",
            offset,
            hex_part,
            ascii_part
        );

        offset += 16;
    }
}