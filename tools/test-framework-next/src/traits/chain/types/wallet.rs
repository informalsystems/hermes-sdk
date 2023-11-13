use cgp_core::Async;

pub trait HasWalletType: Async {
    type Wallet: Async;
}
