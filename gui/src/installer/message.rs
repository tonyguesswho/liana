use liana::miniscript::{
    bitcoin::{util::bip32::Fingerprint, Network},
    DescriptorPublicKey,
};
use std::path::PathBuf;

use super::Error;
use crate::hw::HardwareWallet;

#[derive(Debug, Clone)]
pub enum Message {
    CreateWallet,
    ParticipateWallet,
    ImportWallet,
    UserActionDone(bool),
    Exit(PathBuf),
    Clibpboard(String),
    Next,
    Skip,
    Previous,
    Install,
    Close,
    Reload,
    Select(usize),
    UseHotSigner,
    Installed(Result<PathBuf, Error>),
    Network(Network),
    DefineBitcoind(DefineBitcoind),
    DefineDescriptor(DefineDescriptor),
    ImportXpub(usize, Result<DescriptorPublicKey, Error>),
    ConnectedHardwareWallets(Vec<HardwareWallet>),
    WalletRegistered(Result<(Fingerprint, Option<[u8; 32]>), Error>),
    MnemonicWord(usize, String),
    ImportMnemonic(bool),
}

#[derive(Debug, Clone)]
pub enum DefineBitcoind {
    CookiePathEdited(String),
    AddressEdited(String),
}

#[derive(Debug, Clone)]
pub enum DefineDescriptor {
    ImportDescriptor(String),
    /// AddKey(is_recovery)
    AddKey(bool),
    Key(bool, usize, DefineKey),
    HWXpubImported(Result<DescriptorPublicKey, Error>),
    XPubEdited(String),
    EditName,
    NameEdited(String),
    SequenceEdited(String),
    ThresholdEdited(bool, usize),
    ConfirmXpub,
}

#[derive(Debug, Clone)]
pub enum DefineKey {
    Delete,
    Edit,
    Clipboard(String),
    Edited(String, DescriptorPublicKey),
}
