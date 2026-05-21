#[cfg(test)]
mod tests {
    use arp::{arphdr, arp_send, neighbour, ndisc_ops, net_device, htons};
    use arp::{ARPHRD_ETHER, ETH_P_IP, ETH_HLEN, ARPOP_REQUEST};
    use kernel_types::{sk_buff, in_addr, c_int, c_void};
    use core::ptr;

    unsafe extern "C" fn mock_output(skb: *mut sk_buff, neighbour: *mut neighbour) -> c_int {
        // Mock returning 0 for success
        0
    }

    #[test]
    fn test_arp_send_success() {
        unsafe {
            // Setup mocks
            let mut ops = ndisc_ops {
                output: Some(mock_output),
            };

            let mut dev = net_device {
                type_: ARPHRD_ETHER,
            };

            let mut dst = neighbour {
                dev: &mut dev as *mut _,
                ops: &mut ops as *mut _,
            };

            // Setup SKB data
            let mut data = [0u64; 16];
            let data_ptr = data.as_mut_ptr() as *mut u8;
            let eth_start = data_ptr.add(2); // NET_IP_ALIGN
            
            // Set ethernet header (ETH_P_IP)
            // Ethernet header is 14 bytes: dest(6) + src(6) + proto(2)
            let eth_proto_ptr = eth_start.add(12) as *mut u16;
            ptr::write_unaligned(eth_proto_ptr, htons(ETH_P_IP));

            // Set ARP header (ARPOP_REQUEST)
            let arp_ptr = eth_start.add(ETH_HLEN) as *mut arphdr;
            ptr::write_unaligned(core::ptr::addr_of_mut!((*arp_ptr).ar_op), htons(ARPOP_REQUEST));

            // Setup fake IP addresses
            let mut s_addr = in_addr { s_addr: 1, ip: ptr::null_mut() };
            let mut d_addr = in_addr { s_addr: 2, ip: ptr::null_mut() };
            ptr::write_unaligned(core::ptr::addr_of_mut!((*arp_ptr).ar_sip), &mut s_addr as *mut _);
            ptr::write_unaligned(core::ptr::addr_of_mut!((*arp_ptr).ar_tip), &mut d_addr as *mut _);

            let mut skb = sk_buff {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
                tstamp: 0,
                dev: &mut dev as *mut _ as *mut c_void,
                len: 128,
                data_len: 0,
                mac_len: 14,
                hdr_len: 0,
                csum: 0,
                priority: 0,
                protocol: 0,
                flags: 0,
                cb: [0u8; 48],
                ip_summed: 0,
                csum_level: 0,
                csum_valid: 0,
                csum_complete_sw: 0,
                remcsum_offload: ptr::null_mut(),
                mark: ptr::null_mut(),
                data: eth_start as *mut c_void,
                sk: ptr::null_mut(),
                dst: &mut dst as *mut _ as *mut c_void,
                head: eth_start,
                network_header: 0,
                transport_header: 0,
                transport_offset: 0,
                network_header_len: 0,
            };

            let mut mock_ip: u32 = 0;
            let ip = &mut mock_ip as *mut _ as *mut c_void;

            // Call function
            let result = arp_send(&mut skb as *mut _, ip);
            assert_eq!(result, 0);
        }
    }

    #[test]
    fn test_arp_send_invalid_dev() {
        unsafe {
            let mut dev = net_device {
                type_: 0, // Not ARPHRD_ETHER
            };

            let mut skb = sk_buff {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
                tstamp: 0,
                dev: &mut dev as *mut _ as *mut c_void,
                len: 128,
                data_len: 0,
                mac_len: 14,
                hdr_len: 0,
                csum: 0,
                priority: 0,
                protocol: 0,
                flags: 0,
                cb: [0u8; 48],
                ip_summed: 0,
                csum_level: 0,
                csum_valid: 0,
                csum_complete_sw: 0,
                remcsum_offload: ptr::null_mut(),
                mark: ptr::null_mut(),
                data: ptr::null_mut(),
                sk: ptr::null_mut(),
                dst: ptr::null_mut(),
                head: ptr::null_mut(),
                network_header: 0,
                transport_header: 0,
                transport_offset: 0,
                network_header_len: 0,
            };

            let mut mock_ip: u32 = 0;
            let ip = &mut mock_ip as *mut _ as *mut c_void;

            // Should fail because dev type is not ARPHRD_ETHER
            let result = arp_send(&mut skb as *mut _, ip);
            assert_eq!(result, -kernel_types::EINVAL);
        }
    }
}
