//! Constants for the Avalanche network.
use std::collections::HashMap;

use lazy_static::lazy_static;

pub const DEFAULT_CUSTOM_NETWORK_ID: u32 = 1000000;

// name

pub const MAINNET_NAME: &str = "mainnet";
pub const FUJI_NAME: &str = "fuji";
pub const LOCAL_NAME: &str = "local";

pub const CAMINO_NAME: &str = "camino";
pub const COLUMBUS_NAME: &str = "columbus";
pub const KOPERNIKUS_NAME: &str = "kopernikus";

// hrp

pub const MAINNET_HRP: &str = "avax";
pub const FUJI_HRP: &str = "fuji";
pub const LOCAL_HRP: &str = "local";

pub const CAMINO_HRP: &str = "camino";
pub const COLUMBUS_HRP: &str = "columbus";
pub const KOPERNIKUS_HRP: &str = "kopernikus";

pub const FALLBACK_HRP: &str = "custom";

// networkID

pub const MAINNET_NETWORK_ID: u32 = 1;
pub const FUJI_NETWORK_ID: u32 = 5;
pub const LOCAL_NETWORK_ID: u32 = 12345;

pub const CAMINO_NETWORK_ID: u32 = 1000;
pub const COLUMBUS_NETWORK_ID: u32 = 1001;
pub const KOPERNIKUS_NETWORK_ID: u32 = 1002;

lazy_static! {
    /// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/utils/constants>
    pub static ref NETWORK_ID_TO_NETWORK_NAME: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();

        m.insert(MAINNET_NETWORK_ID, MAINNET_NAME);
        m.insert(FUJI_NETWORK_ID, FUJI_NAME);
        m.insert(LOCAL_NETWORK_ID, LOCAL_NAME);

        m.insert(CAMINO_NETWORK_ID, CAMINO_NAME);
        m.insert(COLUMBUS_NETWORK_ID, COLUMBUS_NAME);
        m.insert(KOPERNIKUS_NETWORK_ID, KOPERNIKUS_NAME);

        m
    };

    /// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/utils/constants>
    pub static ref NETWORK_NAME_TO_NETWORK_ID: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();

        m.insert(MAINNET_NAME, MAINNET_NETWORK_ID);
        m.insert(FUJI_NAME, FUJI_NETWORK_ID);
        m.insert(LOCAL_NAME, LOCAL_NETWORK_ID);

        m.insert(CAMINO_NAME, CAMINO_NETWORK_ID);
        m.insert(COLUMBUS_NAME, COLUMBUS_NETWORK_ID);
        m.insert(KOPERNIKUS_NAME, KOPERNIKUS_NETWORK_ID);

        m
    };

    /// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/utils/constants>
    pub static ref NETWORK_ID_TO_HRP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();

        m.insert(MAINNET_NETWORK_ID, MAINNET_HRP);
        m.insert(FUJI_NETWORK_ID, FUJI_HRP);
        m.insert(LOCAL_NETWORK_ID, LOCAL_HRP);

        m.insert(CAMINO_NETWORK_ID, CAMINO_HRP);
        m.insert(COLUMBUS_NETWORK_ID, COLUMBUS_HRP);
        m.insert(KOPERNIKUS_NETWORK_ID, KOPERNIKUS_HRP);

        m
    };

    /// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/utils/constants>
    pub static ref HRP_TO_NETWORK_ID: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();

        m.insert(MAINNET_HRP, MAINNET_NETWORK_ID);
        m.insert(FUJI_HRP, FUJI_NETWORK_ID);
        m.insert(LOCAL_HRP, LOCAL_NETWORK_ID);

        m.insert(CAMINO_HRP, CAMINO_NETWORK_ID);
        m.insert(COLUMBUS_HRP, COLUMBUS_NETWORK_ID);
        m.insert(KOPERNIKUS_HRP, KOPERNIKUS_NETWORK_ID);

        m
    };
}
