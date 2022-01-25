#[allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Windows {
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Win32 {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Foundation {
            #[repr(transparent)]
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct PSTR(pub *mut u8);
            impl PSTR {
                pub const NULL: Self = Self(::std::ptr::null_mut());
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl ::std::default::Default for PSTR {
                fn default() -> Self {
                    Self(::std::ptr::null_mut())
                }
            }
            impl ::std::cmp::PartialEq for PSTR {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            unsafe impl ::windows::Abi for PSTR {
                type Abi = Self;
                fn drop_param(param: &mut ::windows::Param<'_, Self>) {
                    if let ::windows::Param::Boxed(value) = param {
                        if !value.0.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, PSTR> for &'a str {
                fn into_param(self) -> ::windows::Param<'a, PSTR> {
                    ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                        self.bytes()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u8>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            impl<'a> ::windows::IntoParam<'a, PSTR> for String {
                fn into_param(self) -> ::windows::Param<'a, PSTR> {
                    ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                        self.bytes()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u8>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[repr(transparent)]
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct PWSTR(pub *mut u16);
            impl PWSTR {
                pub const NULL: Self = Self(::std::ptr::null_mut());
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl ::std::default::Default for PWSTR {
                fn default() -> Self {
                    Self(::std::ptr::null_mut())
                }
            }
            impl ::std::cmp::PartialEq for PWSTR {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            unsafe impl ::windows::Abi for PWSTR {
                type Abi = Self;
                fn drop_param(param: &mut ::windows::Param<'_, Self>) {
                    if let ::windows::Param::Boxed(value) = param {
                        if !value.0.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, PWSTR> for &'a str {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_utf16()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            impl<'a> ::windows::IntoParam<'a, PWSTR> for String {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_utf16()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod NetworkManagement {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod IpHelper {
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct ADDRESS_FAMILY(pub u32);
                pub const AF_INET: ADDRESS_FAMILY = ADDRESS_FAMILY(2u32);
                pub const AF_INET6: ADDRESS_FAMILY = ADDRESS_FAMILY(23u32);
                pub const AF_UNSPEC: ADDRESS_FAMILY = ADDRESS_FAMILY(0u32);
                impl ::std::convert::From<u32> for ADDRESS_FAMILY {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for ADDRESS_FAMILY {
                    type Abi = Self;
                }
                impl ::std::ops::BitOr for ADDRESS_FAMILY {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for ADDRESS_FAMILY {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for ADDRESS_FAMILY {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for ADDRESS_FAMILY {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct GET_ADAPTERS_ADDRESSES_FLAGS(pub u32);
                pub const GAA_FLAG_SKIP_UNICAST: GET_ADAPTERS_ADDRESSES_FLAGS =
                    GET_ADAPTERS_ADDRESSES_FLAGS(1u32);
                pub const GAA_FLAG_SKIP_ANYCAST: GET_ADAPTERS_ADDRESSES_FLAGS =
                    GET_ADAPTERS_ADDRESSES_FLAGS(2u32);
                pub const GAA_FLAG_SKIP_MULTICAST: GET_ADAPTERS_ADDRESSES_FLAGS =
                    GET_ADAPTERS_ADDRESSES_FLAGS(4u32);
                pub const GAA_FLAG_SKIP_DNS_SERVER: GET_ADAPTERS_ADDRESSES_FLAGS =
                    GET_ADAPTERS_ADDRESSES_FLAGS(8u32);
                pub const GAA_FLAG_INCLUDE_PREFIX: GET_ADAPTERS_ADDRESSES_FLAGS =
                    GET_ADAPTERS_ADDRESSES_FLAGS(16u32);
                pub const GAA_FLAG_SKIP_FRIENDLY_NAME: GET_ADAPTERS_ADDRESSES_FLAGS =
                    GET_ADAPTERS_ADDRESSES_FLAGS(32u32);
                pub const GAA_FLAG_INCLUDE_WINS_INFO: GET_ADAPTERS_ADDRESSES_FLAGS =
                    GET_ADAPTERS_ADDRESSES_FLAGS(64u32);
                pub const GAA_FLAG_INCLUDE_GATEWAYS: GET_ADAPTERS_ADDRESSES_FLAGS =
                    GET_ADAPTERS_ADDRESSES_FLAGS(128u32);
                pub const GAA_FLAG_INCLUDE_ALL_INTERFACES: GET_ADAPTERS_ADDRESSES_FLAGS =
                    GET_ADAPTERS_ADDRESSES_FLAGS(256u32);
                pub const GAA_FLAG_INCLUDE_ALL_COMPARTMENTS: GET_ADAPTERS_ADDRESSES_FLAGS =
                    GET_ADAPTERS_ADDRESSES_FLAGS(512u32);
                pub const GAA_FLAG_INCLUDE_TUNNEL_BINDINGORDER: GET_ADAPTERS_ADDRESSES_FLAGS =
                    GET_ADAPTERS_ADDRESSES_FLAGS(1024u32);
                impl ::std::convert::From<u32> for GET_ADAPTERS_ADDRESSES_FLAGS {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for GET_ADAPTERS_ADDRESSES_FLAGS {
                    type Abi = Self;
                }
                impl ::std::ops::BitOr for GET_ADAPTERS_ADDRESSES_FLAGS {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for GET_ADAPTERS_ADDRESSES_FLAGS {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for GET_ADAPTERS_ADDRESSES_FLAGS {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for GET_ADAPTERS_ADDRESSES_FLAGS {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                pub unsafe fn GetAdaptersAddresses(
                    family: ADDRESS_FAMILY,
                    flags: GET_ADAPTERS_ADDRESSES_FLAGS,
                    reserved: *mut ::std::ffi::c_void,
                    adapteraddresses: *mut IP_ADAPTER_ADDRESSES_LH,
                    sizepointer: *mut u32,
                ) -> u32 {
                    #[cfg(windows)]
                    {
                        #[link(name = "IPHLPAPI")]
                        extern "system" {
                            fn GetAdaptersAddresses(
                                family: ADDRESS_FAMILY,
                                flags: GET_ADAPTERS_ADDRESSES_FLAGS,
                                reserved: *mut ::std::ffi::c_void,
                                adapteraddresses: *mut IP_ADAPTER_ADDRESSES_LH,
                                sizepointer: *mut u32,
                            ) -> u32;
                        }
                        GetAdaptersAddresses(
                            ::std::mem::transmute(family),
                            ::std::mem::transmute(flags),
                            ::std::mem::transmute(reserved),
                            ::std::mem::transmute(adapteraddresses),
                            ::std::mem::transmute(sizepointer),
                        )
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct IF_OPER_STATUS(pub i32);
                pub const IfOperStatusUp: IF_OPER_STATUS = IF_OPER_STATUS(1i32);
                pub const IfOperStatusDown: IF_OPER_STATUS = IF_OPER_STATUS(2i32);
                pub const IfOperStatusTesting: IF_OPER_STATUS = IF_OPER_STATUS(3i32);
                pub const IfOperStatusUnknown: IF_OPER_STATUS = IF_OPER_STATUS(4i32);
                pub const IfOperStatusDormant: IF_OPER_STATUS = IF_OPER_STATUS(5i32);
                pub const IfOperStatusNotPresent: IF_OPER_STATUS = IF_OPER_STATUS(6i32);
                pub const IfOperStatusLowerLayerDown: IF_OPER_STATUS = IF_OPER_STATUS(7i32);
                impl ::std::convert::From<i32> for IF_OPER_STATUS {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for IF_OPER_STATUS {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_ADDRESSES_LH {
                    pub Anonymous1: IP_ADAPTER_ADDRESSES_LH_0,
                    pub Next: *mut IP_ADAPTER_ADDRESSES_LH,
                    pub AdapterName: super::super::Foundation::PSTR,
                    pub FirstUnicastAddress: *mut IP_ADAPTER_UNICAST_ADDRESS_LH,
                    pub FirstAnycastAddress: *mut IP_ADAPTER_ANYCAST_ADDRESS_XP,
                    pub FirstMulticastAddress: *mut IP_ADAPTER_MULTICAST_ADDRESS_XP,
                    pub FirstDnsServerAddress: *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP,
                    pub DnsSuffix: super::super::Foundation::PWSTR,
                    pub Description: super::super::Foundation::PWSTR,
                    pub FriendlyName: super::super::Foundation::PWSTR,
                    pub PhysicalAddress: [u8; 8],
                    pub PhysicalAddressLength: u32,
                    pub Anonymous2: IP_ADAPTER_ADDRESSES_LH_1,
                    pub Mtu: u32,
                    pub IfType: u32,
                    pub OperStatus: IF_OPER_STATUS,
                    pub Ipv6IfIndex: u32,
                    pub ZoneIndices: [u32; 16],
                    pub FirstPrefix: *mut IP_ADAPTER_PREFIX_XP,
                    pub TransmitLinkSpeed: u64,
                    pub ReceiveLinkSpeed: u64,
                    pub FirstWinsServerAddress: *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH,
                    pub FirstGatewayAddress: *mut IP_ADAPTER_GATEWAY_ADDRESS_LH,
                    pub Ipv4Metric: u32,
                    pub Ipv6Metric: u32,
                    pub Luid: NET_LUID_LH,
                    pub Dhcpv4Server: super::super::Networking::WinSock::SOCKET_ADDRESS,
                    pub CompartmentId: u32,
                    pub NetworkGuid: ::windows::Guid,
                    pub ConnectionType: NET_IF_CONNECTION_TYPE,
                    pub TunnelType: TUNNEL_TYPE,
                    pub Dhcpv6Server: super::super::Networking::WinSock::SOCKET_ADDRESS,
                    pub Dhcpv6ClientDuid: [u8; 130],
                    pub Dhcpv6ClientDuidLength: u32,
                    pub Dhcpv6Iaid: u32,
                    pub FirstDnsSuffix: *mut IP_ADAPTER_DNS_SUFFIX,
                }
                impl IP_ADAPTER_ADDRESSES_LH {}
                unsafe impl ::windows::Abi for IP_ADAPTER_ADDRESSES_LH {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub union IP_ADAPTER_ADDRESSES_LH_0 {
                    pub Alignment: u64,
                    pub Anonymous: IP_ADAPTER_ADDRESSES_LH_0_0,
                }
                impl IP_ADAPTER_ADDRESSES_LH_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_ADDRESSES_LH_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_ADDRESSES_LH_0_0 {
                    pub Length: u32,
                    pub IfIndex: u32,
                }
                impl IP_ADAPTER_ADDRESSES_LH_0_0 {}
                impl ::std::default::Default for IP_ADAPTER_ADDRESSES_LH_0_0 {
                    fn default() -> Self {
                        Self {
                            Length: 0,
                            IfIndex: 0,
                        }
                    }
                }
                impl ::std::fmt::Debug for IP_ADAPTER_ADDRESSES_LH_0_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_Anonymous_e__Struct")
                            .field("Length", &self.Length)
                            .field("IfIndex", &self.IfIndex)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for IP_ADAPTER_ADDRESSES_LH_0_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self.Length == other.Length && self.IfIndex == other.IfIndex
                    }
                }
                impl ::std::cmp::Eq for IP_ADAPTER_ADDRESSES_LH_0_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_ADDRESSES_LH_0_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub union IP_ADAPTER_ADDRESSES_LH_1 {
                    pub Flags: u32,
                    pub Anonymous: IP_ADAPTER_ADDRESSES_LH_1_0,
                }
                impl IP_ADAPTER_ADDRESSES_LH_1 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_ADDRESSES_LH_1 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_ADDRESSES_LH_1_0 {
                    pub _bitfield: u32,
                }
                impl IP_ADAPTER_ADDRESSES_LH_1_0 {}
                impl ::std::default::Default for IP_ADAPTER_ADDRESSES_LH_1_0 {
                    fn default() -> Self {
                        Self { _bitfield: 0 }
                    }
                }
                impl ::std::fmt::Debug for IP_ADAPTER_ADDRESSES_LH_1_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_Anonymous_e__Struct")
                            .field("_bitfield", &self._bitfield)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for IP_ADAPTER_ADDRESSES_LH_1_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self._bitfield == other._bitfield
                    }
                }
                impl ::std::cmp::Eq for IP_ADAPTER_ADDRESSES_LH_1_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_ADDRESSES_LH_1_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_ANYCAST_ADDRESS_XP {
                    pub Anonymous: IP_ADAPTER_ANYCAST_ADDRESS_XP_0,
                    pub Next: *mut IP_ADAPTER_ANYCAST_ADDRESS_XP,
                    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
                }
                impl IP_ADAPTER_ANYCAST_ADDRESS_XP {}
                unsafe impl ::windows::Abi for IP_ADAPTER_ANYCAST_ADDRESS_XP {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub union IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
                    pub Alignment: u64,
                    pub Anonymous: IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0,
                }
                impl IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
                    pub Length: u32,
                    pub Flags: u32,
                }
                impl IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {}
                impl ::std::default::Default for IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
                    fn default() -> Self {
                        Self {
                            Length: 0,
                            Flags: 0,
                        }
                    }
                }
                impl ::std::fmt::Debug for IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_Anonymous_e__Struct")
                            .field("Length", &self.Length)
                            .field("Flags", &self.Flags)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self.Length == other.Length && self.Flags == other.Flags
                    }
                }
                impl ::std::cmp::Eq for IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
                    pub Anonymous: IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0,
                    pub Next: *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP,
                    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
                }
                impl IP_ADAPTER_DNS_SERVER_ADDRESS_XP {}
                unsafe impl ::windows::Abi for IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub union IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
                    pub Alignment: u64,
                    pub Anonymous: IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0,
                }
                impl IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
                    pub Length: u32,
                    pub Reserved: u32,
                }
                impl IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {}
                impl ::std::default::Default for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
                    fn default() -> Self {
                        Self {
                            Length: 0,
                            Reserved: 0,
                        }
                    }
                }
                impl ::std::fmt::Debug for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_Anonymous_e__Struct")
                            .field("Length", &self.Length)
                            .field("Reserved", &self.Reserved)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self.Length == other.Length && self.Reserved == other.Reserved
                    }
                }
                impl ::std::cmp::Eq for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_DNS_SUFFIX {
                    pub Next: *mut IP_ADAPTER_DNS_SUFFIX,
                    pub String: [u16; 256],
                }
                impl IP_ADAPTER_DNS_SUFFIX {}
                impl ::std::default::Default for IP_ADAPTER_DNS_SUFFIX {
                    fn default() -> Self {
                        Self {
                            Next: ::std::ptr::null_mut(),
                            String: [0; 256],
                        }
                    }
                }
                impl ::std::fmt::Debug for IP_ADAPTER_DNS_SUFFIX {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("IP_ADAPTER_DNS_SUFFIX")
                            .field("Next", &self.Next)
                            .field("String", &self.String)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for IP_ADAPTER_DNS_SUFFIX {
                    fn eq(&self, other: &Self) -> bool {
                        self.Next == other.Next && self.String == other.String
                    }
                }
                impl ::std::cmp::Eq for IP_ADAPTER_DNS_SUFFIX {}
                unsafe impl ::windows::Abi for IP_ADAPTER_DNS_SUFFIX {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_GATEWAY_ADDRESS_LH {
                    pub Anonymous: IP_ADAPTER_GATEWAY_ADDRESS_LH_0,
                    pub Next: *mut IP_ADAPTER_GATEWAY_ADDRESS_LH,
                    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
                }
                impl IP_ADAPTER_GATEWAY_ADDRESS_LH {}
                unsafe impl ::windows::Abi for IP_ADAPTER_GATEWAY_ADDRESS_LH {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub union IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
                    pub Alignment: u64,
                    pub Anonymous: IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0,
                }
                impl IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
                    pub Length: u32,
                    pub Reserved: u32,
                }
                impl IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {}
                impl ::std::default::Default for IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
                    fn default() -> Self {
                        Self {
                            Length: 0,
                            Reserved: 0,
                        }
                    }
                }
                impl ::std::fmt::Debug for IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_Anonymous_e__Struct")
                            .field("Length", &self.Length)
                            .field("Reserved", &self.Reserved)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self.Length == other.Length && self.Reserved == other.Reserved
                    }
                }
                impl ::std::cmp::Eq for IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_MULTICAST_ADDRESS_XP {
                    pub Anonymous: IP_ADAPTER_MULTICAST_ADDRESS_XP_0,
                    pub Next: *mut IP_ADAPTER_MULTICAST_ADDRESS_XP,
                    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
                }
                impl IP_ADAPTER_MULTICAST_ADDRESS_XP {}
                unsafe impl ::windows::Abi for IP_ADAPTER_MULTICAST_ADDRESS_XP {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub union IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
                    pub Alignment: u64,
                    pub Anonymous: IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0,
                }
                impl IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
                    pub Length: u32,
                    pub Flags: u32,
                }
                impl IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {}
                impl ::std::default::Default for IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
                    fn default() -> Self {
                        Self {
                            Length: 0,
                            Flags: 0,
                        }
                    }
                }
                impl ::std::fmt::Debug for IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_Anonymous_e__Struct")
                            .field("Length", &self.Length)
                            .field("Flags", &self.Flags)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self.Length == other.Length && self.Flags == other.Flags
                    }
                }
                impl ::std::cmp::Eq for IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_PREFIX_XP {
                    pub Anonymous: IP_ADAPTER_PREFIX_XP_0,
                    pub Next: *mut IP_ADAPTER_PREFIX_XP,
                    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
                    pub PrefixLength: u32,
                }
                impl IP_ADAPTER_PREFIX_XP {}
                unsafe impl ::windows::Abi for IP_ADAPTER_PREFIX_XP {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub union IP_ADAPTER_PREFIX_XP_0 {
                    pub Alignment: u64,
                    pub Anonymous: IP_ADAPTER_PREFIX_XP_0_0,
                }
                impl IP_ADAPTER_PREFIX_XP_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_PREFIX_XP_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_PREFIX_XP_0_0 {
                    pub Length: u32,
                    pub Flags: u32,
                }
                impl IP_ADAPTER_PREFIX_XP_0_0 {}
                impl ::std::default::Default for IP_ADAPTER_PREFIX_XP_0_0 {
                    fn default() -> Self {
                        Self {
                            Length: 0,
                            Flags: 0,
                        }
                    }
                }
                impl ::std::fmt::Debug for IP_ADAPTER_PREFIX_XP_0_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_Anonymous_e__Struct")
                            .field("Length", &self.Length)
                            .field("Flags", &self.Flags)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for IP_ADAPTER_PREFIX_XP_0_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self.Length == other.Length && self.Flags == other.Flags
                    }
                }
                impl ::std::cmp::Eq for IP_ADAPTER_PREFIX_XP_0_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_PREFIX_XP_0_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_UNICAST_ADDRESS_LH {
                    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_LH_0,
                    pub Next: *mut IP_ADAPTER_UNICAST_ADDRESS_LH,
                    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
                    pub PrefixOrigin: super::super::Networking::WinSock::NL_PREFIX_ORIGIN,
                    pub SuffixOrigin: super::super::Networking::WinSock::NL_SUFFIX_ORIGIN,
                    pub DadState: super::super::Networking::WinSock::NL_DAD_STATE,
                    pub ValidLifetime: u32,
                    pub PreferredLifetime: u32,
                    pub LeaseLifetime: u32,
                    pub OnLinkPrefixLength: u8,
                }
                impl IP_ADAPTER_UNICAST_ADDRESS_LH {}
                unsafe impl ::windows::Abi for IP_ADAPTER_UNICAST_ADDRESS_LH {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub union IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
                    pub Alignment: u64,
                    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_LH_0_0,
                }
                impl IP_ADAPTER_UNICAST_ADDRESS_LH_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
                    pub Length: u32,
                    pub Flags: u32,
                }
                impl IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {}
                impl ::std::default::Default for IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
                    fn default() -> Self {
                        Self {
                            Length: 0,
                            Flags: 0,
                        }
                    }
                }
                impl ::std::fmt::Debug for IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_Anonymous_e__Struct")
                            .field("Length", &self.Length)
                            .field("Flags", &self.Flags)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self.Length == other.Length && self.Flags == other.Flags
                    }
                }
                impl ::std::cmp::Eq for IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
                    pub Anonymous: IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0,
                    pub Next: *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH,
                    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
                }
                impl IP_ADAPTER_WINS_SERVER_ADDRESS_LH {}
                unsafe impl ::windows::Abi for IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub union IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
                    pub Alignment: u64,
                    pub Anonymous: IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0,
                }
                impl IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
                    pub Length: u32,
                    pub Reserved: u32,
                }
                impl IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {}
                impl ::std::default::Default for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
                    fn default() -> Self {
                        Self {
                            Length: 0,
                            Reserved: 0,
                        }
                    }
                }
                impl ::std::fmt::Debug for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_Anonymous_e__Struct")
                            .field("Length", &self.Length)
                            .field("Reserved", &self.Reserved)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self.Length == other.Length && self.Reserved == other.Reserved
                    }
                }
                impl ::std::cmp::Eq for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {}
                unsafe impl ::windows::Abi for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
                    type Abi = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct NET_IF_CONNECTION_TYPE(pub i32);
                pub const NET_IF_CONNECTION_DEDICATED: NET_IF_CONNECTION_TYPE =
                    NET_IF_CONNECTION_TYPE(1i32);
                pub const NET_IF_CONNECTION_PASSIVE: NET_IF_CONNECTION_TYPE =
                    NET_IF_CONNECTION_TYPE(2i32);
                pub const NET_IF_CONNECTION_DEMAND: NET_IF_CONNECTION_TYPE =
                    NET_IF_CONNECTION_TYPE(3i32);
                pub const NET_IF_CONNECTION_MAXIMUM: NET_IF_CONNECTION_TYPE =
                    NET_IF_CONNECTION_TYPE(4i32);
                impl ::std::convert::From<i32> for NET_IF_CONNECTION_TYPE {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for NET_IF_CONNECTION_TYPE {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub union NET_LUID_LH {
                    pub Value: u64,
                    pub Info: NET_LUID_LH_0,
                }
                impl NET_LUID_LH {}
                unsafe impl ::windows::Abi for NET_LUID_LH {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct NET_LUID_LH_0 {
                    pub _bitfield: u64,
                }
                impl NET_LUID_LH_0 {}
                impl ::std::default::Default for NET_LUID_LH_0 {
                    fn default() -> Self {
                        Self { _bitfield: 0 }
                    }
                }
                impl ::std::fmt::Debug for NET_LUID_LH_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_Info_e__Struct")
                            .field("_bitfield", &self._bitfield)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for NET_LUID_LH_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self._bitfield == other._bitfield
                    }
                }
                impl ::std::cmp::Eq for NET_LUID_LH_0 {}
                unsafe impl ::windows::Abi for NET_LUID_LH_0 {
                    type Abi = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct TUNNEL_TYPE(pub i32);
                pub const TUNNEL_TYPE_NONE: TUNNEL_TYPE = TUNNEL_TYPE(0i32);
                pub const TUNNEL_TYPE_OTHER: TUNNEL_TYPE = TUNNEL_TYPE(1i32);
                pub const TUNNEL_TYPE_DIRECT: TUNNEL_TYPE = TUNNEL_TYPE(2i32);
                pub const TUNNEL_TYPE_6TO4: TUNNEL_TYPE = TUNNEL_TYPE(11i32);
                pub const TUNNEL_TYPE_ISATAP: TUNNEL_TYPE = TUNNEL_TYPE(13i32);
                pub const TUNNEL_TYPE_TEREDO: TUNNEL_TYPE = TUNNEL_TYPE(14i32);
                pub const TUNNEL_TYPE_IPHTTPS: TUNNEL_TYPE = TUNNEL_TYPE(15i32);
                impl ::std::convert::From<i32> for TUNNEL_TYPE {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for TUNNEL_TYPE {
                    type Abi = Self;
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Networking {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod WinSock {
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IN6_ADDR {
                    pub u: IN6_ADDR_0,
                }
                impl IN6_ADDR {}
                unsafe impl ::windows::Abi for IN6_ADDR {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub union IN6_ADDR_0 {
                    pub Byte: [u8; 16],
                    pub Word: [u16; 8],
                }
                impl IN6_ADDR_0 {}
                unsafe impl ::windows::Abi for IN6_ADDR_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IN_ADDR {
                    pub S_un: IN_ADDR_0,
                }
                impl IN_ADDR {}
                unsafe impl ::windows::Abi for IN_ADDR {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub union IN_ADDR_0 {
                    pub S_un_b: IN_ADDR_0_0,
                    pub S_un_w: IN_ADDR_0_1,
                    pub S_addr: u32,
                }
                impl IN_ADDR_0 {}
                unsafe impl ::windows::Abi for IN_ADDR_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IN_ADDR_0_0 {
                    pub s_b1: u8,
                    pub s_b2: u8,
                    pub s_b3: u8,
                    pub s_b4: u8,
                }
                impl IN_ADDR_0_0 {}
                impl ::std::default::Default for IN_ADDR_0_0 {
                    fn default() -> Self {
                        Self {
                            s_b1: 0,
                            s_b2: 0,
                            s_b3: 0,
                            s_b4: 0,
                        }
                    }
                }
                impl ::std::fmt::Debug for IN_ADDR_0_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_S_un_b_e__Struct")
                            .field("s_b1", &self.s_b1)
                            .field("s_b2", &self.s_b2)
                            .field("s_b3", &self.s_b3)
                            .field("s_b4", &self.s_b4)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for IN_ADDR_0_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self.s_b1 == other.s_b1
                            && self.s_b2 == other.s_b2
                            && self.s_b3 == other.s_b3
                            && self.s_b4 == other.s_b4
                    }
                }
                impl ::std::cmp::Eq for IN_ADDR_0_0 {}
                unsafe impl ::windows::Abi for IN_ADDR_0_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct IN_ADDR_0_1 {
                    pub s_w1: u16,
                    pub s_w2: u16,
                }
                impl IN_ADDR_0_1 {}
                impl ::std::default::Default for IN_ADDR_0_1 {
                    fn default() -> Self {
                        Self { s_w1: 0, s_w2: 0 }
                    }
                }
                impl ::std::fmt::Debug for IN_ADDR_0_1 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_S_un_w_e__Struct")
                            .field("s_w1", &self.s_w1)
                            .field("s_w2", &self.s_w2)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for IN_ADDR_0_1 {
                    fn eq(&self, other: &Self) -> bool {
                        self.s_w1 == other.s_w1 && self.s_w2 == other.s_w2
                    }
                }
                impl ::std::cmp::Eq for IN_ADDR_0_1 {}
                unsafe impl ::windows::Abi for IN_ADDR_0_1 {
                    type Abi = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct NL_DAD_STATE(pub i32);
                pub const NldsInvalid: NL_DAD_STATE = NL_DAD_STATE(0i32);
                pub const NldsTentative: NL_DAD_STATE = NL_DAD_STATE(1i32);
                pub const NldsDuplicate: NL_DAD_STATE = NL_DAD_STATE(2i32);
                pub const NldsDeprecated: NL_DAD_STATE = NL_DAD_STATE(3i32);
                pub const NldsPreferred: NL_DAD_STATE = NL_DAD_STATE(4i32);
                pub const IpDadStateInvalid: NL_DAD_STATE = NL_DAD_STATE(0i32);
                pub const IpDadStateTentative: NL_DAD_STATE = NL_DAD_STATE(1i32);
                pub const IpDadStateDuplicate: NL_DAD_STATE = NL_DAD_STATE(2i32);
                pub const IpDadStateDeprecated: NL_DAD_STATE = NL_DAD_STATE(3i32);
                pub const IpDadStatePreferred: NL_DAD_STATE = NL_DAD_STATE(4i32);
                impl ::std::convert::From<i32> for NL_DAD_STATE {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for NL_DAD_STATE {
                    type Abi = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct NL_PREFIX_ORIGIN(pub i32);
                pub const IpPrefixOriginOther: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(0i32);
                pub const IpPrefixOriginManual: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(1i32);
                pub const IpPrefixOriginWellKnown: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(2i32);
                pub const IpPrefixOriginDhcp: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(3i32);
                pub const IpPrefixOriginRouterAdvertisement: NL_PREFIX_ORIGIN =
                    NL_PREFIX_ORIGIN(4i32);
                pub const IpPrefixOriginUnchanged: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(16i32);
                impl ::std::convert::From<i32> for NL_PREFIX_ORIGIN {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for NL_PREFIX_ORIGIN {
                    type Abi = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct NL_SUFFIX_ORIGIN(pub i32);
                pub const NlsoOther: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(0i32);
                pub const NlsoManual: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(1i32);
                pub const NlsoWellKnown: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(2i32);
                pub const NlsoDhcp: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(3i32);
                pub const NlsoLinkLayerAddress: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(4i32);
                pub const NlsoRandom: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(5i32);
                pub const IpSuffixOriginOther: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(0i32);
                pub const IpSuffixOriginManual: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(1i32);
                pub const IpSuffixOriginWellKnown: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(2i32);
                pub const IpSuffixOriginDhcp: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(3i32);
                pub const IpSuffixOriginLinkLayerAddress: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(4i32);
                pub const IpSuffixOriginRandom: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(5i32);
                pub const IpSuffixOriginUnchanged: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(16i32);
                impl ::std::convert::From<i32> for NL_SUFFIX_ORIGIN {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for NL_SUFFIX_ORIGIN {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct SCOPE_ID {
                    pub Anonymous: SCOPE_ID_0,
                }
                impl SCOPE_ID {}
                unsafe impl ::windows::Abi for SCOPE_ID {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub union SCOPE_ID_0 {
                    pub Anonymous: SCOPE_ID_0_0,
                    pub Value: u32,
                }
                impl SCOPE_ID_0 {}
                unsafe impl ::windows::Abi for SCOPE_ID_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct SCOPE_ID_0_0 {
                    pub _bitfield: u32,
                }
                impl SCOPE_ID_0_0 {}
                impl ::std::default::Default for SCOPE_ID_0_0 {
                    fn default() -> Self {
                        Self { _bitfield: 0 }
                    }
                }
                impl ::std::fmt::Debug for SCOPE_ID_0_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_Anonymous_e__Struct")
                            .field("_bitfield", &self._bitfield)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for SCOPE_ID_0_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self._bitfield == other._bitfield
                    }
                }
                impl ::std::cmp::Eq for SCOPE_ID_0_0 {}
                unsafe impl ::windows::Abi for SCOPE_ID_0_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct SOCKADDR {
                    pub sa_family: u16,
                    pub sa_data: [super::super::System::SystemServices::CHAR; 14],
                }
                impl SOCKADDR {}
                impl ::std::default::Default for SOCKADDR {
                    fn default() -> Self {
                        Self {
                            sa_family: 0,
                            sa_data: [::std::default::Default::default(); 14],
                        }
                    }
                }
                impl ::std::fmt::Debug for SOCKADDR {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("SOCKADDR")
                            .field("sa_family", &self.sa_family)
                            .field("sa_data", &self.sa_data)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for SOCKADDR {
                    fn eq(&self, other: &Self) -> bool {
                        self.sa_family == other.sa_family && self.sa_data == other.sa_data
                    }
                }
                impl ::std::cmp::Eq for SOCKADDR {}
                unsafe impl ::windows::Abi for SOCKADDR {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct SOCKADDR_IN {
                    pub sin_family: u16,
                    pub sin_port: u16,
                    pub sin_addr: IN_ADDR,
                    pub sin_zero: [super::super::System::SystemServices::CHAR; 8],
                }
                impl SOCKADDR_IN {}
                unsafe impl ::windows::Abi for SOCKADDR_IN {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct SOCKADDR_IN6 {
                    pub sin6_family: u16,
                    pub sin6_port: u16,
                    pub sin6_flowinfo: u32,
                    pub sin6_addr: IN6_ADDR,
                    pub Anonymous: SOCKADDR_IN6_0,
                }
                impl SOCKADDR_IN6 {}
                unsafe impl ::windows::Abi for SOCKADDR_IN6 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub union SOCKADDR_IN6_0 {
                    pub sin6_scope_id: u32,
                    pub sin6_scope_struct: SCOPE_ID,
                }
                impl SOCKADDR_IN6_0 {}
                unsafe impl ::windows::Abi for SOCKADDR_IN6_0 {
                    type Abi = Self;
                }
                #[repr(C)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct SOCKET_ADDRESS {
                    pub lpSockaddr: *mut SOCKADDR,
                    pub iSockaddrLength: i32,
                }
                impl SOCKET_ADDRESS {}
                impl ::std::default::Default for SOCKET_ADDRESS {
                    fn default() -> Self {
                        Self {
                            lpSockaddr: ::std::ptr::null_mut(),
                            iSockaddrLength: 0,
                        }
                    }
                }
                impl ::std::fmt::Debug for SOCKET_ADDRESS {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("SOCKET_ADDRESS")
                            .field("lpSockaddr", &self.lpSockaddr)
                            .field("iSockaddrLength", &self.iSockaddrLength)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for SOCKET_ADDRESS {
                    fn eq(&self, other: &Self) -> bool {
                        self.lpSockaddr == other.lpSockaddr
                            && self.iSockaddrLength == other.iSockaddrLength
                    }
                }
                impl ::std::cmp::Eq for SOCKET_ADDRESS {}
                unsafe impl ::windows::Abi for SOCKET_ADDRESS {
                    type Abi = Self;
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod System {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod SystemServices {
                #[repr(transparent)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct CHAR(pub u8);
                impl CHAR {}
                impl ::std::default::Default for CHAR {
                    fn default() -> Self {
                        Self(0)
                    }
                }
                impl CHAR {
                    pub const NULL: Self = Self(0);
                    pub fn is_null(&self) -> bool {
                        self.0 == 0
                    }
                }
                impl ::std::fmt::Debug for CHAR {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("CHAR").field("Value", &self.0).finish()
                    }
                }
                impl ::std::cmp::PartialEq for CHAR {
                    fn eq(&self, other: &Self) -> bool {
                        self.0 == other.0
                    }
                }
                impl ::std::cmp::Eq for CHAR {}
                unsafe impl ::windows::Abi for CHAR {
                    type Abi = Self;
                }
            }
        }
    }
}
