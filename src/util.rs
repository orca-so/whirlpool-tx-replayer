use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use poc_framework::LocalEnvironmentBuilder;
use solana_program::bpf_loader_upgradeable;
use bincode;

// LocalEnvironmentBuilder.add_program doesn't work for upgradeable programs
// https://github.com/solana-labs/solana/blob/170478924705c9c62dbeb475c5425b68ba61b375/sdk/program/src/bpf_loader_upgradeable.rs#L27-L53
pub fn add_upgradable_program(
    builder: &mut LocalEnvironmentBuilder,
    pubkey: Pubkey,
    data: &[u8],
) {
    let program_pubkey = pubkey;
    let programdata_pubkey = Keypair::new().pubkey();

    let program_data = bpf_loader_upgradeable::UpgradeableLoaderState::Program {
      programdata_address: programdata_pubkey
    };

    let programdata_header = bpf_loader_upgradeable::UpgradeableLoaderState::ProgramData {
      slot: 1, // 0 is not valid
      upgrade_authority_address: Some(Pubkey::default()), // None is not valid
    };

    let program_bytes = bincode::serialize(&program_data).unwrap();
    let mut programdata_bytes = bincode::serialize(&programdata_header).unwrap();
    programdata_bytes.extend_from_slice(data);

    builder.add_account_with_data(program_pubkey, bpf_loader_upgradeable::ID, &program_bytes, true);
    builder.add_account_with_data(programdata_pubkey, bpf_loader_upgradeable::ID, &programdata_bytes, false);
}
