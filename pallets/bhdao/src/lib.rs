#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//#[cfg(feature = "runtime-benchmarks")]
//mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*,traits::Currency,};
	use frame_system::pallet_prelude::*;
	use scale_info::{
		TypeInfo,
	};
	use sp_runtime::{ArithmeticError,traits::{CheckedAdd,One}};
	use sp_std::{
		vec,
		vec::Vec,
		collections::vec_deque::VecDeque,
	};

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

	type BalanceOf<T> = <T as pallet_nft::Config>::Balance;
	type TokenIdOf<T> = <T as pallet_nft::Config>::TokenId;

	#[pallet::type_value]
	pub fn ContributorTokenShare<T: Config>() -> BalanceOf<T>
	{
		90u128.try_into().ok().unwrap()
	}

	#[pallet::type_value]
	pub fn DAOTokenShare<T: Config>() -> BalanceOf<T>
	{
		10u128.try_into().ok().unwrap()
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Document<T:Config> {
		pub creator: T::AccountId,
		pub title: Vec<u8>,
		pub description: Vec<u8>,
		pub format: Vec<u8>,
		pub hash: Vec<u8>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Upload<T:Config> {
		pub creator: T::AccountId,
		pub hash: Vec<u8>,
		pub status: UploadStatus,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Vote<T:Config> {
		pub yes_votes: u64,
		pub no_votes: u64,
		pub start: T::BlockNumber,
		pub end: T::BlockNumber,
		pub status: VoteStatus,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Objection<T:Config> {
		pub objector: T::AccountId,
		pub hash: Vec<u8>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct ExpertReview<T:Config> {
		pub start: T::BlockNumber,
		pub end: T::BlockNumber,
		pub objections: Option<Vec<Objection<T>>>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Member<T:Config> {
		pub member_id: u32,
		pub metadata: Vec<u8>,
		pub vote_count: u64,
		pub approved_contributions: u32,
		pub role: Roles,
		pub joined: T::BlockNumber,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum VoteStatus {
		InProgress,
		Passed,
		Failed,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum UploadStatus {
		QualificationVoteInProgress,
		VerificationVoteInProgress,
		UnderExpertReview,
		SuccessfulExpertReview,
		CouncilVoteInProgress,
		Verified,
		Rejected,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum VoteType {
		Qualification,
		Verification,
		CuratorVerification,
		CuratorCouncilApproval,
		ExpertVerification,
		ExpertCouncilApproval,
		Proposal,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Roles {
		None = 0,
		Qualifier = 1,
		Contributor = 2,
		Verifier = 3,
		Expert = 4,
		Collector = 5,
	}

	#[pallet::pallet]
    #[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_nft::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		#[pallet::constant]
		type VotingWindow: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn get_key)]
	pub(super) type Key<T:Config> = StorageValue<_, T::AccountId,OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_total_items)]
	pub(super) type TotalItems<T> = StorageValue<_, u64,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_total_transactions)]
	pub(super) type TotalTransactions<T> = StorageValue<_, u64,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn members_uid_count)]
	pub(super) type MembersCount<T> = StorageValue<_, u32,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn upload_uid_count)]
	pub(super) type UploadCount<T> = StorageValue<_, u64,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn token_uid_count)]
	pub(super) type TokenCount<T> = StorageValue<_, TokenIdOf<T>,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn role_application_uid_count)]
	pub(super) type ApplicationCount<T> = StorageValue<_, u64,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_approved)]
	pub(super) type Approved<T> = StorageValue<_, Vec<TokenIdOf<T>>,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_contributor_share)]
	pub(super) type ContributorShare<T> = StorageValue<_, BalanceOf<T>,ValueQuery,ContributorTokenShare<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_dao_share)]
	pub(super) type DAOShare<T> = StorageValue<_, BalanceOf<T>,ValueQuery,DAOTokenShare<T>>;

	//#[pallet::storage]
	//#[pallet::getter(fn get_dao_wallet)]
	//pub(super) type DAOWalletAccount<T> = StorageValue<_, T::AccountId,OptionQuery,DAOWallet<T>>;


	#[pallet::storage]
	#[pallet::getter(fn get_member)]
	pub(super) type Members<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Member<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_upload)]
	pub(super) type Uploads<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64,
		Upload<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_expert_review)]
	pub(super) type ExpertReviews<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64,
		ExpertReview<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_role_application)]
	pub(super) type ExpertApplication<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64,
		T::AccountId,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_vote)]
	pub(super) type Votes<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		(VoteType,u64),
		Vote<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_member_vote)]
	pub(super) type CheckVote<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		(T::AccountId,VoteType,u64),
		bool,
		OptionQuery,
	>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MemberAdded{who: T::AccountId,uid: u32},
		NewUpload{uid: u64},
		NewVote{vote_type: VoteType, uid: u64},
		VoteCast{vote_type: VoteType, uid: u64},
		VoteEnded{vote_type: VoteType, uid: u64},
		ExpertReviewStarted{uid: u64},
		ExpertReviewEnded{uid: u64},
		ObjectionRaised{uid: u64, who: T::AccountId},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Member Already Exists
		MemberAlreadyExists,
		/// Not A Member
		NotAMember,
		/// Not Eligible To Contribute
		NotEligibleToContribute,
		// Not Eligible To Verify
		NotEligibleToVerify,
		/// Vote Not Found
		VoteNotFound,
		/// Vote Not In Progress
		VoteNotInProgress,
		/// Voting Window Not Valid
		VotingWindowNotValid,
		/// Vote Still In Progress
		VoteStillInProgress,
		/// Upload Not Found
		UploadNotFound,
		/// Not Eligible For Expert Role
		NotEligibleForExpertRole,
		/// Wrong Vote Type
		WrongVoteType,
		/// Not An Expert
		NotAnExpert,
		/// WrongRoleAssigned
		WrongRoleAssigned,
		/// Wrong Role Applied
		WrongRoleApplied,
		/// Not Eligible For Verifier Role
		NotEligibleForVerifierRole,
		/// NotUnderExpertReview
		NotUnderExpertReview,
		
	}

	
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3,2).ref_time())]
		pub fn join_dao(origin: OriginFor<T>, metadata: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(!Members::<T>::contains_key(&who.clone()), Error::<T>::MemberAlreadyExists);
			let uid = Self::members_uid_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;
			let now = <frame_system::Pallet<T>>::block_number();

			let member = Member::<T> {
				member_id: uid.clone(),
				metadata: metadata,
				vote_count: 0,
				approved_contributions: 0,
				role: Roles::Qualifier,
				joined: now,
			};

			Members::<T>::insert(who.clone(),&member);
			MembersCount::<T>::put(uid.clone());

			Self::deposit_event(Event::MemberAdded { who, uid });

			Ok(())

		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(5,3).ref_time())]
		pub fn upload_document(origin: OriginFor<T>, hash: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let member = Self::get_member(who.clone()).ok_or(Error::<T>::NotAMember)?;

			ensure!(member.role == Roles::Contributor, Error::<T>::NotEligibleToContribute);

			let uid = Self::upload_uid_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			let upload = Upload::<T> {
				creator: who.clone(),
				hash: hash,
				status: UploadStatus::QualificationVoteInProgress,
			};

			Uploads::<T>::insert(uid.clone(),upload);
			UploadCount::<T>::put(uid.clone());


			Self::deposit_event(Event::NewUpload { uid });

			let now = <frame_system::Pallet<T>>::block_number();

			let end = now + T::VotingWindow::get().into();

			let vote = Vote::<T> {
				yes_votes: 0,
				no_votes: 0,
				start: now,
				end: end,
				status: VoteStatus::InProgress,
			};

			let vote_type = VoteType::Qualification;

			Votes::<T>::insert((vote_type,uid.clone()),vote);

			Self::deposit_event(Event::NewVote { vote_type, uid});

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,3).ref_time())]
		pub fn cast_vote(origin: OriginFor<T>, vote_type: VoteType, voting_id: u64, vote_cast: bool) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// Check if member
			let mut member = Self::get_member(who.clone()).ok_or(Error::<T>::NotAMember)?;
			// Check Vote Type
			ensure!(vote_type == VoteType::Verification || vote_type == VoteType::Qualification, Error::<T>::WrongVoteType);
			// Check the role
			if vote_type == VoteType::Verification {
				ensure!(member.role == Roles::Verifier,Error::<T>::NotEligibleToVerify);
			}

			// Check if the vote exists
			let mut vote = Self::get_vote((vote_type.clone(),voting_id.clone())).ok_or(Error::<T>::VoteNotFound)?;
			ensure!(vote.status == VoteStatus::InProgress, Error::<T>::VoteNotInProgress);

			
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(now > vote.start && now < vote.end, Error::<T>::VotingWindowNotValid);

			if vote_cast {
				vote.yes_votes = vote.yes_votes + 1;
			} else {
				vote.no_votes = vote.no_votes + 1;
			}

			Votes::<T>::insert((vote_type.clone(),voting_id.clone()),vote);
			CheckVote::<T>::insert((who.clone(),vote_type.clone(),voting_id.clone()),true);

			let vote_count = member.vote_count.checked_add(1).ok_or(ArithmeticError::Overflow)?;
			
			if vote_count == 10 {
				member.role = Roles::Contributor;
			}

			member.vote_count = vote_count;

			Members::<T>::insert(who.clone(),&member);

			Self::deposit_event(Event::VoteCast { vote_type: vote_type, uid: voting_id});

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(6,4).ref_time())]
		pub fn finalize_vote(origin: OriginFor<T>, vote_type: VoteType, voting_id: u64) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// Check if member
			let member = Self::get_member(who.clone()).ok_or(Error::<T>::NotAMember)?;
			// Check if the vote exists
			let mut vote = Self::get_vote((vote_type.clone(),voting_id.clone())).ok_or(Error::<T>::VoteNotFound)?;
			ensure!(vote.status == VoteStatus::InProgress, Error::<T>::VoteNotInProgress);
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(now > vote.end,Error::<T>::VoteStillInProgress);
			// Check vote types
			

			// ToDo : Add Quorum

			// Votes
			//let total_votes = vote.yes_votes + vote.no_votes;

			match vote.yes_votes > vote.no_votes {
				true => {
					vote.status = VoteStatus::Passed;
					Votes::<T>::insert((vote_type.clone(),voting_id.clone()),vote);
					Self::deposit_event(Event::VoteEnded { vote_type: vote_type, uid: voting_id});
					if vote_type == VoteType::Qualification  {
						let mut upload = Self::get_upload(voting_id.clone()).ok_or(Error::<T>::UploadNotFound)?;
						let now = <frame_system::Pallet<T>>::block_number();

						let end = now + T::VotingWindow::get().into();

						let new_vote = Vote::<T> {
							yes_votes: 0,
							no_votes: 0,
							start: now,
							end: end,
							status: VoteStatus::InProgress,
						};

						let vote_type = VoteType::Verification;

						Votes::<T>::insert((vote_type.clone(),voting_id.clone()),new_vote);
						upload.status = UploadStatus::VerificationVoteInProgress;
						Uploads::<T>::insert(voting_id.clone(),&upload);

						Self::deposit_event(Event::NewVote { vote_type: vote_type,uid: voting_id});
					} else if vote_type == VoteType::Verification  {
						let mut upload = Self::get_upload(voting_id.clone()).ok_or(Error::<T>::UploadNotFound)?;
						//Changes
						//upload.status = UploadStatus::Verified;
						upload.status = UploadStatus::UnderExpertReview;
						Uploads::<T>::insert(voting_id.clone(),&upload);

						let now = <frame_system::Pallet<T>>::block_number();

						let end = now + T::VotingWindow::get().into();

						let expert_review = ExpertReview::<T> {
							start: now,
							end: end,
							objections: None,
						};

						ExpertReviews::<T>::insert(voting_id.clone(),&expert_review);
						Self::deposit_event(Event::ExpertReviewStarted { uid: voting_id});
						
					}
				},
				false => {
					vote.status = VoteStatus::Failed;
					Votes::<T>::insert((vote_type.clone(),voting_id.clone()),vote);
					Self::deposit_event(Event::VoteEnded { vote_type : vote_type, uid: voting_id});
					if vote_type == VoteType::Qualification || vote_type == VoteType::Verification  {
						let mut upload = Self::get_upload(voting_id.clone()).ok_or(Error::<T>::UploadNotFound)?;
						upload.status = UploadStatus::Rejected;
						Uploads::<T>::insert(voting_id.clone(),&upload);
					} 
				}
			}

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn apply_for_expanded_role(origin: OriginFor<T>,applied_role: Roles) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// Check roles applied
			ensure!(applied_role == Roles::Expert || applied_role == Roles::Verifier,Error::<T>::WrongRoleApplied);
			// Check if member
			let member = Self::get_member(who.clone()).ok_or(Error::<T>::NotAMember)?;
			let mut vote_type = VoteType::ExpertVerification;

			if applied_role == Roles::Expert {
				ensure!(member.role == Roles::Verifier,Error::<T>::NotEligibleForExpertRole);
			}

			if applied_role == Roles::Verifier {
				ensure!(member.role == Roles::Contributor,Error::<T>::NotEligibleForVerifierRole);
				vote_type = VoteType::CuratorVerification;
			}
			

			let uid = Self::role_application_uid_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			let now = <frame_system::Pallet<T>>::block_number();

			let end = now + T::VotingWindow::get().into();

			let vote = Vote::<T> {
				yes_votes: 0,
				no_votes: 0,
				start: now,
				end: end,
				status: VoteStatus::InProgress,
			};


			Votes::<T>::insert((vote_type,uid.clone()),vote);
			ExpertApplication::<T>::insert(uid.clone(),who.clone());
			ApplicationCount::<T>::put(uid.clone());

			Self::deposit_event(Event::NewVote { vote_type, uid});

			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cast_vote_for_expanded_role(origin: OriginFor<T>, vote_type: VoteType, voting_id: u64, vote_cast: bool) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// Check if member
			let mut member = Self::get_member(who.clone()).ok_or(Error::<T>::NotAMember)?;
			// Check Vote Type
			ensure!(vote_type == VoteType::ExpertVerification || vote_type == VoteType::ExpertCouncilApproval ||
				 vote_type == VoteType::CuratorVerification || vote_type == VoteType::CuratorCouncilApproval, Error::<T>::WrongVoteType);

			// Check if the vote exists
			let mut vote = Self::get_vote((vote_type.clone(),voting_id.clone())).ok_or(Error::<T>::VoteNotFound)?;
			ensure!(vote.status == VoteStatus::InProgress, Error::<T>::VoteNotInProgress);
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(now > vote.start && now < vote.end, Error::<T>::VotingWindowNotValid);

			// Check the role
			if vote_type == VoteType::ExpertVerification || vote_type == VoteType::CuratorVerification {
				ensure!(member.role == Roles::Verifier,Error::<T>::NotEligibleToVerify);
			}

			if vote_type == VoteType::ExpertCouncilApproval || vote_type == VoteType::CuratorCouncilApproval {
				ensure!(member.role == Roles::Expert,Error::<T>::NotAnExpert);
			}

			// Check if the vote exists
			let mut vote = Self::get_vote((vote_type.clone(),voting_id.clone())).ok_or(Error::<T>::VoteNotFound)?;
			ensure!(vote.status == VoteStatus::InProgress, Error::<T>::VoteNotInProgress);

			
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(now > vote.start && now < vote.end, Error::<T>::VotingWindowNotValid);

			if vote_cast {
				vote.yes_votes = vote.yes_votes + 1;
			} else {
				vote.no_votes = vote.no_votes + 1;
			}

			Votes::<T>::insert((vote_type.clone(),voting_id.clone()),vote);
			CheckVote::<T>::insert((who.clone(),vote_type.clone(),voting_id.clone()),true);

			let vote_count = member.vote_count.checked_add(1).ok_or(ArithmeticError::Overflow)?;
			
			member.vote_count = vote_count;

			Members::<T>::insert(who.clone(),&member);

			Self::deposit_event(Event::VoteCast { vote_type: vote_type, uid: voting_id});

			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn finalize_vote_for_expanded_role(origin: OriginFor<T>, vote_type: VoteType, voting_id: u64) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// Check if member
			let mut member = Self::get_member(who.clone()).ok_or(Error::<T>::NotAMember)?;
			// Check if the vote exists
			let mut vote = Self::get_vote((vote_type.clone(),voting_id.clone())).ok_or(Error::<T>::VoteNotFound)?;
			ensure!(vote.status == VoteStatus::InProgress, Error::<T>::VoteNotInProgress);
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(now > vote.end,Error::<T>::VoteStillInProgress);

			match vote.yes_votes > vote.no_votes { 
				true => {
					vote.status = VoteStatus::Passed;
					Votes::<T>::insert((vote_type.clone(),voting_id.clone()),vote);
					Self::deposit_event(Event::VoteEnded { vote_type : vote_type, uid: voting_id});

					if vote_type == VoteType::ExpertVerification  {
						let now = <frame_system::Pallet<T>>::block_number();

						let end = now + T::VotingWindow::get().into();

						let new_vote = Vote::<T> {
							yes_votes: 0,
							no_votes: 0,
							start: now,
							end: end,
							status: VoteStatus::InProgress,
						};

						let vote_type = VoteType::ExpertCouncilApproval;

						Votes::<T>::insert((vote_type.clone(),voting_id.clone()),new_vote);

						Self::deposit_event(Event::NewVote { vote_type: vote_type,uid: voting_id});

					} else if vote_type == VoteType::ExpertCouncilApproval {
						member.role = Roles::Expert;
						Members::<T>::insert(who.clone(),&member);
					} else if vote_type == VoteType::CuratorVerification  {
						let now = <frame_system::Pallet<T>>::block_number();

						let end = now + T::VotingWindow::get().into();

						let new_vote = Vote::<T> {
							yes_votes: 0,
							no_votes: 0,
							start: now,
							end: end,
							status: VoteStatus::InProgress,
						};

						let vote_type = VoteType::CuratorCouncilApproval;

						Votes::<T>::insert((vote_type.clone(),voting_id.clone()),new_vote);

						Self::deposit_event(Event::NewVote { vote_type: vote_type,uid: voting_id});
					} else if vote_type == VoteType::CuratorCouncilApproval {
						member.role = Roles::Verifier;
						Members::<T>::insert(who.clone(),&member);
					}

				},
				false => {
					vote.status = VoteStatus::Failed;
					Votes::<T>::insert((vote_type.clone(),voting_id.clone()),vote);
					Self::deposit_event(Event::VoteEnded { vote_type : vote_type, uid: voting_id});
				}
			}

			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn set_membership(origin: OriginFor<T>, new_member: T::AccountId, member_role: u8, metadata: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(!Members::<T>::contains_key(&new_member.clone()), Error::<T>::MemberAlreadyExists);

			let role = match member_role {
				1 => Roles::Qualifier,
				2 => Roles::Contributor,
				3 => Roles::Verifier,
				4 => Roles::Expert,
				5 => Roles::Collector,
				_ => Roles::None,
			};

			let uid = Self::members_uid_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;
			let now = <frame_system::Pallet<T>>::block_number();

			let member = Member::<T> {
				member_id: uid.clone(),
				metadata: metadata,
				vote_count: 0,
				approved_contributions: 0,
				role: role,
				joined: now,
			};

			Members::<T>::insert(new_member.clone(),&member);
			MembersCount::<T>::put(uid.clone());

			Self::deposit_event(Event::MemberAdded { who: new_member, uid });

			Ok(())
		}

		#[pallet::call_index(8)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn raise_expert_objection(origin: OriginFor<T>, upload_id: u64, reason: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// Check if member
			let mut member = Self::get_member(who.clone()).ok_or(Error::<T>::NotAMember)?;
			ensure!(member.role == Roles::Expert,Error::<T>::NotAnExpert);

			let upload = Self::get_upload(upload_id.clone()).ok_or(Error::<T>::UploadNotFound)?;

			ensure!(upload.status == UploadStatus::UnderExpertReview,Error::<T>::NotUnderExpertReview);

			let objection = Objection::<T> {
				objector: who.clone(),
				hash: reason,
			};

			let mut expert_review = Self::get_expert_review(upload_id.clone()).ok_or(Error::<T>::NotUnderExpertReview)?;

			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(now > expert_review.start && now < expert_review.end, Error::<T>::VotingWindowNotValid);

			let mut object1 = expert_review.objections.clone();

			match object1 {
				Some(mut object1) => {
					object1.push(objection);
					expert_review.objections = Some(object1);
				} ,
				None => {
					let mut obj1 = Vec::new();
					obj1.push(objection);
					expert_review.objections = Some(obj1);
				},
			}

			ExpertReviews::<T>::insert(upload_id.clone(),expert_review);
			Self::deposit_event(Event::ObjectionRaised {  uid: upload_id, who:  who});

			Ok(())
		}

		#[pallet::call_index(9)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn finalize_expert_review(origin: OriginFor<T>, upload_id: u64) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			// Check if member
			let member = Self::get_member(who.clone()).ok_or(Error::<T>::NotAMember)?;
			// Check if the vote exists
			let mut expert_review = Self::get_expert_review(upload_id.clone()).ok_or(Error::<T>::NotUnderExpertReview)?;
			let mut upload = Self::get_upload(upload_id.clone()).ok_or(Error::<T>::UploadNotFound)?;
			
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(now > expert_review.end,Error::<T>::VoteStillInProgress);

			let object1 = expert_review.objections;

			match object1 {
				Some(object1) => {
					upload.status = UploadStatus::Rejected;
					Uploads::<T>::insert(upload_id.clone(),&upload);
				} ,
				None => {
					upload.status = UploadStatus::Verified;
					Uploads::<T>::insert(upload_id.clone(),&upload);
					// Move to finalize_expert_review
					let exist = Approved::<T>::exists();
					let token_id = Self::token_uid_count().checked_add(&One::one()).ok_or(ArithmeticError::Overflow)?;
					let tuid: TokenIdOf<T> = token_id.try_into().ok().unwrap();
					let share1 = Self::get_contributor_share();
					let share2 = Self::get_dao_share();

					pallet_nft::Pallet::<T>::mint_batch(origin.clone(),vec![upload.creator,who.clone()],tuid,vec![share1,share2],upload.hash).ok();

					match exist {
						true => {
							let mut temp = Approved::<T>::get();
							temp.push(tuid);
							Approved::<T>::put(temp);
						},
						false => {
							let mut temp: Vec<TokenIdOf<T>> = Vec::new();
							temp.push(tuid);
							Approved::<T>::put(temp);
						}
					};
		
				},
			};

			Ok(())
		}

		
	}

	// Helpful functions
	impl<T: Config> Pallet<T> {
		
	}
}
