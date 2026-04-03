//! packet_capture.rs  패킷 파싱 및 분석
//!
//! 로그 설정을 세팅하고, 일반 동작/탐지/설정 관련 로그를 출력한다.

use crate::print_log;
use crate::log::LogLevel;

use std::net::Ipv4Addr;
use std::fmt::Write;

/// 파싱 된 패킷을 저장하는 구조체.
struct ParsedPacket<'a> {
    ip: IpHeader,
    tcp: TcpHeader,
    payload: &'a [u8],
}

// IP Header 정보 저장.
struct IpHeader {
    src: Ipv4Addr,
    dst_ip: Ipv4Addr,
    header_len: usize,
}

// TCP Header 정보 저장.
struct TcpHeader {
    src_port: u16,
	dst_port: u16,
    header_len: usize,
}

// IP header 길이 반환
fn parse_ipv4(packet: &[u8]) -> Option<IpHeader> {

}

// TCP header 길이 반환
fn parse_tcp(packet: &[u8], ip_header_len: usize) -> Option<TcpHeader> {

}

// slice 반환
fn extract_payload(packet: &[u8], offset: usize) -> &[u8] {

}

/// 캡쳐한 패킷을 파싱한다.
pub fn parse_packet(packet: &[u8]) -> Option<ParsedPacket> {
    print_log!(LogLevel::Debug, "패킷 처리 : ");

    if packet.is_empty() {
        print_log!(LogLevel::Debug, "Empty packet");
        return nfqueue::Verdict::Accept;
    }

    // print_packet(packet);

    parse_ipv4(packet);

    parse_tcp(packet);

    extract_payloat(packet);

    print_log!(LogLevel::Debug, "\n");

    nfqueue::Verdict::Accept
}

// 파싱 된 패킷을 대상으로 정책 탐지 및 차단 처리한다.
pub fn handle_packet(packet: &ParsedPacket) -> nfqueue::Verdict {
    // 탐지 및 차단

    Accept;  // 차후 구현 예정
}

/// 캡쳐한 패킷을 Hex와 ASCII 형태로 출력한다.
pub fn print_packet(packet: &[u8]) {
    let mut offset = 0;

    // 16 바이트 단위로 처리
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

pub fn flow_manager() {

}

pub fn inspect_packet() {

}