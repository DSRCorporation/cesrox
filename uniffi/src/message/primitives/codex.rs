pub use cesrox::matter::Codex as CesroxMatterCodex;
pub use cesrox::indexer::Codex as CesroxIndexerCodex;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum MatterCodex {
    Ed25519Seed,
    Ed25519N,
    X25519,
    Ed25519,
    Blake3_256,
    Blake2b_256,
    Blake2s_256,
    SHA3_256,
    SHA2_256,
    ECDSA_256k1_Seed,
    Ed448_Seed,
    X448,
    Short,
    Big,
    X25519_Private,
    X25519_Cipher_Seed,
    Salt_128,
    Ed25519_Sig,
    ECDSA_256k1_Sig,
    Blake3_512,
    Blake2b_512,
    SHA3_512,
    SHA2_512,
    Long,
    ECDSA_256k1N,
    ECDSA_256k1,
    Ed448N,
    Ed448,
    Ed448_Sig,
    Tern,
    DateTime,
    X25519_Cipher_Salt,
    TBD1,
    TBD2,
    StrB64_L0,
    StrB64_L1,
    StrB64_L2,
    StrB64_Big_L0,
    StrB64_Big_L1,
    StrB64_Big_L2,
    Bytes_L0,
    Bytes_L1,
    Bytes_L2,
    Bytes_Big_L0,
    Bytes_Big_L1,
    Bytes_Big_L2
}

impl MatterCodex {
    pub fn code(&self) -> &str {
        match self {
            MatterCodex::Ed25519Seed => CesroxMatterCodex::Ed25519_Seed,
            MatterCodex::Ed25519N => CesroxMatterCodex::Ed25519N,
            MatterCodex::X25519 => CesroxMatterCodex::X25519,
            MatterCodex::Ed25519 => CesroxMatterCodex::Ed25519,
            MatterCodex::Blake3_256 => CesroxMatterCodex::Blake3_256,
            MatterCodex::Blake2b_256 => CesroxMatterCodex::Blake2b_256,
            MatterCodex::Blake2s_256 => CesroxMatterCodex::Blake2s_256,
            MatterCodex::SHA3_256 => CesroxMatterCodex::SHA3_256,
            MatterCodex::SHA2_256 => CesroxMatterCodex::SHA2_256,
            MatterCodex::ECDSA_256k1_Seed => CesroxMatterCodex::ECDSA_256k1_Seed,
            MatterCodex::Ed448_Seed => CesroxMatterCodex::Ed448_Seed,
            MatterCodex::X448 => CesroxMatterCodex::X448,
            MatterCodex::Short => CesroxMatterCodex::Short,
            MatterCodex::Big => CesroxMatterCodex::Big,
            MatterCodex::X25519_Private => CesroxMatterCodex::X25519_Private,
            MatterCodex::X25519_Cipher_Seed => CesroxMatterCodex::X25519_Cipher_Seed,
            MatterCodex::Salt_128 => CesroxMatterCodex::Salt_128,
            MatterCodex::Ed25519_Sig => CesroxMatterCodex::Ed25519_Sig,
            MatterCodex::ECDSA_256k1_Sig => CesroxMatterCodex::ECDSA_256k1_Sig,
            MatterCodex::Blake3_512 => CesroxMatterCodex::Blake3_512,
            MatterCodex::Blake2b_512 => CesroxMatterCodex::Blake2b_512,
            MatterCodex::SHA3_512 => CesroxMatterCodex::SHA3_512,
            MatterCodex::SHA2_512 => CesroxMatterCodex::SHA2_512,
            MatterCodex::Long => CesroxMatterCodex::Long,
            MatterCodex::ECDSA_256k1N => CesroxMatterCodex::ECDSA_256k1N,
            MatterCodex::ECDSA_256k1 => CesroxMatterCodex::ECDSA_256k1,
            MatterCodex::Ed448N => CesroxMatterCodex::Ed448N,
            MatterCodex::Ed448 => CesroxMatterCodex::Ed448,
            MatterCodex::Ed448_Sig => CesroxMatterCodex::Ed448_Sig,
            MatterCodex::Tern => CesroxMatterCodex::Tern,
            MatterCodex::DateTime => CesroxMatterCodex::DateTime,
            MatterCodex::X25519_Cipher_Salt => CesroxMatterCodex::X25519_Cipher_Salt,
            MatterCodex::TBD1 => CesroxMatterCodex::TBD1,
            MatterCodex::TBD2 => CesroxMatterCodex::TBD2,
            MatterCodex::StrB64_L0 => CesroxMatterCodex::StrB64_L0,
            MatterCodex::StrB64_L1 => CesroxMatterCodex::StrB64_L1,
            MatterCodex::StrB64_L2 => CesroxMatterCodex::StrB64_L2,
            MatterCodex::StrB64_Big_L0 => CesroxMatterCodex::StrB64_Big_L0,
            MatterCodex::StrB64_Big_L1 => CesroxMatterCodex::StrB64_Big_L1,
            MatterCodex::StrB64_Big_L2 => CesroxMatterCodex::StrB64_Big_L2,
            MatterCodex::Bytes_L0 => CesroxMatterCodex::Bytes_L0,
            MatterCodex::Bytes_L1 => CesroxMatterCodex::Bytes_L1,
            MatterCodex::Bytes_L2 => CesroxMatterCodex::Bytes_L2,
            MatterCodex::Bytes_Big_L0 => CesroxMatterCodex::Bytes_Big_L0,
            MatterCodex::Bytes_Big_L1 => CesroxMatterCodex::Bytes_Big_L1,
            MatterCodex::Bytes_Big_L2 => CesroxMatterCodex::Bytes_Big_L2,
        }
    }
}

pub fn matter_codex_code(codex: &MatterCodex) -> String {
    codex.code().to_string()
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum IndexerCodex {
    Ed25519,
    Ed25519_Crt,
    ECDSA_256k1,
    ECDSA_256k1_Crt,
    Ed448,
    Ed448_Crt,
    Ed25519_Big,
    Ed25519_Big_Crt,
    ECDSA_256k1_Big,
    ECDSA_256k1_Big_Crt,
    Ed448_Big,
    Ed448_Big_Crt,
    TBD0,
    TBD1,
    TBD4,
}

impl IndexerCodex {
    pub fn code(&self) -> &str {
        match self {
            IndexerCodex::Ed25519 => CesroxIndexerCodex::Ed25519,
            IndexerCodex::Ed25519_Crt => CesroxIndexerCodex::Ed25519_Crt,
            IndexerCodex::ECDSA_256k1 => CesroxIndexerCodex::ECDSA_256k1,
            IndexerCodex::ECDSA_256k1_Crt => CesroxIndexerCodex::ECDSA_256k1_Crt,
            IndexerCodex::Ed448 => CesroxIndexerCodex::Ed448,
            IndexerCodex::Ed448_Crt => CesroxIndexerCodex::Ed448_Crt,
            IndexerCodex::Ed25519_Big => CesroxIndexerCodex::Ed25519_Big,
            IndexerCodex::Ed25519_Big_Crt => CesroxIndexerCodex::Ed25519_Big_Crt,
            IndexerCodex::ECDSA_256k1_Big => CesroxIndexerCodex::ECDSA_256k1_Big,
            IndexerCodex::ECDSA_256k1_Big_Crt => CesroxIndexerCodex::ECDSA_256k1_Big_Crt,
            IndexerCodex::Ed448_Big => CesroxIndexerCodex::Ed448_Big,
            IndexerCodex::Ed448_Big_Crt => CesroxIndexerCodex::Ed448_Big_Crt,
            IndexerCodex::TBD0 => CesroxIndexerCodex::TBD0,
            IndexerCodex::TBD1 => CesroxIndexerCodex::TBD1,
            IndexerCodex::TBD4 => CesroxIndexerCodex::TBD4,

        }
    }
}

pub fn indexer_codex_code(codex: &IndexerCodex) -> String {
    codex.code().to_string()
}