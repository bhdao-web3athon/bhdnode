# Black History DAO Substrate Node

## BlockChain Overview

Black History DAO aims to collect, preserve and share the real stories of Black history and anchoring them on the blockchain. The documents and stories are meant to be verified by the Black History DAO community and appointed by vote experts. Once approved, the data will be stored as a structure on the blockchain and IPFS.

### DAO Membership Roles

DAO Membership is multi-tiered

```
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
```

Qualifier : General Members who vote on documents and proposals.
Contributor : Tier-2 members who contribute documents to the DAO that are evaluated by the full membership.
Verifier : Tier-3 long-standing members of DAO who perform verification after they are voted on by general membership.
Expert : Experts from different field who make sure that the documents are authentic, legal etc. They can vote down the documents if they are not upto the standard.
Collector : Institutions and collectors.

Uploaded Document Struct

```
#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
#[scale_info(skip_type_params(T))]
	pub struct Upload<T:Config> {
		pub creator: T::AccountId,
		pub hash: Vec<u8>,
		pub status: UploadStatus,
	}

```

Document Upload

```
pub fn upload_document(origin: OriginFor<T>, hash: Vec<u8>) -> DispatchResult
```

Membership Management and Governance Functions

```
pub fn join_dao(origin: OriginFor<T>, metadata: Vec<u8>) -> DispatchResult
```

```
pub fn cast_vote(origin: OriginFor<T>, vote_type: VoteType, voting_id: u64, vote_cast: bool) 
```

```
pub fn finalize_vote(origin: OriginFor<T>, vote_type: VoteType, voting_id: u64) -> DispatchResult
```

```
pub fn apply_for_expanded_role(origin: OriginFor<T>) -> DispatchResult
```

```
pub fn cast_vote_for_expanded_role(origin: OriginFor<T>, vote_type: VoteType, voting_id: u64, vote_cast: bool) -> DispatchResult
```

```
pub fn finalize_vote_for_expanded_role(origin: OriginFor<T>, vote_type: VoteType, voting_id: u64) -> DispatchResult
```

```
pub fn raise_expert_objection(origin: OriginFor<T>, upload_id: u64, reason: Vec<u8>) -> DispatchResult
```

```
pub fn finalize_expert_review(origin: OriginFor<T>, upload_id: u64) -> DispatchResult
```

## Local Build and Testing

### Install Rust Environment

```
curl https://getsubstrate.io/ -sSf | bash -s - --fast
```

### Clone The Repository

```
```

### Build the node and run in dev mode

```
$ cargo build --release
$ ./target/release/node-template --dev --tmp

```

### Run Tests

```
cargo test
```
