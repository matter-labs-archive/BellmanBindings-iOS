initSidebarItems({"constant":[["CRH_IVK_PERSONALIZATION","BLAKE2s Personalization for CRH^ivk = BLAKE2s(ak | nk)"],["GH_FIRST_BLOCK","First 64 bytes of the BLAKE2s input during group hash. This is chosen to be some random string that we couldn't have anticipated when we designed the algorithm, for rigidity purposes. We deliberately use an ASCII hex string of 32 bytes here."],["KEY_DIVERSIFICATION_PERSONALIZATION","BLAKE2s Personalization for the group hash for key diversification"],["MATTER_EDDSA_BLAKE2S_PERSONALIZATION","BLAKE2s Personalization hash of (R_x || message) in EdDSA variant with 256 bit hash"],["NULLIFIER_POSITION_IN_TREE_GENERATOR_PERSONALIZATION","BLAKE2s Personalization for the nullifier position generator (for computing rho)"],["PEDERSEN_HASH_GENERATORS_PERSONALIZATION","BLAKE2s Personalization for Pedersen hash generators."],["PRF_NF_PERSONALIZATION","BLAKE2s Personalization for PRF^nf = BLAKE2s(nk | rho)"],["PROOF_GENERATION_KEY_BASE_GENERATOR_PERSONALIZATION","BLAKE2s Personalization for the proof generation key base point"],["SPENDING_KEY_GENERATOR_PERSONALIZATION","BLAKE2s Personalization for the spending key base point"],["VALUE_COMMITMENT_GENERATOR_PERSONALIZATION","BLAKE2s Personalization for the value commitment generator for the value"]]});