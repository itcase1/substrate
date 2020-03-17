use structopt::StructOpt;
use sc_cli::{RunCmd, Subcommand, RuntimeAdapter};
use node_template_runtime::{Runtime, SignedExtra, Index};
use sp_runtime::generic::Era;
use sp_core::sr25519;

#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[structopt(flatten)]
	pub run: RunCmd,
}

pub struct Adapter;

impl RuntimeAdapter for Adapter {
	type Pair = sr25519::Pair;
	type Public =  sr25519::Public;
	type Signature = sr25519::Signature;
	type Runtime = Runtime;
	type Extra = SignedExtra;

	fn build_extra(index: Index) -> Self::Extra {
		(
			frame_system::CheckVersion::new(),
			frame_system::CheckGenesis::new(),
			frame_system::CheckEra::from(Era::Immortal),
			frame_system::CheckNonce::from(index),
			frame_system::CheckWeight::new(),
			pallet_transaction_payment::ChargeTransactionPayment::from(0),
		)
	}
}
