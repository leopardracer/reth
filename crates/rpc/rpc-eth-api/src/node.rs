//! Helper trait for interfacing with [`FullNodeComponents`].

use reth_node_api::{FullNodeComponents, NodeTypes, PrimitivesTy};
use reth_payload_builder::PayloadBuilderHandle;
use reth_rpc_eth_types::EthStateCache;
use reth_storage_api::{BlockReader, ProviderBlock, ProviderReceipt};

/// Helper trait that provides the same interface as [`FullNodeComponents`] but without requiring
/// implementation of trait bounds.
///
/// This trait is structurally equivalent to [`FullNodeComponents`], exposing the same associated
/// types and methods. However, it doesn't enforce the trait bounds required by
/// [`FullNodeComponents`]. This makes it useful for RPC types that need access to node components
/// where the full trait bounds of the components are not necessary.
///
/// Every type that is a [`FullNodeComponents`] also implements this trait.
pub trait RpcNodeCore: Clone + Send + Sync {
    /// Blockchain data primitives.
    type Primitives: Send + Sync + Clone + Unpin;
    /// The provider type used to interact with the node.
    type Provider: Send + Sync + Clone + Unpin;
    /// The transaction pool of the node.
    type Pool: Send + Sync + Clone + Unpin;
    /// The node's EVM configuration, defining settings for the Ethereum Virtual Machine.
    type Evm: Send + Sync + Clone + Unpin;
    /// Network API.
    type Network: Send + Sync + Clone;

    /// Builds new blocks.
    type PayloadBuilder: Send + Sync + Clone;

    /// Returns the transaction pool of the node.
    fn pool(&self) -> &Self::Pool;

    /// Returns the node's evm config.
    fn evm_config(&self) -> &Self::Evm;

    /// Returns the handle to the network
    fn network(&self) -> &Self::Network;

    /// Returns the handle to the payload builder service.
    fn payload_builder(&self) -> &Self::PayloadBuilder;

    /// Returns the provider of the node.
    fn provider(&self) -> &Self::Provider;
}

impl<T> RpcNodeCore for T
where
    T: FullNodeComponents,
{
    type Primitives = PrimitivesTy<T::Types>;
    type Provider = T::Provider;
    type Pool = T::Pool;
    type Evm = T::Evm;
    type Network = T::Network;
    type PayloadBuilder = PayloadBuilderHandle<<T::Types as NodeTypes>::Payload>;

    #[inline]
    fn pool(&self) -> &Self::Pool {
        FullNodeComponents::pool(self)
    }

    #[inline]
    fn evm_config(&self) -> &Self::Evm {
        FullNodeComponents::evm_config(self)
    }

    #[inline]
    fn network(&self) -> &Self::Network {
        FullNodeComponents::network(self)
    }

    #[inline]
    fn payload_builder(&self) -> &Self::PayloadBuilder {
        FullNodeComponents::payload_builder_handle(self)
    }

    #[inline]
    fn provider(&self) -> &Self::Provider {
        FullNodeComponents::provider(self)
    }
}

/// Additional components, asides the core node components, needed to run `eth_` namespace API
/// server.
pub trait RpcNodeCoreExt: RpcNodeCore<Provider: BlockReader> {
    /// Returns handle to RPC cache service.
    fn cache(
        &self,
    ) -> &EthStateCache<ProviderBlock<Self::Provider>, ProviderReceipt<Self::Provider>>;
}
