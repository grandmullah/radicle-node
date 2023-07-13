let
  mozillaOverlay =
    import (builtins.fetchGit {
      url = "https://github.com/mozilla/nixpkgs-mozilla.git";
      rev = "57c8084c7ef41366993909c20491e359bbb90f54";
    });
  pinned = builtins.fetchGit {
    # Descriptive name to make the store path easier to identify
    url = "https://github.com/nixos/nixpkgs/";
    # Commit hash for nixos-unstable as of 2020-04-26
    # `git ls-remote https://github.com/nixos/nixpkgs nixos-unstable`
    ref = "refs/heads/nixos-unstable";
    rev = "1fe6ed37fd9beb92afe90671c0c2a662a03463dd";
  };
  nixpkgs = import pinned { overlays = [ mozillaOverlay ]; };
  toolchain = with nixpkgs; (rustChannelOf { date = "2021-09-14"; channel = "nightly"; });
  rust-wasm = toolchain.rust.override {
    targets = [ "wasm32-unknown-unknown" ];
  };
in
with nixpkgs; pkgs.mkShell {
  buildInputs = [
    clang
    pkg-config
    rust-wasm
  ] ++ stdenv.lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
  ];

  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
  PROTOC = "${protobuf}/bin/protoc";
  RUST_SRC_PATH = "${toolchain.rust-src}/lib/rustlib/src/rust/library/";
  ROCKSDB_LIB_DIR = "${rocksdb}/lib";

}

./target/release/node-template key insert --base-path /tmp/node1 \
  --chain customSpecRaw.json \
  --scheme Ed25519 \
  --suri "cloth end journey alert drill echo cheap agent ethics current liberty way" \
  --key-type gran
  ENTRYPOINT ["/usr/src/app/target/release/node-template",  "--chain", "./customSpecRaw.json", "--base-path", "/tmp/node1", "--keystore-path", "/tmp/node1/chains/radicle_testnet/keystore"  "--port", "30333", "--ws-port", "9944" ,"--unsafe-ws-external", "--rpc-port","9933","--unsafe-rpc-external","--rpc-cors","all", "--validator","--name","node1" ]


# #[pallet::call_index(0)]
# 		#[pallet::weight(0)]
# 		pub fn add_usr(
# 			origin: OriginFor<T>,
# 			phone_number:PhoneNumber<MaxIdLengthBytes>,
# 			name:Name<MaxIdLengthBytes>
			
# 		) -> DispatchResultWithPostInfo {
# 			let signer = ensure_signed(origin)?;
# 			let verified:bool = false;
# 			let rating:u32 = 0;
# 			let role  =  String::from("rider");
# 			let usr_to_store = User::<T> {
# 				name,
# 				verified,
# 				phone_number,
# 				rating
# 			};

# 			<Identity<T>>::insert(&signer,usr_to_store);
# 			// make a offchain call 
# 			Self::deposit_event(Event::<T>::AccountCreated {who:signer,role});
# 			Ok(Pays::No.into())
# 		}

# 		#[pallet::call_index(1)]
# 		#[pallet::weight(0)]
# 		pub fn add_cab(
# 			origin : OriginFor<T>,
# 			plate:Name<T>,
# 			manufacture_year:u16,
# 			model:Name<T>,
# 			driver:T::AccountId,
# 		) -> DispatchResultWithPostInfo{
# 			let owner = ensure_signed(origin)?;
# 			let count = Self::cab_count();
# 			let g = owner.clone();

# 			// let mut  cab = Self::get_cab(count).ok_or(Error::<T>::InvalidIndex)?;
# 			<CabDetails::<T>>::insert(count+1,Cab{
# 				plate,
# 				manufacture_year,
# 				model,
# 				owner,
# 				driver,
# 				verified:false,
# 				rating:Zero::zero(),
# 			});
		
# 			<CabCount<T>>::put(count+1);
# 			Self::deposit_event(Event::<T>::CabAdded{who:g,count:count+1});
# 			Ok(Pays::No.into())
# 		}


# 		#[pallet::call_index(2)]
# 		#[pallet::weight(0)]
# 		pub fn add_driver(
# 			origin: OriginFor<T>,
# 			phone_number:PhoneNumber<T>,
# 			name:Name<T>,
# 			cab:u32
# 		) -> DispatchResultWithPostInfo {
# 			let account = ensure_signed(origin)?;
# 			let verified:bool = false;
# 			let rating:u32 = 0;
			
# 			let role  =  String::from("rider");
# 			<Drivers<T>>::insert(&account,Driver {
# 				name,
# 				verified,
# 				phone_number,
# 				cab,
# 				rating,
# 			});
# 			// make a offchain call 
# 			Self::deposit_event(Event::<T>::AccountCreated {who:account,role});
# 			Ok(Pays::No.into())
# 		}

# 		#[pallet::call_index(3)]
# 		#[pallet::weight(0)]
# 		pub fn assign_driver(
# 			origin: OriginFor<T>,
# 			cab:u32,
# 			driver:T::AccountId
# 		) -> DispatchResultWithPostInfo {
# 			let account = ensure_signed(origin)?;

# 			let mut cab_details = Self::get_cab(cab).ok_or(Error::<T>::InvalidIndex)?;
# 			let mut driver_details = Self::get_driver(&driver).ok_or(Error::<T>::InvalidIndex)?;
# 			// cab.owner
# 			ensure!(account == cab_details.owner,<Error<T>>::InvalidOwner);
# 			let d = &driver.clone();
# 			cab_details.driver  = driver;
# 			driver_details.cab = cab;

	

# 			<Drivers<T>>::insert(&d,driver_details);
# 			<CabDetails::<T>>::insert(cab,cab_details);


# 			Ok(Pays::No.into())
# 			// emmit  transer driver 
# 		}