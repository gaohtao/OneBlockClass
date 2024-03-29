//! Substrate Proof-of-Existence Pallet
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, storage::bounded_vec::BoundedVec};
    use frame_system::pallet_prelude::*;

    // Define the pallet struct placeholder, various pallet functions are implemented on it.
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // TODO: Update the `Config` block below
    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// For constraining the maximum bytes of a hash used for any proof
        type MaxBytesInHash: Get<u32>;
    }

    // TODO: Update the `event` block below
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
       
	    /// Event emitted when a proof has been claimed. [who, claim]
        ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
        /// Event emitted when a claim is revoked by the owner. [who, claim]
        ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
        /// Event emitted when a claim is transfered by the owner to receiver. 
        ClaimTransfered(T::AccountId, T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),

    }

    #[pallet::error]
    pub enum Error<T> {
        /// The proof has already been claimed.
        ProofAlreadyClaimed,
        /// The proof does not exist, so it cannot be revoked.
        NoSuchProof,
        /// The proof is claimed by another account, so caller can't revoke it.
        NotProofOwner,
    }

    #[pallet::storage]
    /// Maps each proof to its owner and block number when the proof was made
    pub(super) type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxBytesInHash>,
        (T::AccountId, T::BlockNumber),
        OptionQuery,
    >;

    // Dispatchable functions allow users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_000)]
        pub fn create_claim(
            origin: OriginFor<T>,
            proof: BoundedVec<u8, T::MaxBytesInHash>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://docs.substrate.io/v3/runtime/origins
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has not already been claimed.
            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            // Get the block number from the FRAME System pallet.
            let current_block = <frame_system::Pallet<T>>::block_number();

            // Store the proof with the sender and block number.
            Proofs::<T>::insert(&proof, (&sender, current_block));

            // Emit an event that the claim was created.
            Self::deposit_event(Event::ClaimCreated(sender, proof));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            proof: BoundedVec<u8, T::MaxBytesInHash>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://docs.substrate.io/v3/runtime/origins
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has been claimed.
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            // Get owner of the claim.
            // Panic condition: there is no way to set a `None` owner, so this must always unwrap.
            let (owner, _) = Proofs::<T>::get(&proof).expect("All proofs must have an owner!");

            // Verify that sender of the current call is the claim owner.
            ensure!(sender == owner, Error::<T>::NotProofOwner);

            // Remove claim from storage.
            Proofs::<T>::remove(&proof);

            // Emit an event that the claim was erased.
            Self::deposit_event(Event::ClaimRevoked(sender, proof));
            Ok(())
        }

        //转移存证功能， 把存证转移给另一个账户. 接收参数3个：所有者账户，内容哈希值，接收者账户
        #[pallet::weight(1_000)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            receiver: T::AccountId,
            proof: BoundedVec<u8, T::MaxBytesInHash>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://docs.substrate.io/v3/runtime/origins
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has been claimed.
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            // Get the block number from the FRAME System pallet.
            let current_block = <frame_system::Pallet<T>>::block_number();

            // Store the proof with the new receiver and block number.
            //Proofs::<T>::insert(&proof, (&receiver, current_block));
	   Proofs::<T>::mutate(&proof, |value| {
                value.as_mut().unwrap().0 = receiver.clone();
                value.as_mut().unwrap().1 = current_block;
            });

            // Emit an event that the claim was created.
            Self::deposit_event(Event::ClaimTransfered(sender, receiver,proof));

            Ok(())
        }

    }
}
