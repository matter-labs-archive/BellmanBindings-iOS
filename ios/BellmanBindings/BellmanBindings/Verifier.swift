//
//  Verifier.swift
//  BellmanBindings
//
//  Created by Anton on 28/03/2019.
//  Copyright © 2019 TheMatter. All rights reserved.
//

import Foundation

public class VerifyError: Error {
    var description: String
    init(description: String) {
        self.description = description
    }
}

public enum Engine: UInt8 {
    case BLS12_381 = 0
    case Bn256 = 1
}

public final class Verifier {
    public func verifyProof(filename: String,
                            inputs: [UInt8],
                            engine: Engine) throws -> Bool {
        let result = verify(filename,
                            inputs,
                            UInt(inputs.count),
                            EngineType(rawValue: UInt32(engine.rawValue)))
        let verificationResult = result.value
        let error = String(cString: result.error!)
        free_memory(result)
        if !error.isEmpty {
            throw VerifyError(description: error)
        }
        return verificationResult
    }
    
    public func verifyWithPrecompiledProof(filename: String,
                                           inputs: [UInt8],
                                           engine: Engine,
                                           proofVec: [UInt8]) throws -> Bool {
        let result = verify_with_precompiled_proof(filename,
                                                   inputs,
                                                   UInt(inputs.count),
                                                   EngineType(rawValue: UInt32(engine.rawValue)),
                                                   proofVec,
                                                   UInt(proofVec.count))
        let verificationResult = result.value
        let error = String(cString: result.error!)
        free_memory(result)
        if !error.isEmpty {
            throw VerifyError(description: error)
        }
        return verificationResult
    }
}
