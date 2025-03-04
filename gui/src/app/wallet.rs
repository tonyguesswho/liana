use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::{
    app::{config::Config, settings},
    hw::HardwareWalletConfig,
    signer::Signer,
};

use liana::{miniscript::bitcoin, signer::HotSigner};

use liana::descriptors::MultipathDescriptor;
use liana::miniscript::bitcoin::util::bip32::Fingerprint;

pub const DEFAULT_WALLET_NAME: &str = "Liana";

#[derive(Debug)]
pub struct Wallet {
    pub name: String,
    pub main_descriptor: MultipathDescriptor,
    pub keys_aliases: HashMap<Fingerprint, String>,
    pub hardware_wallets: Vec<HardwareWalletConfig>,
    pub signer: Option<Signer>,
}

impl Wallet {
    pub fn new(main_descriptor: MultipathDescriptor) -> Self {
        Self {
            name: DEFAULT_WALLET_NAME.to_string(),
            main_descriptor,
            keys_aliases: HashMap::new(),
            hardware_wallets: Vec::new(),
            signer: None,
        }
    }

    pub fn with_key_aliases(mut self, aliases: HashMap<Fingerprint, String>) -> Self {
        self.keys_aliases = aliases;
        self
    }

    pub fn with_hardware_wallets(mut self, hardware_wallets: Vec<HardwareWalletConfig>) -> Self {
        self.hardware_wallets = hardware_wallets;
        self
    }

    pub fn with_signer(mut self, signer: Signer) -> Self {
        self.signer = Some(signer);
        self
    }

    pub fn descriptor_keys(&self) -> HashSet<Fingerprint> {
        let info = self.main_descriptor.info();
        let mut descriptor_keys = HashSet::new();
        for (fingerprint, _) in info.primary_path().thresh_origins().1.iter() {
            descriptor_keys.insert(*fingerprint);
        }
        for (fingerprint, _) in info.recovery_path().1.thresh_origins().1.iter() {
            descriptor_keys.insert(*fingerprint);
        }
        descriptor_keys
    }

    pub fn descriptor_checksum(&self) -> String {
        self.main_descriptor
            .to_string()
            .split_once('#')
            .map(|(_, checksum)| checksum)
            .unwrap()
            .to_string()
    }

    pub fn load_settings(
        self,
        gui_config: &Config,
        datadir_path: &Path,
        network: bitcoin::Network,
    ) -> Result<Self, WalletError> {
        let gui_config_hws = gui_config
            .hardware_wallets
            .as_ref()
            .cloned()
            .unwrap_or_default();

        let mut wallet = match settings::Settings::from_file(datadir_path.to_path_buf(), network) {
            Ok(settings) => {
                if let Some(wallet_setting) = settings.wallets.first() {
                    self.with_hardware_wallets(wallet_setting.hardware_wallets.clone())
                        .with_key_aliases(wallet_setting.keys_aliases())
                } else {
                    self.with_hardware_wallets(gui_config_hws)
                }
            }
            Err(settings::SettingsError::NotFound) => self.with_hardware_wallets(gui_config_hws),
            Err(e) => return Err(e.into()),
        };

        let hot_signers = match HotSigner::from_datadir(datadir_path, network) {
            Ok(signers) => signers,
            Err(e) => match e {
                liana::signer::SignerError::MnemonicStorage(e) => {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        Vec::new()
                    } else {
                        return Err(WalletError::HotSigner(e.to_string()));
                    }
                }
                _ => return Err(WalletError::HotSigner(e.to_string())),
            },
        };

        let curve = bitcoin::secp256k1::Secp256k1::signing_only();
        let keys = wallet.descriptor_keys();
        if let Some(hot_signer) = hot_signers
            .into_iter()
            .find(|s| keys.contains(&s.fingerprint(&curve)))
        {
            wallet = wallet.with_signer(Signer::new(hot_signer));
        }

        Ok(wallet)
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum WalletError {
    Settings(settings::SettingsError),
    HotSigner(String),
}

impl std::fmt::Display for WalletError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Settings(e) => write!(f, "Failed to load settings: {}", e),
            Self::HotSigner(e) => write!(f, "Failed to load hot signer: {}", e),
        }
    }
}

impl From<settings::SettingsError> for WalletError {
    fn from(error: settings::SettingsError) -> Self {
        WalletError::Settings(error)
    }
}
